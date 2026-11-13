use std::error::Error;

// Example custom build script.
fn main() -> Result<(), Box<dyn Error>> {
    println!("cargo::rerun-if-changed=build.rs");
    println!("cargo::rerun-if-changed=graphql_builder/src/lib.rs");

    // bruce_graphql_builder::builder("example")
    //     .with_schema("../bruce-graphql-builder-test/graphql/example/schema.graphql")
    // //     .with_query("foo", "bar")
    //     // .with_query("../bruce-graphql-builder-test/graphql/example/get-luke.graphql", "get-example")
    // //     .with_query("../bruce-graphql-builder-test/graphql/example/get-logged-in-user.graphql", "get-logged-in-user")
    //     .with_query("../bruce-graphql-builder-test/graphql/example/get-meters.graphql", "meters")
    //     .with_print(true)
    //     .build()?;

    // bruce_graphql_builder::builder("swapi")
    //     .with_schema("../bruce-graphql-builder-test/graphql/swapi/schema.graphql")
    //     .with_query("../bruce-graphql-builder-test/graphql/swapi/pagination.graphql", "pagination")
    // //     .with_query("../bruce-graphql-builder-test/graphql/swapi/get-luke.graphql", "luke")
    // //     .with_query("../bruce-graphql-builder-test/graphql/swapi/get-person.graphql", "person")
    // //     .with_query("../bruce-graphql-builder-test/graphql/swapi/get-luke-strikes-back.graphql", "strikes")
    // //     .with_query("../bruce-graphql-builder-test/graphql/swapi/fragment.graphql", "fragment")
    //     .build()?;

    bruce_graphql_builder::builder("octopus")
        .with_type("Date", "bruce_graphql::types::Date")
        .with_type("DateTime", "bruce_graphql::types::DateTime")
        .with_type("Decimal", "crate::octopus::decimal::Decimal")
        .with_schema("../../bruce-smart-home/graphql/octopus/octopus-schema.graphql")
        .with_query("../../bruce-smart-home/graphql/octopus/Login.graphql", "login")
        .with_query("../../bruce-smart-home/graphql/octopus/account.graphql", "account")
        .with_query("../../bruce-smart-home/graphql/octopus/meter.graphql", "meter")
        .with_query("../../bruce-smart-home/graphql/octopus/bill.graphql", "bill")
        // .with_query("../bruce-graphql-builder-test/graphql/octopus/test.graphql", "yesy")
        // .with_query("../bruce-graphql-builder-test/graphql/octopus/main.graphql", "main")
        // .with_query("../../bruce-smart-home/graphql/octopus/bill.graphql", "bill")
        // .with_query("../../bruce-smart-home/graphql/octopus/meter.graphql", "meter")
        // .with_query("../../bruce-smart-home/graphql/octopus/meter.graphql", "meter")
        // .with_print(true)
        .build()?;
    Ok(())
}
