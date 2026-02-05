#Builder stage
# We use the latest Rust stable relase as base image
# FROM 1.93.0-slim AS builder
FROM lukemathwalker/cargo-chef:latest-rust-1 AS chef

# Let's switch out working dir to `app` (equivalent to `cd app`)
# The `app` folder will be created for us by Docker in case it does not exist 
WORKDIR /app

# install the required system dependencies for our linking configuration 
RUN apt update && apt install lld clang -y

FROM chef AS planner
# Copy all files from our working env to out Docker image 
COPY . .
RUN cargo chef prepare --recipe-path recipe.json


FROM chef AS builder 
COPY --from=planner /app/recipe.json recipe.json
# Build our porject dependencies, not our application!
RUN cargo chef cook --release --recipe-path recipe.json
COPY . . 

ENV SQLX_OFFLINE=true
 # Build project 

# Let's build our binary! 
# We'll use the release profile to make it fast
RUN cargo build --release --bin be 

# Runtime stage
FROM debian::bookwarm-slim AS runtime
WORKDIR /app

# Install OpenSSL - it is dynamically linked to some of our dependencies. 
# Install ca-certificates - it is needed to verify TLS Certificates when extablishing HTTPS connections
RUN apt-get update -y \
	&& apt-get install -y --no-install-recommends openssl ca-certificates \
	# clean up 
	&& apt-get autoremove -y \ 
	&& apt-get clean -y \
	&& rm -rf /var/lib/apt/lists/*

# copy the cmopiled binary from the builder env to our runtime env
COPY --from=builder /app/target/release/be be 
COPY configuration configuration 
# Load Production env
ENV APP_ENV=production


# When `docker run` is executed, launch the binary
ENTRYPOINT ["./be"]
