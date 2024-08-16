# Build the VoiceVox Core
FROM --platform=linux/arm64 debian as voicevox
RUN mkdir -p /opt/voicevox
WORKDIR /opt/voicevox

# curl をインストールする
RUN set -ex \
    && apt-get update \
    && apt-get install -y curl unzip \
    && rm -rf /var/lib/apt/lists/*

COPY ./cache /tmp/cache

RUN set -ex \
    && if [ -e /tmp/cache/voicevox_core-linux-arm64-cpu-0.15.4.zip ] ; then \
        echo "Using cache for voicevox_core-linux-arm64-cpu-0.15.4.zip" && \
        cp /tmp/cache/voicevox_core-linux-arm64-cpu-0.15.4.zip . ; \
    else \
        echo "No cache for voicevox_core-linux-arm64-cpu-0.15.4.zip" && \
        curl -L -O https://github.com/VOICEVOX/voicevox_core/releases/download/0.15.4/voicevox_core-linux-arm64-cpu-0.15.4.zip ; \
    fi \
    && unzip voicevox_core-linux-arm64-cpu-0.15.4.zip && rm -f voicevox_core-linux-arm64-cpu-0.15.4.zip \
    && mv /opt/voicevox/voicevox_core-linux-arm64-cpu-0.15.4 /opt/voicevox/voicevox_core

RUN set -ex \
    && if [ -e /tmp/cache/open_jtalk_dic_utf_8-1.11.tar.gz ] ; then \
        echo "Using cache for open_jtalk_dic_utf_8-1.11.tar.gz" && \
        cp /tmp/cache/open_jtalk_dic_utf_8-1.11.tar.gz . ; \
    else \
        echo "No cache for open_jtalk_dic_utf_8-1.11.tar.gz" && \
        curl -L -o open_jtalk_dic_utf_8-1.11.tar.gz \
            https://sourceforge.net/projects/open-jtalk/files/Dictionary/open_jtalk_dic-1.11/open_jtalk_dic_utf_8-1.11.tar.gz/download ; \
    fi \
    && tar xzf open_jtalk_dic_utf_8-1.11.tar.gz && rm -f open_jtalk_dic_utf_8-1.11.tar.gz

# Build the Rust application
FROM --platform=linux/arm64 rust:1.80 as builder
COPY --from=voicevox /opt/voicevox /opt/voicevox
ENV LD_LIBRARY_PATH /opt/voicevox/voicevox_core

WORKDIR /usr/src/app
COPY . .
RUN cargo build --release --target aarch64-unknown-linux-gnu


# Build the final image
FROM --platform=linux/arm64 debian:bookworm-slim
RUN apt-get update && apt-get install -y ca-certificates tzdata && rm -rf /var/lib/apt/lists/*
COPY --from=amazon/aws-lambda-provided:al2 /usr/local/bin/aws-lambda-rie /usr/local/bin/aws-lambda-rie
COPY --from=voicevox /opt/voicevox /opt/voicevox
COPY --from=builder /usr/src/app/target/aarch64-unknown-linux-gnu/release/lambda-vvcore /usr/local/bin/lambda-vvcore
ENV LD_LIBRARY_PATH /opt/voicevox/voicevox_core
ENV OPEN_JTALK_DICT_DIR /opt/voicevox/open_jtalk_dic_utf_8-1.11

COPY entry.sh /
RUN chmod +x /entry.sh
ENTRYPOINT ["/entry.sh"]
CMD ["lambda-vvcore"]