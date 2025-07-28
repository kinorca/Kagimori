#!/usr/bin/env bash

set -e pipefail

kubectl apply -f test-secret.yaml

kubectl exec -n kube-system -l
