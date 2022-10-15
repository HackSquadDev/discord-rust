FROM --platform=$BUILDPLATFORM rust:1-slim-bullseye AS build

ENV CARGO_TARGET_AARCH64_UNKNOWN_LINUX_GNU_LINKER="aarch64-linux-gnu-gcc"

RUN apt update \
    && apt upgrade -y \
    && apt install -y git pkg-config libssl-dev perl make

ARG TARGETPLATFORM
RUN case "$TARGETPLATFORM" in \
    "linux/amd64") echo "x86_64-unknown-linux-gnu" > /target.txt ;; \
    "linux/arm64") echo "aarch64-unknown-linux-gnu" > /target.txt ;; \
    *) exit 1 ;; \
    esac

RUN if [ "$TARGETPLATFORM" = "linux/arm64" ]; then \
    dpkg --add-architecture arm64 \
    && apt update \
    && apt install gcc-aarch64-linux-gnu libc6-dev-arm64-cross libssl-dev:arm64 pkg-config -y; \
    fi

RUN rustup target add $(cat /target.txt)

RUN cargo new --bin bot

WORKDIR /bot

COPY Cargo.toml Cargo.lock ./

RUN cargo build --target $(cat /target.txt) --release && rm -rf .git src/ target/$(cat /target.txt)/release/deps/hacksquad*

COPY src/ src/

COPY .git/ .git/

RUN mkdir /out

RUN cargo build --target $(cat /target.txt) --release && mv target/$(cat /target.txt)/release/hacksquad-bot /out



FROM --platform=$TARGETPLATFORM debian:bullseye-slim AS runner

RUN apt update \
    && apt upgrade -y \
    && apt install --no-install-recommends ca-certificates -y \
    && rm -rf /var/lib/apt/lists/*

RUN adduser \
    --disabled-password \
    --gecos "" \
    --home "/none" \
    --shell "/sbin/nologin" \
    --no-create-home \
    bot

WORKDIR /bot

COPY --from=build /out/hacksquad-bot ./
COPY ./.git ./.git

RUN chown -R bot:bot /bot

USER bot

CMD ["./hacksquad-bot"]
