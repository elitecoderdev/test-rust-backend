use std::collections::HashMap;
use std::io::{Cursor};
use calamine::{Reader, Xlsx, XlsxError};
use warp::{http::StatusCode, Reply, Rejection};
use warp::reply::{json, with_status};
use bytes::Bytes;
use serde::Serialize; 
use crate::models::{LoanApplication, LoanResponse, PaymentDetail, LoanDecision, LoanStatus};
use std::sync::{Arc, Mutex};

#[derive(Debug)]
struct FileError(XlsxError);

impl warp::reject::Reject for FileError {}

impl std::fmt::Display for FileError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Failed to read XLSX file: {}", self.0)
    }
}

#[derive(Debug)]
struct MutexError;

impl warp::reject::Reject for MutexError {}

impl std::fmt::Display for MutexError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Failed to acquire lock")
    }
}

#[derive(Serialize)]
struct ErrorMessage {
    message: String,
}

pub async fn post_portfolio(body: Bytes) -> Result<impl Reply, Rejection> {
    let cursor = Cursor::new(body);
    let mut workbook = Xlsx::new(cursor).map_err(|e| warp::reject::custom(FileError(e)))?;

    if let Some(Ok(range)) = workbook.worksheet_range_at(0) {
        // Validate headers
        let headers: Vec<Option<&str>> = range.rows().next().unwrap_or_default().iter()
            .map(|cell| cell.get_string()).collect();

        if headers != vec![Some("ID_CLIENTE"), Some("MONTO (USD)"), Some("AÑO"), Some("MES")] {
            return Ok(with_status(json(&ErrorMessage { message: "Invalid Excel format".to_string() }), StatusCode::BAD_REQUEST));
        }

        let mut client_months: HashMap<String, Vec<u32>> = HashMap::new();
        let mut client_data: HashMap<String, f64> = HashMap::new();

        for row in range.rows().skip(1) {
            let client_id = row.get(0).and_then(|cell| cell.get_string()).unwrap_or_default();
            let mrr = row.get(1).and_then(|cell| cell.get_float()).unwrap_or(0.0);
            let month = row.get(3).and_then(|cell| cell.get_float()).map(|m| m as u32).unwrap_or(0);

            client_months.entry(client_id.to_string()).or_insert_with(Vec::new).push(month);
            *client_data.entry(client_id.to_string()).or_insert(0.0) += mrr;
        }

        let mut decisions = Vec::new();
        for (client_id, mut months) in client_months {
            let total_mrr = *client_data.get(&client_id).unwrap_or(&0.0);
            months.sort_unstable();
            months.dedup();

            let expected_months = 12;
            let active_months = months.len() as u32;
            let churn_rate = if expected_months > active_months {
                (expected_months - active_months) as f64 / expected_months as f64
            } else {
                0.0
            };

            let score = (total_mrr / 1000.0) - (churn_rate * 10.0);
            let decision = if score > 70.0 { "Approved" } else { "Rejected" };
            decisions.push(LoanDecision { 
                client_id, 
                total_mrr, 
                score, 
                decision: decision.to_string() 
            });
        }

        Ok(with_status(json(&decisions), StatusCode::OK))
    } else {
        Ok(with_status(json(&ErrorMessage { message: "Invalid or empty worksheet".to_string() }), StatusCode::BAD_REQUEST))
    }
}

pub async fn handle_apply_loan(app: LoanApplication, loans: Arc<Mutex<HashMap<String, f64>>>) -> Result<impl Reply, Rejection> {
    if app.requested_amount <= 0.0 {
        return Ok(with_status(json(&ErrorMessage {
            message: "Requested amount must be greater than zero".to_string(),
        }), StatusCode::BAD_REQUEST));
    }

    let loans = loans.lock().map_err(|_| warp::reject::custom(MutexError))?;
    match loans.get(&app.client_id) {
        Some(&max_loan) if app.requested_amount > max_loan => Ok(with_status(json(&ErrorMessage {
            message: "Loan amount exceeds credit limit".to_string(),
        }), StatusCode::BAD_REQUEST)),
        Some(_) => {
            let payment_table: Vec<PaymentDetail> = (1..=12).map(|month| PaymentDetail {
                month,
                payment_amount: app.requested_amount / 12.0,
            }).collect();

            let response = LoanResponse {
                client_id: app.client_id,
                approved_amount: app.requested_amount,
                payment_table,
            };

            Ok(with_status(json(&response), StatusCode::OK))
        },
        None => Ok(with_status(json(&ErrorMessage {
            message: "Client ID not found".to_string(),
        }), StatusCode::NOT_FOUND))
    }
}


pub async fn get_loan_status(client_id: String, loan_statuses: Arc<Mutex<HashMap<String, LoanStatus>>>) -> Result<impl Reply, Rejection> {
    if client_id.trim().is_empty() {
        return Ok(with_status(json(&ErrorMessage {
            message: "Client ID cannot be empty".to_string(),
        }), StatusCode::BAD_REQUEST));
    }

    let statuses = loan_statuses.lock().map_err(|_| warp::reject::custom(MutexError))?;
    
    if let Some(status) = statuses.get(&client_id) {
        Ok(with_status(json(status), StatusCode::OK))
    } else {
        Ok(with_status(json(&ErrorMessage {
            message: "Loan status not found".to_string(),
        }), StatusCode::NOT_FOUND))
    }
}


