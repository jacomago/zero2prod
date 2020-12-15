# We use the latest Rust stable release as base image
FROM rust:latest AS planner
# Let's switch our working directory to `app` (equivalent to `cd app`)
# The `app` folder will be created for us by Docker in case it does not
# exist already.
WORKDIR app
RUN cargo install cargo-chef
# Copy all files from our working environment to our Docker image
COPY . .
RUN cargo chef prepare --recipe-path recipe.json


# Caching stage
FROM rust:latest AS cacher
WORKDIR app
RUN cargo install cargo-chef
COPY --from=planner /app/recipe.json recipe.json
# Buid project dep
RUN cargo chef cook --release --recipe-path recipe.json

# Building stage
FROM rust:latest AS builder
WORKDIR app
# copy cached dep
COPY --from=cacher /app/target target
COPY --from=cacher /usr/local/cargo /usr/local/cargo
COPY . .
# Set sqlx to offline mode 
ENV SQLX_OFFLINE true
# Let's build our binary!
# We'll use the release profile to make it faaaast
RUN cargo build --release --bin zero2prod

# runtime stage
FROM  debian:buster-slim AS runtime
WORKDIR app
# Install OpenSSL - it is dynamically linked by some of our dependencies
RUN apt-get update -y \
    && apt-get install -y --no-install-recommends openssl \
    # Clean up
    && apt-get autoremove -y \
    && apt-get clean -y \
    && rm -rf /var/lib/apt/lists/*
# copy the compiled binary form the builder
COPY --from=builder /app/target/release/zero2prod zero2prod
# We need the config for runtime
COPY configuration configuration
# Set the env
ENV APP_ENVIRONMENT production
# When `docker run` is executed, launch the binary!
ENTRYPOINT ["./zero2prod"]