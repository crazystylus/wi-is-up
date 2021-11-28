use anyhow::Result;
use hyper::{client::HttpConnector, Body, Client, Method, Request, StatusCode};
use std::process;
use tokio::time::{sleep, timeout, Duration};

pub async fn app(retry: u64, retry_max_time: u64) {
    let http_client = Client::new();
    for _i in 0..retry {
        if let Ok(response) = timeout(
            Duration::from_secs(5),
            check_metadata_server(http_client.clone()),
        )
        .await
        {
            if let Ok(http_status) = response {
                if http_status == StatusCode::OK {
                    process::exit(0);
                }
            }
        }
        sleep(Duration::from_secs(retry_max_time)).await;
    }
    eprintln!("Unable to connect to http://169.254.169.254/computeMetadata/v1/instance/service-accounts/default/token");
    process::exit(1);
}

async fn check_metadata_server(http_client: Client<HttpConnector>) -> Result<StatusCode> {
    let http_request: Request<Body> = Request::builder()
        .uri("http://169.254.169.254/computeMetadata/v1/instance/service-accounts/default/token")
        .method(Method::GET)
        .header("Metadata-Flavor", "Google")
        .body(Body::empty())
        .unwrap();
    Ok(http_client.request(http_request).await?.status())
}
