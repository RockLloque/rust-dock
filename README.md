# DOCK-RUST

Dock-Rust is a Cli that utilizes [clap](https://docs.rs/clap/latest/clap/) and [bollard](https://docs.rs/bollard/latest/bollard/index.html)
to create a powerful Cli tool for running and orchestrating docker containers.

## Table of Contents
- [Installation](#installation)
- [Quick Start](#quick-start)
- [API Reference](#api-reference)
- [Examples](#examples)

## Installation 
Clone the repository from github: `git clone https://github.com/RockLloque/rust-dock.git`
Run `cargo build -q`

## Quick Start 
- Make sure docker is installed!
- On Posix systems use `command -v docker`
- Make sure that the `docker daemon` is running with `docker info`.
- Find your local docker connection socket patch with `docker context inspect `. The path with the key `Endpoint`.
- Copy the value to the `.env.example` and rename `.env.example` to `.env`


## API Reference
- `list containers` with optional flag `-a`
- `list images` with optional flag `-a`
- `start name` with name being the name of the docker container
- `stop name` with optional flag delay `-d`
- `pull name` 
