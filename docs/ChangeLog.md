# ChangeLog

# Release 0.5.0

The first release on crates.io. Earlier versions, released on GitHub only, were called `sparko_graphql` and
`sparko_graphql_builder`.

## New

- `bruce-graphql`, the runtime: requests and authenticated requests, with progress reports, errors which say what the
  server said briefly, with secrets redacted from verbose output, and the types the generated code uses, e.g. dates,
  times, currencies, and pages of edges.
- `bruce-graphql-builder`, the code generator, run from a build script, which writes Rust types for each query's
  variables and response from a GraphQL schema and the queries an app makes.

## Improved

- The generated code has no clippy warnings, e.g. `is_none()` rather than `if let None`, so that an app which
  includes it can be checked with clippy.
- The generated code has no debugging comments left in it, and the builder prints less while it runs.
- `PageOf`, `ForwardPageOf` and `ReversePageOf` have `is_empty`, beside `len`.
