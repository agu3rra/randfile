FROM rust:latest as prepare

COPY . /app
WORKDIR /app

# Install Curl
RUN apt-get update && apt-get upgrade -y && \
    apt-get install -y curl

# Linux requirements
RUN echo "Yay, Linux needs nothing more than just Rust! \o/" 

# Windows requirements
RUN apt-get install -y gcc-mingw-w64
RUN rustup target add x86_64-pc-windows-gnu
RUN rustup toolchain install stable-x86_64-pc-windows-gnu --force-non-host

# Building in-progress...
RUN echo "Building in-progress..."
RUN cargo build --release --target x86_64-unknown-linux-musl
RUN cargo build --release --target x86_64-pc-windows-gnu

# macOS requirements
# RUN apt-get install -y \
#     clang cmake libxml2-dev openssl-dev fts-dev bsd-compat-headers xz
# RUN rustup target add x86_64-apple-darwin
# RUN rustup toolchain install stable-x86_64-apple-darwin
# RUN cargo build --release --target x86_64-apple-darwin

# -----------------------------------------------------------------------
FROM scratch as linux_build
COPY --from=prepare /app/target/release/randfile /app/randfile
RUN chmod +x /app/randfile

FROM scratch as windows_build
COPY --from=prepare /app/target/x86_64-pc-windows-gnu/randfile.exe /app/randfile.exe
RUN chmod +x /app/randfile.exe

