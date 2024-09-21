# DMS Web Server

## About: Rust Translation Layer

Since I will start working on the FFI layer
that will be the backbone of this project. This
means that moving forward, ***Bun is required***,
and support for NodeJS will not be prioritized.

A PR adding support for NodeJS is highly
appreciated, but I will not be personally
working on NodeJS support because of my
tech stack listed [here](https://github.com/Kiwifuit/DMS/pull/6) in the *Info* section.

## Development

For development, both Cargo and Bun are
prerequisites. If you haven't already, please
install [Bun](https://bun.sh/) and [Rust](https://rustup.rs/)

`dms-backend` must be built first. Make
sure you are in the `dms-backend` directory
and run:

> [!IMPORTANT]
> This step only needs to be done when you
> are developing `dms-web`. For `dms-backend`
> you need to recompile the shared lib

```
cargo build --features dms
```

After this command succeeds, copy the `libdmsb`
shared library (`.so` for Linux, `.dll` for
Windows) and copy it to the `dms-web` directory.

After copying, make sure you are in the
`dms-web` directory and run the dev server:

```
bun dev
```