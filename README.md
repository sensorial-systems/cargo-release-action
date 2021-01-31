# cargo-release-action
GitHub Action for publishing `pushed` changes from a `pull request` with a `release label`.

## Usage

This GitHub action can be used either to publish the crate[1] or to check if it's releasable.
It depends on the execution context.

[1]: Workspaces aren't supported yet.

### On push
The push triggering this event must have come from a Pull Request.
This action bumps the semantic version, depending on the release label from the original Pull Request, and publishes the crate.
```yaml
on:
  push:
    branches:
      - main
jobs:
  release:
    runs-on: ubuntu-latest
    environment: Release
    name: Release
    steps:
      - uses: actions/checkout@v2
      - uses: actions-rs/toolchain@v1
        with:
          toolchain: stable
      - uses: nash-ws/cargo-release-action@main
        with:
          major-label: major
          minor-label: minor
          patch-label: patch
          cargo-token: ${{ secrets.CARGO_TOKEN }}
```

### On pull_request
For checking if the crate is releasable.
1. Checks if the Pull Request has a release label.
2. Checks if the crate is publishable with `cargo publish`.
```yaml
on:
  pull_request:
    branches:
      - main
jobs:
  check-release:
    runs-on: ubuntu-latest
    name: Check Release
    steps:
      - uses: actions/checkout@v2
      - uses: actions-rs/toolchain@v1
        with:
          toolchain: stable
      - uses: nash-ws/cargo-release-action@main
        with:
          major-label: major
          minor-label: minor
          patch-label: patch
```