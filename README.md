# cargo-release-action
GitHub Action for publishing `pushed` changes from a `pull request` with a `release label` to https://crates.io/.

## Usage

This GitHub action can be used either to publish the crate[1] or to check if it's releasable.
It depends on the execution context.

[1]: Workspaces aren't supported yet.

`.github/workflows/release.yml`

```yaml
name: Release

on:
  push:
    branches:
      - main
  pull_request:
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
      - uses: sensorial-systems/cargo-release-action@main
        with:
          major-label: major
          minor-label: minor
          patch-label: patch
          cargo-token: ${{ secrets.CARGO_TOKEN }}
```

### Behavior

#### On push
The push triggering this event must have come from a `Pull Request`.

If the `Pull Request` has a `Release Label`, this action will bump the semantic version, depending on the type of the `Release Label` from the original Pull `Request`, and it will publish the crate.

#### On pull_request
For checking if the crate is releasable.
1. Checks if the `Pull Request` has a `Release Label`.
2. Checks if the crate is publishable with `cargo publish`.
