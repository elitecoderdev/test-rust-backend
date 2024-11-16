use serde::{Serialize, Deserialize};


#[derive(Serialize)]
pub struct LoanDecision {
    pub client_id: String,
    pub total_mrr: f64,
    pub score: f64,
    pub decision: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoanApplication {
    pub client_id: String,
    pub requested_amount: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoanResponse {
    pub client_id: String,
    pub approved_amount: f64,
    pub payment_table: Vec<PaymentDetail>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentDetail {
    pub month: u32,
    pub payment_amount: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoanStatus {
    pub client_id: String,
    pub status: String,
    pub details: Option<String>,
}