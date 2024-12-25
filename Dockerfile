FROM rust:1.83.0 as build
WORKDIR /src
COPY . /src/
RUN cargo build --release --target-dir=/spendings-svc-build

FROM debian:bookworm-slim
COPY --from=build /spendings-svc-build/release/spendings-svc /bin/spendings-svc
CMD ["/bin/spendings-svc"]
