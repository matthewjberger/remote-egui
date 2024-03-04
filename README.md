# Remote Egui App

This repo is an example of an egui app that can be run natively or be compiled to a wasm frontend with a websocket client included that communicates with a websocket server running on the host.

## Prerequisites

M1 Macs:

```
just init-wasm-m1
```

Ubuntu / Debian Linux:

`sudo apt-get install libxcb-render0-dev libxcb-shape0-dev libxcb-xfixes0-dev libxkbcommon-dev libssl-dev`

Fedora:

`dnf install clang clang-devel clang-tools-extra libxkbcommon-devel pkg-config openssl-devel libxcb-devel gtk3-devel atk fontconfig-devel`

## Quickstart

- Install [just](https://github.com/casey/just).

Show command list:

```
just
```

Run the frontend:

```bash
# native
just build
just run

# wasm
just init-wasm
just build-bundle
just run-web
```

Run the backend:

```bash
just run-server
```

> The backend can run on another computer
> belonging to the same network the frontend is running on
