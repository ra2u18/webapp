# We use the latest rust stable release as base image
FROM rust:1.63.0

# Let's switch our working directory to `app` (equivalent to cd app)
# The `app` folder will be created for us by Docker in case it doesn't exist
WORKDIR /app

# Install the required system dependencies for our linking configuration
RUN apt update && apt install lld clang -y

# COPY all files from our working env to our Docker image
COPY . .

ENV SQLX_OFFLINE true

# Let's build our binary
RUN cargo build --release

ENV APP_ENVIRONMENT production

# When `docker run` is executed launch the binary
ENTRYPOINT ["./target/release/zero2prod"]