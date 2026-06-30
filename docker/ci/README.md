# CI Docker Harness

This directory will hold no-hardware integration services:

- OPC TCP listener
- fake Madmom sidecar
- fake orientation UDP emitter
- integration runner

The initial CI validates the layout; later increments replace this placeholder with Docker Compose services.

## Planned Example

```sh
docker compose -f docker/ci/docker-compose.yml up --abort-on-container-exit
```

Expected future services:

```text
server-under-test -> opc-listener
server-under-test -> fake-madmom
fake-orientation  -> server-under-test:5005/udp
integration-runner -> server-under-test
```

## TODO Images

TODO: Add image of Docker e2e run.

- Capture: terminal or GitHub Actions log with all Docker CI services passing.
- Expected: OPC listener, fake sidecars, and integration runner exit successfully.
- Suggested file: `docs/images/docker-ci-e2e-success.png`.
