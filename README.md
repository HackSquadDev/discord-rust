# A Discord Bot for Hacksquad

## How to Deploy?

### Requirements

- Docker
- Docker Compose

### Steps To Run

1. Copy the `docker-compose.yml` and `.env.example` files to your server
1. Rename `.env.example` -> `.env`. And fill out the values
1. `docker pull ghcr.io/hacksquaddev/discord-rust:main`
1. `docker compose up` or `docker-compose up` for earlier versions of docker compose

### How to autodeploy?

Let the watchtower container run :P
