FROM rust:1.55.0

WORKDIR /app

ARG bin_name="toolbox-web"
COPY . .
ENV SQLX_OFFLINE true
# cacheはbuild kitが勝手に削除する
# herokuだとbuild kitが使えない模様
# RUN --mount=type=cache,target=/usr/local/cargo/registry \
#   --mount=type=cache,target=/app/target \
#   cargo build --release && \
#   cp /app/target/release/${bin_name} /app/${bin_name}

RUN cargo build --release && \
  cp /app/target/release/${bin_name} /app/${bin_name}
ENTRYPOINT ["/app/${bin_name}"]
