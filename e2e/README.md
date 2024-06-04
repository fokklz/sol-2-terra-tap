# E2E

This directory contains end-to-end tests for the project. These tests are written in Rust, they start the HUB and the Broker. They then will imitate a client connecting and communicating with the HUB (over the broker).

## Running

You can run them by using the following command in the root of the project (1 dir up):

```bash
cargo run -p e2e
```