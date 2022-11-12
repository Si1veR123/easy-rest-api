use clap::{arg, command, value_parser, Arg, ArgAction};

use std::net::SocketAddr;

use std::sync::Mutex;

use lib::api_http_server::routing::BasicRoute;
use rest_api as lib;
use lib::enable_logging;
use lib::config_parser::read_config;
use lib::database::interfaces::{SQLite3Interface, DatabaseInterface};
use lib::app::App;
use lib::routes;
use lib::api_http_server::routing::Route;
use lib::api_http_server::http::run_app_server;

#[tokio::main]
async fn main() {
    let cli_matches = command!()
        .arg(
            arg!(
                -c --config <FILE> "Sets a custom config file"
           )
           .value_parser(value_parser!(String))
        )
        .arg(
            Arg::new("resetdb")
                .short('r')
                .long("reset database")
                .action(ArgAction::SetTrue)
        )
        .get_matches();

    let optional_path: Option<String> = cli_matches.get_one::<String>("config").cloned();
    let config = read_config(optional_path.as_deref());

    enable_logging(&config);

    if cli_matches.get_flag("resetdb") {
        SQLite3Interface::delete_db(&config)
    }

    let interface = SQLite3Interface::connect(&config);

    let routes = routes!(
        ("/people", "people sql table"),
        ("/jobs", "jobs sql table")
    );

    let app = App {
        routes: routes,
        middleware: vec![],
        database_interface: Box::new(interface)
    };

    run_app_server(SocketAddr::from(([127, 0, 0, 1], 3000)), app).await;
}
