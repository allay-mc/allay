// FIXME: segmentation fault on android

use super::build;
use super::prelude::*;
use allay::paths;
use clap::{Arg, ArgMatches, Command};
use local_ip_address::local_ip;
use qrcode::QrCode;
use std::net::{IpAddr, Ipv4Addr};
use std::process::ExitCode;
use warp::Filter;

pub fn cmd() -> Command {
    Command::new("share")
        .about("Serve an HTTP server for sharing the recently built add-ons")
        .arg(
            Arg::new("host")
                .short('n')
                .long("hostname")
                .help("Host address to bind the server to"),
        )
        .arg(
            Arg::new("port")
                .short('p')
                .long("port")
                .help("Port to use for the HTTP connection")
                .value_parser(clap::value_parser!(u16))
                .default_value("6464"),
        )
        .arg_build_opts()
}

#[tokio::main(flavor = "current_thread")]
pub async fn run(matches: &ArgMatches) -> ExitCode {
    build::run(matches);

    let host: Option<&String> = matches.get_one("host");
    let host: IpAddr = match host {
        Some(h) => h.parse::<IpAddr>().expect("invalid host address"),
        None => local_ip().unwrap_or(IpAddr::V4(Ipv4Addr::UNSPECIFIED)),
    };
    let port: &u16 = matches.get_one("port").unwrap();

    let url = format!("http://{host}:{port}");
    match QrCode::new(url) {
        Ok(code) => {
            let text = code
                .render()
                .light_color("  ")
                .dark_color("\u{2588}\u{2588}")
                .build();
            println!("Scan the QR code below to download the built add-on");
            println!("{}", text);
        }
        Err(e) => log::error!("Error while trying to generate QR code: {}", e),
    };
    let app =
        warp::path::end()
            .and(warp::fs::file(paths::build()))
            .with(warp::reply::with::header(
                "Content-Disposition",
                r#"attachment; filename="build.mcaddon""#, // TODO: use project name?
            ));
    warp::serve(app).run((host, *port)).await;
    ExitCode::SUCCESS
}
