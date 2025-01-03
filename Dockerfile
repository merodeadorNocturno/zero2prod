# Set the build phase environment variable
ARG BUILD_PHASE=1

# Latest LTS rust
FROM rust:1.83.0 AS builder
LABEL authors="tonatiuhmartinez"

# Let's switch our working directory to 'app' (equivalent to 'cd app')
# The 'app' folder will be created for us by Docker in case it does not
# exist already
WORKDIR /app

# Install the required system dependencies for our linking configuration
RUN apt update && apt install lld clang -y

# Copy all files from our working envirnonment to our Docker image
COPY . .

# Let's build our binary!
# We'll use the release profile to make it fast
RUN cargo build --release --features skip_database

# FROM rust:1.83.0
# WORKDIR /app
# COPY --from=builder /app/target/release/zero2prod .
# CMD ["./zero2prod"]

# WHEN `docker run` is executed, launch the binary
ENTRYPOINT ["./target/release/zero2prod"]
