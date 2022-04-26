mod parser;
mod builder;
mod listener;
mod webserver;
// mod wsserver;

use webserver::{WebServer, WebOptions};

use clap::{Arg, Command};

use std::{net::SocketAddr, thread};


fn main() {
    let matches = Command::new(clap::crate_name!())
        .version(clap::crate_version!())
        .author(clap::crate_authors!())
        .subcommands(vec![
            Command::new("run").args(&[
                Arg::new("path").required(false).default_value("./src"),
                Arg::new("addr").required(false).default_value("127.0.0.1:8080")
            ]),
            Command::new("build").args(&[
                Arg::new("source-path").required(false).default_value("./src"),
                Arg::new("build-path").required(false).default_value("./build")
            ])
        ])
        .subcommand(Command::new("help"))
        .get_matches();


    match matches.subcommand() {
        Some(("run", args)) => {
            let addr = args.value_of("addr").unwrap().parse::<SocketAddr>().unwrap();

            let (mut web_server, shutdown) = WebServer::new(addr.clone(), WebOptions::default()).unwrap();
            web_server.listen().unwrap();

            println!("WebServer listening at {}", &addr);

            let web_handle = thread::spawn(move || {
                web_server.handle_connections();
            });

            ctrlc::set_handler(move || {
                shutdown.shutdown().unwrap();
            }).expect("Failed to set Ctrl-C handler");

            web_handle.join().unwrap();
            println!("\nWebServer stopped!");
        },

        Some(("build", args)) => {
            let source = args.value_of("source-path").unwrap().into();
            let dest = args.value_of("build-path").unwrap().into();

            match builder::build(source, dest) {
                Ok(stats) => {
                    println!("Done!\n\nFiles copied: {}\nFiles parsed: {}", stats.files_copied, stats.files_parsed);
                },
                Err(e) => eprintln!("Error building: {}", e)
            }
        },

        Some((name, _)) => eprintln!("Unknown subcommand: {}", name),
        None => println!("Use --help for command usage")
    }
}
