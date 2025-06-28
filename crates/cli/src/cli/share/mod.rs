#![cfg(feature = "share")]

use super::ext::*;
use crate::utils;
use allay::project::Profile;
use clap::{Arg, ArgMatches, Command};
use get_if_addrs::get_if_addrs;
use qrcode::QrCode;
use std::{
    fs,
    net::{IpAddr, Ipv4Addr, SocketAddr},
    os::unix::fs::MetadataExt,
    path::PathBuf,
    process,
};
use tokio::signal;
use warp::http::{HeaderMap, HeaderValue};
use warp::Filter;

mod pages;

pub(crate) fn cmd() -> Command {
    Command::new("share")
        .about("Launch a HTTP server where built add-ons can be downloaded")
        .arg(
            Arg::new("host")
                .long("host")
                .help("The host address to use for the server")
                .value_parser(clap::value_parser!(IpAddr)),
        )
        .arg(
            Arg::new("port")
                .long("port")
                .help("The port to use for the server")
                .value_parser(clap::value_parser!(u16))
                .default_value("8000"),
        )
        .arg_build_opts()
}

#[tokio::main(flavor = "current_thread")]
pub(crate) async fn run(matches: &ArgMatches) -> process::ExitCode {
    let quiet = matches.get_flag("quiet");

    let maybe_project = match matches.get_one::<&PathBuf>("project-dir") {
        Some(dir) => allay::Project::load_from_dir(dir),
        None => allay::Project::load_from_within(),
    };
    let mut project = match maybe_project {
        Ok(project) => project,
        Err(error) => {
            log::error!("{}", error);
            log::info!("Try `allay health` to automatically find and fix issues");
            return process::ExitCode::FAILURE;
        }
    };

    let fallback_host = default_host();
    let host: &IpAddr = matches.get_one("host").unwrap_or(&fallback_host);
    let port: u16 = *matches.get_one("port").unwrap();

    if host.is_loopback() {
        log::warn!("The used IP address is a loopback and may not be reachable from other devices");
    }

    let addr = SocketAddr::new(*host, port);

    let mut build_context = utils::build::build_context_from_args_and_config(matches, &project.config);
    for profile in [Profile::Debug, Profile::Release] {
        build_context.with_profile(profile);
        match utils::build::build_project(&mut project, &build_context) {
            Ok(_) => (),
            Err(_) => {
                return process::ExitCode::FAILURE;
            }
        };
    }

    let proj_name = project.slugify_project_name();
    let file_name_debug = project.slugify_project_name_full(Profile::Debug);
    let file_name_release = project.slugify_project_name_full(Profile::Release);

    let file_debug = allay::paths::project::build_file_debug(&project.root);
    let file_release = allay::paths::project::build_file_release(&project.root);

    let file_debug_exists = file_debug.exists();
    let file_release_exists = file_release.exists();

    let mut headers_debug = HeaderMap::new();
    if let Ok(metadata) = fs::metadata(&file_debug) {
        headers_debug.insert("Content-Length", metadata.size().into());
    }
    headers_debug.insert(
        "Content-Disposition",
        HeaderValue::from_str(&format!(
            r#"attachment; filename="{}.mcaddon""#,
            file_name_debug,
        ))
        .unwrap(),
    );
    let mut headers_release = HeaderMap::new();
    if let Ok(metadata) = fs::metadata(&file_release) {
        headers_release.insert("Content-Length", metadata.size().into());
    }
    headers_release.insert(
        "Content-Disposition",
        HeaderValue::from_str(&format!(
            r#"attachment; filename="{}.mcaddon""#,
            file_name_release,
        ))
        .unwrap(),
    );
    let download_debug = warp::path("debug")
        .and(warp::fs::file(file_debug))
        .with(warp::reply::with::headers(headers_debug));
    let download_release = warp::path("release")
        .and(warp::fs::file(file_release))
        .with(warp::reply::with::headers(headers_release));
    let script_download = warp::path("download.js").map(|| {
        warp::http::Response::builder()
            .header("Content-Type", "text/javascript; charset=utf-8")
            .body(include_str!("download.js"))
    });
    let script_notfound = warp::path("notfound.js").map(|| {
        warp::http::Response::builder()
            .header("Content-Type", "text/javascript; charset=utf-8")
            .body(include_str!("notfound.js"))
    });
    let style = warp::path("style.css").map(|| {
        warp::http::Response::builder()
            .header("Content-Type", "text/css; charset=utf-8")
            .body(include_str!("style.css"))
    });
    let home = warp::path::end().map(move || {
        warp::reply::html(
            pages::download(
                &proj_name,
                file_release_exists.then_some(&file_name_release),
                file_debug_exists.then_some(&file_name_debug),
            )
            .into_string(),
        )
    });

    let routes = warp::get()
        .and(home)
        .or(download_debug)
        .or(download_release)
        .or(script_download)
        .or(script_notfound)
        .or(style)
        .or(warp::any().map(|| {
            warp::reply::with_status(
                warp::reply::html(pages::not_found().into_string()),
                warp::http::StatusCode::NOT_FOUND,
            )
        }));

    let signal_handler = async move {
        signal::ctrl_c().await.expect("Failed to listen for Ctrl-C");
        if !quiet {
            println!();
        }
        log::info!("Gracefully shutting down");
    };

    let (bound_addr, server) =
        warp::serve(routes).bind_with_graceful_shutdown(addr, signal_handler);
    let string_addr = format!("http://{}:{}", bound_addr.ip(), bound_addr.port());
    log::info!("Serving at {}", string_addr);

    let qr_code = QrCode::new(&string_addr).expect("Failed to generate QR Code for address");
    let terminal_ratio_factor = 2;
    let string_qr_code = qr_code
        .render()
        .light_color(" ".repeat(terminal_ratio_factor).as_str())
        .dark_color("\u{2588}".repeat(terminal_ratio_factor).as_str())
        .build();
    let width = string_qr_code
        .lines()
        .map(|line| line.chars().count())
        .max()
        .expect("QR Code is corrupted");
    let padding = " ".repeat((width - string_addr.chars().count()) / 2);
    let mut bottom = String::new();
    bottom.push_str(&padding);
    bottom.push_str(&string_addr);
    if !quiet {
        println!("{}", string_qr_code);
        println!("{}", bottom);
    }

    server.await;

    process::ExitCode::SUCCESS
}

fn default_host() -> IpAddr {
    match get_if_addrs() {
        Ok(addrs) => {
            for addr in addrs {
                if !addr.is_loopback() {
                    return addr.ip();
                }
            }
        }
        Err(error) => {
            log::debug!("Failed to infer address: {}", error);
        }
    };
    IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0))
}
