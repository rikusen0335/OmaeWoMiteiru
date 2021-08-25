# 開発環境
FROM rust:1.54.0 as develop-stage
WORKDIR /app
RUN cargo install cargo-watch
COPY . .

# Downloading gcloud package
RUN curl https://dl.google.com/dl/cloudsdk/release/google-cloud-sdk.tar.gz > /tmp/google-cloud-sdk.tar.gz

# Installing the package
RUN mkdir -p /usr/local/gcloud \
    && tar -C /usr/local/gcloud -xvf /tmp/google-cloud-sdk.tar.gz \
    && /usr/local/gcloud/google-cloud-sdk/install.sh

# Adding the package path to local
ENV PATH $PATH:/usr/local/gcloud/google-cloud-sdk/bin

RUN apt update -y && apt install -y libopus-dev ffmpeg

# ビルド環境
FROM develop-stage as build-stage
RUN cargo build --release

# 本番環境
FROM rust:1.54.0-slim-stretch
COPY --from=build-stage /app/target/release/bot .
CMD ["/usr/local/bin/bot"]
