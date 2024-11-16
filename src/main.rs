use warp::Filter;
use std::sync::{Arc, Mutex};
use std::collections::HashMap;

mod models;
mod handlers;
mod routes;

#[tokio::main]
async fn main() {
    // Initialize shared state with dummy credit limits
    let loans = Arc::new(Mutex::new(HashMap::new()));
    let loan_statuses = Arc::new(Mutex::new(HashMap::<String, models::LoanStatus>::new()));
    {
        let mut loans_mut = loans.lock().unwrap();
        loans_mut.insert("CLIENT001".to_string(), 50000.0);
        loans_mut.insert("CLIENT002".to_string(), 75000.0);
    }

    {
        let mut statuses_mut = loan_statuses.lock().unwrap();
        statuses_mut.insert("CLIENT001".to_string(), models::LoanStatus {
            client_id: "CLIENT001".to_string(),
            status: "Approved".to_string(),
            details: Some("Loan approved and awaiting final documentation.".to_string()),
        });
        statuses_mut.insert("CLIENT002".to_string(), models::LoanStatus {
            client_id: "CLIENT002".to_string(),
            status: "Pending Review".to_string(),
            details: Some("Loan application under review.".to_string()),
        });
    }

    // Setup CORS
    let cors = warp::cors()
        .allow_any_origin()
        .allow_headers(vec!["Content-Type", "User-Agent", "Authorization"])
        .allow_methods(vec!["GET", "POST", "DELETE", "OPTIONS"]);

    let api = routes::routes(loans, loan_statuses)
        .with(cors);

    println!("Server started at http://localhost:8000");
    warp::serve(api).run(([127, 0, 0, 1], 8000)).await;
}
