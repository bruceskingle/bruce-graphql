# bruce-graphql

Bruce's GraphQL CodeGen: typed Rust from a GraphQL schema and queries. A build script gives the builder a schema and
the queries an app makes, and it writes Rust types for each query's variables and response, which the app calls
through the runtime library.

| Crate | What it is |
|---|---|
| ```bruce-graphql``` | The runtime: requests, authenticated requests, errors, and the types the generated code uses, e.g. pages and edges. |
| ```bruce-graphql-builder``` | The code generator, used from a build script. |
| ```bruce-graphql-builder-test``` | Tests and examples of the builder, not published. |

Its first user is [Bruce's Smart Home](https://github.com/bruceskingle/bruce-smart-home), a desktop app for seeing a household's
energy, which uses it for Octopus Energy's API. It was called ```sparko_graphql``` before 0.5.0.

License
=======

Licensed under either of

* Apache License, Version 2.0,
  (./LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0)
* MIT license (./LICENSE-MIT or http://opensource.org/licenses/MIT)
  at your option.

Contribution
------------

Unless you explicitly state otherwise, any contribution intentionally
submitted for inclusion in the work by you, as defined in the Apache-2.0
license, shall be dual licensed as above, without any additional terms or
conditions.