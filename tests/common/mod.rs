use jikan_rs::JikanClient;
use std::time::Duration;
use tokio::time::sleep;

// Helper function to handle rate limiting between tests
pub async fn wait_between_tests() {
    sleep(Duration::from_secs(2)).await;
}
