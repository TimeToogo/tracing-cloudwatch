# tracing-cloudwatch

tracing-cloudwatch is a custom tracing-subscriber layer that sends your application's tracing events(logs) to AWS CloudWatch Logs.  

## Usage

### With Rusoto

feature `rusoto` required

```rust
use std::time::Duration;

use rusoto_core::Region;
use tracing::info;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() {
    let cw_client = rusoto_logs::CloudWatchLogsClient::new(Region::ApNortheast1);

    tracing_subscriber::registry::Registry::default()
        .with(
            tracing_cloudwatch::layer().with_client(
                cw_client,
                tracing_cloudwatch::ExportConfig::default()
                    .with_batch_size(1)
                    .with_interval(Duration::from_secs(1))
                    .with_log_group_name("tracing-cloudwatch")
                    .with_log_stream_name("stream-1"),
            ),
        )
        .init();

    start().await;

    tokio::time::sleep(Duration::from_secs(5)).await;
}

#[tracing::instrument()]
async fn start() {
    info!("Starting...");
}
```

## Required Permissions

Currently, following AWS IAM Permissions required

* `logs:PutLogEvents`

## License

This project is licensed under the [MIT license.](./LICENSE)