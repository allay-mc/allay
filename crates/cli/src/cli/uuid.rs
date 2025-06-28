use allay::{config, lock};
use clap::{ArgMatches, Command};
use std::{path::PathBuf, process};

pub(crate) fn cmd() -> Command {
    Command::new("uuid").about("Mange project's UUIDs")
}

pub(crate) fn run(matches: &ArgMatches) -> process::ExitCode {
    let maybe_project = match matches.get_one::<&PathBuf>("project-dir") {
        Some(dir) => allay::Project::load_from_dir(dir),
        None => allay::Project::load_from_within(),
    };
    let project = match maybe_project {
        Ok(project) => project,
        Err(error) => {
            log::error!("{}", error);
            log::info!("Try `allay health` to automatically find and fix issues");
            return process::ExitCode::FAILURE;
        }
    };

    // TODO: update uuid db when config or source is changed

    match project.lock {
        Some(lock) => {
            print_table(
                "Behavior Pack",
                lock.uuid_table.bp.as_ref(),
                &project.config.bp.dependencies,
            );
            print_table(
                "Resource Pack",
                lock.uuid_table.rp.as_ref(),
                &project.config.rp.dependencies,
            );
            print_table("Skin Pack", lock.uuid_table.sp.as_ref(), &[]);
            print_table("World Template", lock.uuid_table.sp.as_ref(), &[]);
            process::ExitCode::SUCCESS
        }
        None => {
            log::error!("No lock file found");
            process::ExitCode::FAILURE
        }
    }
}

fn print_table(title: &str, uuids: Option<&lock::Uuids>, deps: &[config::Dependency]) {
    // NOTE: internal dependencies are not emitted
    // NOTE: external dependencies that are built-in are included as well for convenience

    let style = console::Style::new().bold();
    let table_format = *prettytable::format::consts::FORMAT_BOX_CHARS;

    if let Some(bp) = uuids {
        println!("{}", style.apply_to(title));
        let mut table_bp = prettytable::Table::new();
        let mut header_row = prettytable::Row::empty();
        let mut content_row = prettytable::Row::empty();
        header_row.add_cell(prettytable::cell!("Header"));
        content_row.add_cell(prettytable::cell!(bp.header.uuid));
        for (kind, entry) in &bp.modules {
            header_row.add_cell(prettytable::cell!(format!("Module ({})", kind)));
            content_row.add_cell(prettytable::cell!(entry.uuid.to_string()));
        }
        for ext_dependency in deps {
            header_row.add_cell(prettytable::cell!("Dependency (external)"));
            content_row.add_cell(prettytable::cell!(ext_dependency.to_string()));
        }
        table_bp.set_titles(header_row);
        table_bp.add_row(content_row);
        table_bp.set_format(table_format);
        table_bp.print_tty(true).unwrap();
        println!();
    }
}
