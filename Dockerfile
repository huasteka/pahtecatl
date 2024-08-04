# Build stage
FROM rust:1.80 AS builder

ENV APP_HOME=/home/pahtecatl/app
 
WORKDIR ${APP_HOME}

COPY . .

RUN cargo build --release

RUN strip ./target/release/pahtecatl
 
# Final run stage
FROM gcr.io/distroless/cc-debian12 AS runner

ENV APP_HOME=/home/pahtecatl/app

WORKDIR ${APP_HOME}

COPY --from=builder ${APP_HOME}/config ./config

COPY --from=builder ${APP_HOME}/target/release/pahtecatl .

EXPOSE 9705

ENTRYPOINT ["./pahtecatl"]
