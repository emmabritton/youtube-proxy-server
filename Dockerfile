# Create container to build project
FROM rustlang/rust:nightly as builder

# Create new project
RUN USER=root cargo new --bin youtube-proxy-server

WORKDIR /youtube-proxy-server

# Check everything works by building 'hello world' project
RUN cargo build --release

# Remove all files from 'hello world' project
RUN rm src/*
RUN rm -r ./target

# Add everything from source dir (except what's in the .dockerignore)
ADD . ./

RUN cargo test

# Build project
RUN cargo build --release

# Create container to run project, this will discard everything not copied from builder
FROM debian:buster-slim as runtime

WORKDIR /bin

# Copy from builder and rename to 'server'
COPY --from=builder /youtube-proxy-server/target/release/youtube-proxy-server ./server

# Make certificates, time zone data is up to date
RUN apt-get update \
    && apt-get install -y ca-certificates tzdata \
    && rm -rf /var/lib/apt/lists/*

# Set timezome
ENV TZ=Etc/UTC \
    USER=appuser

RUN groupadd ${USER} \
    && useradd -g ${USER} ${USER} && \
    chown -R ${USER}:${USER} /bin

USER ${USER}

ENTRYPOINT ["./server"]