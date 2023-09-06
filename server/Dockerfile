FROM clux/muslrust:stable AS build
WORKDIR /release

COPY Cargo.* .
RUN mkdir src ;\
    echo "fn main() {}" >> src/main.rs ;\
    cargo build --release

COPY src src
RUN touch src/main.rs ;\
    cargo build --release --no-default-features

FROM scratch AS run
WORKDIR /home
COPY --from=build /release/server server
COPY config config
COPY migrations migrations

ENTRYPOINT ["./server"]
