use std::io::Write;
use std::{env, error::Error, fs::File, path::Path, process};

// fn get_current_working_dir() -> String {
//     let res = std::env::current_dir();
//     match res {
//         Ok(path) => path.into_os_string().into_string().unwrap(),
//         Err(_) => "FAILED".to_string()
//     }
// }
fn main() {
    if build().is_err() {
        process::exit(1);
        // panic!("Build failed {}", error);
    }
}

// Example custom build script.
fn build() -> Result<(), Box<dyn Error>> {
    // panic!("Panic cwd={}", get_current_working_dir());

    if let Some(dir) = env::var_os("OUT_DIR") {
        let dest_path = Path::new(&dir).join("crate_info.rs");
        // let dest_path_string = format!("{}", dest_path.to_string_lossy());

        let mut file = File::create(dest_path)?;

        writeln!(
            file,
            r#"
#[allow(dead_code)]
mod create_info {{
    pub const PACKAGE_NAME: &str = "{}";
    pub const PACKAGE_VERSION: &str = "{}";
    pub const USER_AGENT: &str = "{}-{}";
}}
"#,
            env!("CARGO_PKG_NAME"),
            env!("CARGO_PKG_VERSION"),
            env!("CARGO_PKG_NAME"),
            env!("CARGO_PKG_VERSION")
        )?;
    }

    println!("cargo::rerun-if-changed=build.rs");
    println!("cargo::rerun-if-changed=graphql_builder/src/lib.rs");

    bruce_graphql_builder::builder("example")
        .with_schema("graphql/example/schema.graphql")
        .with_query("graphql/example/get-luke.graphql", "get-example")
        .with_query("graphql/example/get-logged-in-user.graphql", "get-logged-in-user")
        .with_query(
            "../bruce-graphql-builder-test/graphql/example/get-meters.graphql",
            "meters",
        )
        .build()?;

    bruce_graphql_builder::builder("swapi")
        .with_schema("graphql/swapi/schema.graphql")
        .with_query("graphql/swapi/get-luke.graphql", "luke")
        .with_query("graphql/swapi/get-person.graphql", "person")
        .with_query("graphql/swapi/get-luke-strikes-back.graphql", "strikes")
        .with_query("graphql/swapi/pagination.graphql", "pagination")
        .with_query("graphql/swapi/fragment.graphql", "fragment")
        .build()?;

    bruce_graphql_builder::builder("rootpage")
        .with_schema("graphql/rootpage/schema.graphql")
        .with_query("graphql/rootpage/root-edge.graphql", "root-edge")
        .with_query("graphql/rootpage/root-page.graphql", "root-page")
        .with_query("graphql/rootpage/root-page-info.graphql", "root-page-info")
        .build()?;

    bruce_graphql_builder::builder("octopus")
        .with_type("Date", "bruce_graphql::types::Date")
        .with_type("DateTime", "bruce_graphql::types::DateTime")
        .with_schema("graphql/octopus/schema.graphql")
        .with_query("graphql/octopus/main.graphql", "main")
        .with_query("../../bruce-smart-home/graphql/octopus/bill.graphql", "bill")
        .with_query("../../bruce-smart-home/graphql/octopus/meter.graphql", "meter")
        .build()?;

    // bruce_graphql_builder::builder("octopus")
    //     .with_type("Date", "bruce_graphql::types::Date")
    //     .with_type("DateTime", "bruce_graphql::types::DateTime")
    //     .with_type("Decimal", "crate::octopus::decimal::Decimal")
    //     .with_schema("../../bruce-smart-home/graphql/octopus/octopus-schema.graphql")
    //     // .with_query("../bruce-graphql-builder-test/graphql/octopus/test.graphql", "yesy")
    //     // .with_query("../bruce-graphql-builder-test/graphql/octopus/main.graphql", "main")
    //     .with_query("../../bruce-smart-home/graphql/octopus/bill.graphql", "bill")
    //     // .with_query("../../bruce-smart-home/graphql/octopus/meters.graphql", "meters")
    //     // .with_print(true)
    //     .build()?;
    Ok(())
}
