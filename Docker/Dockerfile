FROM alpine:3.15

# Initial Setup
RUN apk update && apk add --no-cache \
  sudo bash curl jq nano rust

COPY Docker/volumes/.bashrc /root/.bashrc

WORKDIR /src