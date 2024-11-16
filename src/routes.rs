use warp::Filter;
use super::handlers;
use std::sync::{Arc, Mutex};
use std::collections::HashMap;
use crate::models;

pub fn routes(loans: Arc<Mutex<HashMap<String, f64>>>, loan_statuses: Arc<Mutex<HashMap<String, models::LoanStatus>>>) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    let portfolio = post_portfolio();
    let apply_loan = apply_loan(loans);
    let loan_status = get_loan_status(loan_statuses);

    portfolio.or(apply_loan).or(loan_status)
}


fn post_portfolio() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path("portfolio")
        .and(warp::post())
        .and(warp::body::bytes())
        .and_then(handlers::post_portfolio)
}

fn apply_loan(loans: Arc<Mutex<HashMap<String, f64>>>) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path("apply-loan")
        .and(warp::post())
        .and(warp::body::json())
        .and(with_loans(loans))
        .and_then(handlers::handle_apply_loan)
}

// Helper function to pass loans state
fn with_loans(loans: Arc<Mutex<HashMap<String, f64>>>) -> impl Filter<Extract = (Arc<Mutex<HashMap<String, f64>>>,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || loans.clone())
}

fn get_loan_status(loan_statuses: Arc<Mutex<HashMap<String, models::LoanStatus>>>) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path("loan-status")
        .and(warp::get())
        .and(warp::path::param::<String>())
        .and(with_loan_statuses(loan_statuses))
        .and_then(handlers::get_loan_status)
}

fn with_loan_statuses(loan_statuses: Arc<Mutex<HashMap<String, models::LoanStatus>>>) -> impl Filter<Extract = (Arc<Mutex<HashMap<String, models::LoanStatus>>>,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || loan_statuses.clone())
}

