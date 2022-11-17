use clap::{arg, command, value_parser, Arg, ArgAction};

use rest_api as lib;
use lib::enable_logging;
use lib::config_parser::read_config;
use lib::database::interfaces::{SQLite3Interface, DatabaseInterface};
use lib::app::App;
use lib::api_http_server::routing::{BasicRoute, Route};
use lib::api_http_server::http::run_app_server;

#[tokio::main]
async fn main() {
    // CLI FLAG MATCHING
    let cli_matches = command!()
        .arg(
            arg!(
                -c --config <FILE> "Sets the path to a custom config file"
           )
           .value_parser(value_parser!(String))
        )
        .arg(
            Arg::new("resetdb")
                .short('r')
                .long("resetdb")
                .action(ArgAction::SetTrue)
        )
        .get_matches();

    let optional_path: Option<String> = cli_matches.get_one::<String>("config").cloned();

    // Read generic settings and table schemas
    let (config, tables) = read_config(optional_path.as_deref());

    enable_logging(&config);

    // database is deleted and recreated when connecting
    if cli_matches.get_flag("resetdb") {
        SQLite3Interface::delete_db(&config)
    }

    let (interface, existing) = SQLite3Interface::connect(&config);

    // assumes if existing, config tables match the database tables' structure
    // if not existing, recreate tables from config schemas
    if !existing {
        interface.create_tables_from_schemas(tables.values().into_iter().collect())
    }

    // construct route to table schema mappings
    let mut routes = Vec::new();
    for table in tables {
        routes.push(Box::new(
            BasicRoute {route: table.0, table_schema: table.1}
        ) as Box<dyn Route + Send + Sync>)
    }

    let app = App {
        routes,
        middleware: vec![],
        database_interface: Box::new(interface)
    };

    run_app_server(config.get("host").expect("No host in config").parse().expect("Can't parse IP"), app).await;
}
