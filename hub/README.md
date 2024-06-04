# HUB

Core of the project. The HUB is the central point of communication between clients. It is responsible for managing the connections and the messages that are sent between them. It keeps track of the state of the clients and allows states to be requested.

**ALPHA VERSION. PRODUCTION USE NOT RECOMMENDED.**

## Running

You can run the HUB by using the following command in the root of the project (1 dir up):

```bash
cargo run -p hub
```

## Tests

The HUB includes unit tests. You can run them by using the following command in the root of the project (1 dir up):

```bash
cargo test -p hub
```

for [end-to-end tests](../e2e/README.md) see the e2e directory.