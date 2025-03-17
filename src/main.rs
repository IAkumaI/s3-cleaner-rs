use std::{env, sync::Arc};

use aws_sdk_s3::{Credentials, Endpoint};
use chrono::{Local, TimeZone, Utc};
use clap::Parser;
use tokio::sync::Semaphore;
use tokio_stream::StreamExt;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Prefix for search objects
    #[arg(short, long, default_value = "")]
    prefix: String,

    /// Suffix for search objects
    #[arg(short, long, default_value = "")]
    suffix: String,

    /// Objects older than the specified will be deleted (1d2h30m)
    #[arg(short, long)]
    older_than: String,

    /// Set true for real delete objects
    #[arg(long, default_value_t = false)]
    delete: bool,

    /// Page size while retrieving objects
    #[arg(long, default_value_t = 100)]
    page_size: i32,

    /// Max concurrent requests to S3
    #[arg(long, default_value_t = 10)]
    concurrent_requests: usize,
}

#[tokio::main]
async fn main() {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();

    let cli = Cli::parse();
    let older_than = duration_str::parse(&cli.older_than).expect("Duration is invalid");
    let older_than = chrono::Duration::seconds(older_than.as_secs() as i64);
    let start = Utc::now().checked_sub_signed(older_than).unwrap();

    log::info!("Searching for objects since {}", start);

    let c_prefix = cli.prefix.clone();
    let c_suffix = cli.suffix.clone();
    let c_bucket = env::var("S3_BUCKET").expect("S3_BUCKET is required");
    let c_region = env::var("S3_REGION").expect("S3_REGION is required");
    let c_endpoint = env::var("S3_ENDPOINT").expect("S3_ENDPOINT is required");
    let c_access_key = env::var("S3_ACCESS_KEY_ID").expect("S3_ACCESS_KEY_ID is required");
    let c_secret = env::var("S3_ACCESS_KEY_SECRET").expect("S3_ACCESS_KEY_SECRET is required");

    let region = aws_sdk_s3::Region::new(c_region);

    let shared_config = aws_config::from_env()
        .region(region)
        .endpoint_resolver(Endpoint::immutable(c_endpoint.parse().expect("valid URI")))
        .credentials_provider(Credentials::new(c_access_key, c_secret, None, None, "s3"))
        .load()
        .await;

    let s3_client = Arc::new(aws_sdk_s3::Client::new(&shared_config));

    let mut objects = s3_client
        .list_objects_v2()
        .bucket(c_bucket.clone())
        .prefix(c_prefix.clone())
        .into_paginator()
        .page_size(cli.page_size)
        .send();

    let mut tasks_set = tokio::task::JoinSet::new();
    let cr_sem = Arc::new(Semaphore::new(cli.concurrent_requests));
    let start_timestamp = start.timestamp();
    let mut counter = 0;
    while let Some(v) = objects.next().await {
        let v = match v {
            Ok(v) => v,
            Err(e) => {
                log::error!("Can not get ListObjectsV2Output: {e}");
                break;
            }
        };

        let objects = v
            .contents()
            .unwrap_or_default()
            .iter()
            .filter(|obj| !obj.key().unwrap().is_empty()) // Check that the key is not empty
            .filter(|obj| !obj.key().unwrap().eq(&c_prefix)) // Prevent deletion of folder matching the prefix
            .filter(|obj| !obj.key().unwrap().eq(&c_suffix)) // Prevent deletion of folder matching the suffix
            .filter(|obj| obj.key().unwrap().starts_with(&c_prefix)) // Check that object starts with the prefix
            .filter(|obj| obj.last_modified().unwrap().secs().le(&start_timestamp)) // Check that object is older than specified time
            .filter(|obj| {
                // Check that object ends with the suffix
                c_suffix.is_empty() || {
                    let key = obj.key().unwrap();
                    key.len() >= c_suffix.len() && key.ends_with(&c_suffix)
                }
            });

        for obj in objects {
            let ct = Local
                .timestamp_opt(obj.last_modified().unwrap().secs(), 0)
                .unwrap();
            let obj_key = obj.key().unwrap().to_owned();

            if cli.delete {
                let s3_client = s3_client.clone();
                let c_bucket = c_bucket.clone();
                let permit = cr_sem
                    .clone()
                    .acquire_owned()
                    .await
                    .expect("Can not get lock");

                tasks_set.spawn(async move {
                    let _permit = permit;

                    let res = s3_client
                        .delete_object()
                        .bucket(c_bucket)
                        .key(&obj_key)
                        .send()
                        .await;

                    if res.is_ok() {
                        log::info!("Deleted {obj_key} - {ct}");
                    }

                    res
                });
            } else {
                log::info!("Found {obj_key} - {ct}");
            }

            counter += 1;
        }
    }

    // Wait for deletion to complete
    while let Some(res) = tasks_set.join_next().await {
        let _out = res.expect("Can not delete object");
    }

    if cli.delete {
        log::info!("Deleted objects: {counter}");
    } else {
        log::info!("Found objects: {counter}. To delete them, pass --delete");
    }
}
