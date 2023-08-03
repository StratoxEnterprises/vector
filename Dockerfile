FROM rust:1.67 as builder

RUN PROTOC_ZIP=protoc-3.14.0-linux-x86_64.zip && \
   curl -OL https://github.com/protocolbuffers/protobuf/releases/download/v3.14.0/$PROTOC_ZIP && \
   unzip -o $PROTOC_ZIP -d /usr/local bin/protoc && \
   unzip -o $PROTOC_ZIP -d /usr/local 'include/*' && \
   rm -f $PROTOC_ZIP && \ 
   apt-get update && \
   apt-get install -y libsasl2-dev && \ 
   cargo install cargo-deb

WORKDIR /vector
COPY . /vector

# run cargo build
RUN TARGET_CC=x86_64-unknown-linux-gnu-gcc  CFLAGS="-g0 -O3" cargo build --release --target x86_64-unknown-linux-gnu 

# make debian package
RUN echo "cn" > ./target/debian-license.txt && \
    echo "cn" > ./target/debian-extended-description.txt && \
    cargo deb --target "x86_64-unknown-linux-gnu" --deb-version "1.0" --variant "x86_64-unknown-linux-gnu" --no-build --no-strip
    
# unpack debian package
RUN dpkg -i target/x86_64-unknown-linux-gnu/debian/vector_1.0_amd64.deb && \
      mkdir -p /var/lib/vector

FROM docker.io/debian:bullseye-slim

RUN apt-get update && apt-get install -y --no-install-recommends ca-certificates tzdata systemd && rm -rf /var/lib/apt/lists/*

COPY --from=builder /usr/bin/vector /usr/bin/vector
COPY --from=builder /usr/share/doc/vector /usr/share/doc/vector
COPY --from=builder /etc/vector /etc/vector
COPY --from=builder /var/lib/vector /var/lib/vector

# Smoke test
RUN ["vector", "--version"]

ENTRYPOINT ["/usr/bin/vector"]