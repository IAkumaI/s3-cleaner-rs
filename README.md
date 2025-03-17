# S3 Cleaner

A simple utility for cleaning old files from S3-compatible storage.

## Description

S3 Cleaner is a command-line tool that helps automate the process of removing outdated files from S3-compatible storages (e.g., Amazon S3, MinIO, etc.).

## Features

-   Delete files older than specified age
-   Support for S3-compatible storages
-   Connection parameters configuration through environment variables (env)
-   Ability to work with specific prefixes/folders
-   Safe deletion with preview mode

## Installation

```bash
git clone https://github.com/IAkumaI/s3-cleaner-rs
cd s3-cleaner-rs
cargo build --release
```

## Configuration

The following environment variables are required for the utility to work:

-   `S3_ACCESS_KEY_ID` - S3 access key identifier
-   `S3_ACCESS_KEY_SECRET` - S3 secret access key
-   `S3_BUCKET` - S3 bucket name
-   `S3_ENDPOINT` - S3 endpoint URL (e.g., https://s3.amazonaws.com)
-   `S3_REGION` - S3 region (e.g., us-east-1)

## Usage

```bash
s3-cleaner --days 30 --prefix backup/
```

### Launch Parameters

-   `--delete` - actual file deletion (default false)
-   `--prefix` - work only with files starting with the specified prefix (default "")
-   `--suffix` - work only with files ending with the specified suffix (default "")
-   `--older-than` - delete files older than specified time (format 1d2h30m)
-   `--page-size` - page size when retrieving file list (default 100)
-   `--concurrent-requests` - number of simultaneous requests to S3 (default 10)

## Security

Before deletion, it is recommended to:

1. Make a backup of important data
2. Run the utility in preview mode (without the `--delete` flag)
