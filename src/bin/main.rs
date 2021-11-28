use anyhow::{Context, Result};
use clap::{App, Arg};
use std::str::FromStr;
use workload_identity_is_up::app;

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<()> {
    let matches = App::new("workload_identity_is_up")
        .version("0.1.0")
        .author("Kartik Sharma <kartik.sharma522@gmail.com>")
        .about("Check if workload identity is up")
        .arg(
            Arg::with_name("retry")
                .long("retry")
                .takes_value(true)
                .help("Number of retries to attempt"),
        )
        .arg(
            Arg::with_name("retry_max_time")
                .long("retry-max-time")
                .takes_value(true)
                .help("max time per connection attempt"),
        )
        .get_matches();
    let retry: u64 = FromStr::from_str(matches.value_of("retry").unwrap_or("30"))
        .with_context(|| format!("Invalid value for retry, expected value >= 1"))?;
    let retry_max_time: u64 = FromStr::from_str(matches.value_of("retry_max_time").unwrap_or("5"))
        .with_context(|| format!("Invalid value for retry_max_time, expected value >= 1"))?;
    app(retry, retry_max_time).await;
    Ok(())
}
