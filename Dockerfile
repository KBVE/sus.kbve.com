################
# Builder
################
FROM rust:1.61.0-slim as builder

# Workdir for sus-kbve
WORKDIR /usr/src

# Set user and start new sus-kbve
RUN USER=root cargo new sus-kbve

# We want dependencies cached via Cargo.lock.
COPY Cargo.toml Cargo.lock /usr/src/sus-kbve/

WORKDIR /usr/src/sus-kbve

## Install target platform (Cross-Compilation) --> Needed for Alpine!
RUN rustup target add x86_64-unknown-linux-musl

# This is a dummy build to get the dependencies cached.
RUN cargo build --target x86_64-unknown-linux-musl --release

# Now copy in the rest of the sources
COPY src /usr/src/sus-kbve/src/

## Touch main.rs to prevent cached release build
RUN touch /usr/src/sus-kbve/src/main.rs

# This is the actual application build.
RUN cargo build --target x86_64-unknown-linux-musl --release

################
# Runtime
################

FROM alpine:3.16.0 AS runtime 

# Copy application binary from builder image
COPY --from=builder /usr/src/sus-kbve/target/x86_64-unknown-linux-musl/release/sus-kbve /usr/local/bin

EXPOSE 3030

# Run the application
CMD ["/usr/local/bin/sus-kbve"]
