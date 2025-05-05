FROM rust:buster AS build
COPY ./ /app
WORKDIR /app

RUN cargo build --release

FROM debian:bullseye-slim AS runner

RUN cp /usr/share/zoneinfo/Asia/Tokyo /etc/localtime && \
    echo "Asia/Tokyo" > /etc/timezone

WORKDIR /app

COPY --from=build /app/target/release/role_master .
COPY ./config.json .
COPY ./database.db .
ENV DATABASE_URL="sqlite:./database.db" RUST_LOG="info"

ENTRYPOINT ["./role_master"]