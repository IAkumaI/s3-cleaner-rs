FROM rust:1.78-alpine3.19 as build

LABEL maintainer="IAkumaI"
LABEL description="S3 Cleaner - A utility for cleaning old files from S3-compatible storage"

RUN apk add --no-cache musl-dev pkgconfig openssl-dev

WORKDIR /app

# Copy dependency files first for better cache utilization
COPY Cargo.toml Cargo.lock ./

# Create a dummy main.rs to build dependencies
RUN mkdir src && \
    echo "fn main() {}" > src/main.rs && \
    cargo build --release && \
    rm -rf src

# Now copy the real source code
COPY ./src ./src

# Touch all files in the src directory to update timestamps
RUN find /app/src -type f -exec touch {} +

# Build the application
RUN cargo build --release

FROM alpine:3.19

RUN apk add --no-cache libgcc && \
    addgroup -S app && \
    adduser -S app -G app

COPY --from=build /app/target/release/s3-cleaner-rs /usr/local/bin/

USER app

ENTRYPOINT ["/usr/local/bin/s3-cleaner-rs"]