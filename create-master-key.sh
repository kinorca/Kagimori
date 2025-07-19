#!/usr/bin/env bash

kid=$(uuid -v 4)

type=${1:-'ChaCha20Poly1305'}

type=${type,,}

case $type in
  'chacha20poly1305')
    algorithm='ChaCha20Poly1305'
    key=$(openssl rand -base64 32)
    ;;
  'aesgcmsiv')
    algorithm='AesGcmSiv'
    key=$(openssl rand -base64 32)
    ;;
  *)
    echo 'Invalid name of algorithm.'
    echo "usage: $0 [ALGORITHM]"
    echo '  algorithms (case insensitive):'
    echo '    ChaCha20Poly1305: Use ChaCha20-Poly1305 algorithm'
    echo '    AesGcmSiv: Use AES 256 GCM-SIV algorithm'
    exit 1
    ;;
esac

echo "default: $kid"
echo "keys:"
echo "  - algorithm: $algorithm"
echo "    id: $kid"
echo "    key: '$key'"
