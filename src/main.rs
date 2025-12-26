use axum::{Router, extract::Json, routing::post};
use serde::{Deserialize, Serialize};
use tracing::info;

#[derive(Deserialize)]
struct EvenCheckerRequest {
    number: i64,
}

#[derive(Serialize)]
struct EvenCheckerResponse {
    result: bool,
}

async fn even_checker(Json(payload): Json<EvenCheckerRequest>) -> Json<EvenCheckerResponse> {
    let result = is_even(payload.number);
    Json(EvenCheckerResponse { result })
}

fn is_even(number: i64) -> bool {
    number % 2 == 0
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let app = Router::new().route("/even_checker", post(even_checker));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8000").await.unwrap();

    info!("Server running on http://0.0.0.0:8000");

    axum::serve(listener, app).await.unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_even_with_even_number() {
        assert!(is_even(2));
        assert!(is_even(0));
        assert!(is_even(42));
        assert!(is_even(-4));
    }

    #[test]
    fn test_is_even_with_odd_number() {
        assert!(!is_even(1));
        assert!(!is_even(3));
        assert!(!is_even(43));
        assert!(!is_even(-3));
    }

    #[test]
    fn test_is_even_edge_cases() {
        assert!(is_even(i64::MIN));
        assert!(!is_even(i64::MAX));
    }
}
