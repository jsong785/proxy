load('ext://restart_process', 'docker_build_with_restart')
local_resource('cargo-artifacts', 'mkdir -p cargo_artifacts_dir')
local_resource(
    'proxy-rust-compile',
    'docker run --rm -e CARGO_HOME="/mnt/cargo-artifacts" -v $(pwd):/mnt/stuff -v $(pwd)/cargo-artifacts/:/mnt/cargo-artifacts arm64v8/rust:1.80-bullseye /bin/bash -c "cd mnt/stuff && cargo build -r --target-dir release_output"',
    deps=['Cargo.toml', 'Cargo.lock', 'src/'])

docker_build_with_restart(
  'proxy-image',
  '.',
  entrypoint='/proxy',
  dockerfile='Dockerfile',
  only=[
    './release_output/release/',
  ],
  live_update=[
    sync('./release_output/release/', '/'),
  ],
)
k8s_yaml('proxy.yaml')
k8s_resource('proxy', port_forwards=[8080],
             resource_deps=['proxy-rust-compile'])

local_resource('mockserver',
               cmd='',
               serve_cmd='docker run --platform linux/amd64 --rm -p 123:123 --env MOCKSERVER_LOG_LEVEL=TRACE --env MOCKSERVER_SERVER_PORT=123 mockserver/mockserver')