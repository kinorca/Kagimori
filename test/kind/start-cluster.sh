#!/usr/bin/env bash

config_home="/tmp/kagimori/encryption"

mkdir -p $config_home

cp ./encryption/pre.yaml "$config_home/config.yaml"

kind create cluster --config kind-config.yaml

kind load docker-image ghcr.io/kinorca/kagimori:latest --name kmsv2-test-cluster

kubectl apply -f daemonset.yaml
for i in $(seq 10); do
  sleep 5
  if kubectl wait --for=condition=Ready pod -l app.kubernetes.io/name=kagimori --timeout=10s; then
    break
  fi
done

cp ./encryption/post.yaml "$config_home/config.yaml"
