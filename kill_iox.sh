#!/usr/bin/env bash

set -eu -o pipefail

trap '' INT

export RUST_BACKTRACE=1

echo "===== build IOx ====="
cargo build --no-default-features

echo "===== build data generator ====="
pushd iox_data_generator
cargo build --release
popd

echo "===== clean up kafka ====="
docker-compose \
    -f docker/ci-kafka-docker-compose.yml \
    down \
    -v

echo "===== start kafka ====="
docker-compose \
    -f docker/ci-kafka-docker-compose.yml \
    up \
    kafka &
readonly PID_DOCKER=$!

echo "===== start router ====="
./target/debug/influxdb_iox \
    run \
    --server-id 1 \
    -v &
readonly PID_ROUTER=$!

echo "===== start query node ====="
readonly STORE_LOCATION="$(mktemp -d)"
./target/debug/influxdb_iox \
    run \
    --server-id 2 \
    --api-bind 127.0.0.1:8084 \
    --grpc-bind 127.0.0.1:8086 \
    --data-dir "$STORE_LOCATION" \
    --object-store file \
    -v &
readonly PID_QUERY=$!

echo "===== wait for Kafka and IOx to boot up ====="
sleep 8

echo "===== create database ====="
pushd iox_data_generator
cargo run --release --bin create_database -- --writer 127.0.0.1:8082 --reader 127.0.0.1:8086 mlb_pirates
popd

echo "===== start data generator ====="
pushd iox_data_generator
cargo run --release -- --spec schemas/storage_usage_bucket_cardinality.toml --continue --host 127.0.0.1:8080 --token arbitrary --org mlb --bucket pirates &
readonly PID_DATA_GENERATOR=$!
popd

read -p "Waiting for keypress"

echo "===== stop data generator ====="
kill -SIGTERM $PID_DATA_GENERATOR
set +e
wait $PID_DATA_GENERATOR
set -e

echo "===== stop query node ====="
kill -SIGTERM $PID_QUERY
set +e
wait $PID_QUERY
set -e
rm -rf "$STORE_LOCATION"

echo "===== stop router ====="
kill -SIGTERM $PID_ROUTER
set +e
wait $PID_ROUTER
set -e

echo "===== stop kafka ====="
kill -SIGTERM $PID_DOCKER
set +e
wait $PID_DOCKER
set -e

echo "===== clean up kafka ====="
docker-compose -f docker/ci-kafka-docker-compose.yml down -v

echo "done"
