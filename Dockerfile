FROM arm64v8/rust:1.80-bullseye as prod
COPY ./release_output/release/ ./
ENTRYPOINT ./proxy