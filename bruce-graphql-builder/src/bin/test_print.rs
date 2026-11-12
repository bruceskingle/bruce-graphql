use std::error::Error;

// Example custom build script.
fn main() -> Result<(), Box<dyn Error>> {
    println!("cargo::rerun-if-changed=build.rs");
    println!("cargo::rerun-if-changed=graphql_builder/src/lib.rs");

    // bruce_graphql_builder::builder("swapi")
    // .with_schema("../bruce-graphql-builder-test/graphql/swapi/schema.graphql")
    // .with_query("../bruce-graphql-builder-test/graphql/swapi/get-luke.graphql", "luke")
    // // .with_query("../bruce-graphql-builder-test/graphql/query/swapi/fragment.graphql", "fragment")
    // .with_print(true)
    // .with_generate(false)
    // .build()?;

    //     bruce_graphql_builder::builder("example")
    //     .with_schema("../bruce-graphql-builder-test/graphql/example/schema.graphql")
    //     .with_query("../bruce-graphql-builder-test/graphql/example/get-luke.graphql", "get-luke")
    //     .with_query("../bruce-graphql-builder-test/graphql/example/get-logged-in-user.graphql", "get-logged-in-user")
    //     .with_query("../bruce-graphql-builder-test/graphql/example/get-meters.graphql", "meters")
    //     .with_print(true)
    //     .with_generate(false)
    //     .build()?;

    // bruce_graphql_builder::builder("swapi")
    //     .with_schema("../bruce-graphql-builder-test/graphql/swapi/schema.graphql")
    //     .with_query("../bruce-graphql-builder-test/graphql/swapi/get-luke.graphql", "luke")
    //     .with_query("../bruce-graphql-builder-test/graphql/swapi/get-person.graphql", "person")
    //     .with_query("../bruce-graphql-builder-test/graphql/swapi/get-luke-strikes-back.graphql", "strikes")
    //     .with_query("../bruce-graphql-builder-test/graphql/swapi/fragment.graphql", "fragment")
    //     .with_print(true)
    //     .with_generate(false)
    //     .build()?;

    bruce_graphql_builder::builder("octopus")
        .with_schema("../bruce-graphql-builder-test/graphql/octopus/schema.graphql")
        .with_query("../bruce-graphql-builder-test/graphql/octopus/main.graphql", "main")
        .with_print(true)
        .with_generate(false)
        .build()?;
    Ok(())
}
