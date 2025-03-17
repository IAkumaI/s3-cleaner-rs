# S3 Cleaner

A simple utility for cleaning old files from S3-compatible storage.

## Description

S3 Cleaner is a command-line tool that helps automate the process of removing outdated files from S3-compatible storages (e.g., Amazon S3, MinIO, etc.).

## Features

-   Delete files older than specified age
-   Support for S3-compatible storages
-   Connection parameters configuration through environment variables (env)
-   Support for multiple prefixes and suffixes (comma-separated)
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
# Delete files older than 30 days in a single prefix
s3-cleaner --older-than=30d --prefix=backup/

# Delete files older than 7 days in multiple prefixes
s3-cleaner --older-than=7d --prefix=upload/,download/,temp/

# Delete specific file types older than 1 day
s3-cleaner --older-than=1d --suffix=.tmp,.bak,.temp

# Delete specific files from specific locations
s3-cleaner --older-than=12h --prefix=logs/,temp/ --suffix=.log,.tmp --delete
```

### Launch Parameters

-   `--delete` - actual file deletion (default false)
-   `--prefix=<prefixes>` - work with files starting with specified prefixes, comma-separated (e.g., "upload/,backup/")
-   `--suffix=<suffixes>` - work with files ending with specified suffixes, comma-separated (e.g., ".tmp,.bak")
-   `--older-than=<duration>` - delete files older than specified time (format 1d2h30m)
-   `--page-size=<size>` - page size when retrieving file list (default 100)
-   `--concurrent-requests=<num>` - number of simultaneous requests to S3 (default 10)

## Security

Before deletion, it is recommended to:

1. Make a backup of important data
2. Run the utility in preview mode (without the `--delete` flag)
