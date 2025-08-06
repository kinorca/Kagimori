# Copyright 2025 SiLeader.
#
# This file is part of Kagimori.
#
# Kagimori is free software: you can redistribute it and/or modify it under the terms of
# the GNU General Public License as published by the Free Software Foundation,
# either version 3 of the License, or (at your option) any later version.
#
# Kagimori is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY;
# without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.
# See the GNU General Public License for more details.
#
# You should have received a copy of the GNU General Public License along with Kagimori.
# If not, see <https://www.gnu.org/licenses/>.

# syntax=docker/dockerfile:1

ARG version

FROM rust:alpine AS builder

LABEL org.opencontainers.image.source="https://github.com/kinorca/Kagimori.git"

WORKDIR /work

COPY . .

RUN --mount=type=cache,target=/work/target \
    --mount=type=cache,target=/work/.cargo \
    apk add musl-dev protobuf && \
    cargo build --release && \
    cp /work/target/release/kagimori /kagimori

FROM scratch

LABEL org.opencontainers.image.source="https://github.com/kinorca/Kagimori.git" \
      org.opencontainers.image.url="https://github.com/kinorca/Kagimori"

COPY --from=builder /kagimori /usr/local/bin/kagimori

CMD ["/usr/local/bin/kagimori"]
