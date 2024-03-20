# Use the official Rust image as a base
FROM rust:latest

# Install `cargo-watch`
RUN cargo install cargo-watch sqlx-cli

# Create a new directory for your application
WORKDIR /usr/src/rust_app

# Copy your application's source code into the Docker image
COPY . .

# Expose the port your application listens on
EXPOSE 8080

# Use `cargo watch` to rebuild and rerun your application whenever source files change
CMD ["cargo", "watch", "-x", "run"]
