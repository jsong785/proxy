FROM rust:1.80-bullseye as prod
COPY ./build_output/debug/ ./
ENTRYPOINT ./proxy