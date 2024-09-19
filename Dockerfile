# Use the official Rust image as the base
FROM rust:latest

# Set the working directory
WORKDIR /app

# Install necessary system dependencies
RUN apt-get update && apt-get install -y \
    protobuf-compiler \
    && rm -rf /var/lib/apt/lists/*

# Copy the Cargo.toml and Cargo.lock files
COPY Cargo.toml Cargo.lock ./

# Copy the actual source code
COPY . .

# Build the project
RUN cargo build --release

# Define the command to run your project (or customize as needed)
CMD ["/app/target/release/host-analyzer"]
