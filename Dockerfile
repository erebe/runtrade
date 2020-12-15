# Rust backend
FROM rust:1.48 AS builder_backend

LABEL org.opencontainers.image.source https://github.com/erebe/couber

WORKDIR /backend
COPY backend/Cargo.lock .
COPY backend/Cargo.toml .
RUN mkdir .cargo
RUN cargo vendor > .cargo/config

COPY backend .
RUN cargo build --release 


# VueJS Frontend
FROM rust:1.48 AS builder_frontend

RUN apt-get update && \
	curl -sL https://deb.nodesource.com/setup_10.x | bash - && \
	apt install -y nodejs build-essential 

COPY webapp webapp
WORKDIR /webapp

RUN npm install && \
    npm run build

# Runner
FROM debian:stable-slim

RUN useradd -ms /bin/bash app && \
  apt-get update && \
	apt install -y --no-install-recommends ca-certificates postgresql-client && \
	apt-get clean && \
	rm -rf /var/lib/apt/lists 

WORKDIR /home/app

COPY --from=builder_backend backend/target/release/runtrade-backend runtrade-backend
COPY --from=builder_frontend webapp/dist dist

ENV DATABASE=localhost
ENV JWK_N=xxx
ENV JWK_E=xxx
ENV PORT=8080
ENV RUST_LOG=info
ENV WEBAPP_DIR_PATH=/home/app/dist
EXPOSE 8080

CMD chown -R app:app . && \
    runuser -u app ./runtrade-backend


