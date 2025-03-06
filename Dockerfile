FROM alpine:latest

COPY target/x86_64-unknown-linux-musl/release/statistical-testdata-exporter /usr/bin/

EXPOSE 7878

ENTRYPOINT ["/usr/bin/statistical-testdata-exporter"]
