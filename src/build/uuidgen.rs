//! The uuids.csv table has the following form:
//!
//! +------+--------+--------+--------------+
//! | Type | Header | Module | Dependencies |
//! +------+--------+--------+--------------+
//! | BP   | <uuid> | <uuid> | <uuid>*      |
//! | RP   | <uuid> | <uuid> | <uuid>*      |
//! | SP   | <uuid> | <uuid> | <uuid>*      |
//! | WT   | <uuid> | <uuid> | <uuid>*      |
//! +------+--------+--------+--------------+

// TODO: update file when uuid is updated

use std::fs::File;

use prettytable::{cell, table, Table};

use crate::paths;

pub(crate) fn read_uuids() -> prettytable::csv::Result<Table> {
    Table::from_csv_file(paths::uuids())
}

pub(crate) fn save_uuids(table: &Table) -> std::io::Result<()> {
    let f = File::create(paths::uuids())?;
    table.to_csv(f).expect("cannot initialize UUID table");
    Ok(())
}

pub(crate) fn new() -> Table {
    table!(
        ["Type", "Header", "Module", "Dependencies"],
        ["BP", "", "", ""],
        ["RP", "", "", ""],
        ["SP", "", "", ""],
        ["WT", "", "", ""]
    )
}

pub(crate) fn bp_header(table: &Table) -> Option<String> {
    let content = table
        .get_row(1)
        .expect("invalid UUID table; 2nd row should exist")
        .get_cell(1)
        .expect("invalid UUID table; 2nd cell should exist")
        .get_content();
    if content.is_empty() {
        None
    } else {
        Some(content)
    }
}

pub(crate) fn bp_module(table: &Table) -> Option<String> {
    let content = table
        .get_row(1)
        .expect("invalid UUID table; 2nd row should exist")
        .get_cell(2)
        .expect("invalid UUID table; 3rd cell should exist")
        .get_content();
    if content.is_empty() {
        None
    } else {
        Some(content)
    }
}

pub(crate) fn bp_deps(table: &Table) -> Vec<String> {
    table
        .get_row(1)
        .expect("invalid UUID table; 2nd row should exist")
        .get_cell(3)
        .expect("invalid UUID table; 3rd cell should exist")
        .get_content()
        .lines()
        .map(|s| s.to_string())
        .collect()
}

pub(crate) fn rp_header(table: &Table) -> Option<String> {
    let content = table
        .get_row(2)
        .expect("invalid UUID table; 3rd row should exist")
        .get_cell(1)
        .expect("invalid UUID table; 2nd cell should exist")
        .get_content();
    if content.is_empty() {
        None
    } else {
        Some(content)
    }
}

pub(crate) fn rp_module(table: &Table) -> Option<String> {
    let content = table
        .get_row(2)
        .expect("invalid UUID table; 3rd row should exist")
        .get_cell(2)
        .expect("invalid UUID table; 3rd cell should exist")
        .get_content();
    if content.is_empty() {
        None
    } else {
        Some(content)
    }
}

pub(crate) fn rp_deps(table: &Table) -> Vec<String> {
    table
        .get_row(2)
        .expect("invalid UUID table; 3rd row should exist")
        .get_cell(3)
        .expect("invalid UUID table; 3rd cell should exist")
        .get_content()
        .lines()
        .map(|s| s.to_string())
        .collect()
}

pub(crate) fn sp_header(table: &Table) -> Option<String> {
    let content = table
        .get_row(3)
        .expect("invalid UUID table; 4th row should exist")
        .get_cell(1)
        .expect("invalid UUID table; 2nd cell should exist")
        .get_content();
    if content.is_empty() {
        None
    } else {
        Some(content)
    }
}

pub(crate) fn sp_module(table: &Table) -> Option<String> {
    let content = table
        .get_row(3)
        .expect("invalid UUID table; 4th row should exist")
        .get_cell(2)
        .expect("invalid UUID table; 3rd cell should exist")
        .get_content();
    if content.is_empty() {
        None
    } else {
        Some(content)
    }
}

pub(crate) fn sp_deps(table: &Table) -> Vec<String> {
    table
        .get_row(3)
        .expect("invalid UUID table; 4th row should exist")
        .get_cell(3)
        .expect("invalid UUID table; 3rd cell should exist")
        .get_content()
        .lines()
        .map(|s| s.to_string())
        .collect()
}

pub(crate) fn wt_header(table: &Table) -> Option<String> {
    let content = table
        .get_row(4)
        .expect("invalid UUID table; 5th row should exist")
        .get_cell(1)
        .expect("invalid UUID table; 2nd cell should exist")
        .get_content();
    if content.is_empty() {
        None
    } else {
        Some(content)
    }
}

pub(crate) fn wt_module(table: &Table) -> Option<String> {
    let content = table
        .get_row(4)
        .expect("invalid UUID table; 5th row should exist")
        .get_cell(2)
        .expect("invalid UUID table; 3rd cell should exist")
        .get_content();
    if content.is_empty() {
        None
    } else {
        Some(content)
    }
}

pub(crate) fn wt_deps(table: &Table) -> Vec<String> {
    table
        .get_row(4)
        .expect("invalid UUID table; 5th row should exist")
        .get_cell(3)
        .expect("invalid UUID table; 3rd cell should exist")
        .get_content()
        .lines()
        .map(|s| s.to_string())
        .collect()
}

pub(crate) fn update_bp_header(table: &mut Table, uuid: String) -> &mut Table {
    table
        .get_mut_row(1)
        .expect("invalid UUID table; 2nd row should exist")
        .set_cell(cell!(uuid), 1)
        .unwrap();
    table
}

pub(crate) fn update_bp_module(table: &mut Table, uuid: String) -> &mut Table {
    table
        .get_mut_row(1)
        .expect("invalid UUID table; 2nd row should exist")
        .set_cell(cell!(uuid), 2)
        .unwrap();
    table
}

pub(crate) fn update_rp_header(table: &mut Table, uuid: String) -> &mut Table {
    table
        .get_mut_row(2)
        .expect("invalid UUID table; 3rd row should exist")
        .set_cell(cell!(uuid), 1)
        .unwrap();
    table
}

pub(crate) fn update_rp_module(table: &mut Table, uuid: String) -> &mut Table {
    table
        .get_mut_row(2)
        .expect("invalid UUID table; 3rd row should exist")
        .set_cell(cell!(uuid), 2)
        .unwrap();
    table
}

pub(crate) fn update_sp_header(table: &mut Table, uuid: String) -> &mut Table {
    table
        .get_mut_row(3)
        .expect("invalid UUID table; 4th row should exist")
        .set_cell(cell!(uuid), 1)
        .unwrap();
    table
}

pub(crate) fn update_sp_module(table: &mut Table, uuid: String) -> &mut Table {
    table
        .get_mut_row(3)
        .expect("invalid UUID table; 4th row should exist")
        .set_cell(cell!(uuid), 2)
        .unwrap();
    table
}

pub(crate) fn update_wt_header(table: &mut Table, uuid: String) -> &mut Table {
    table
        .get_mut_row(4)
        .expect("invalid UUID table; 5th row should exist")
        .set_cell(cell!(uuid), 1)
        .unwrap();
    table
}

pub(crate) fn update_wt_module(table: &mut Table, uuid: String) -> &mut Table {
    table
        .get_mut_row(4)
        .expect("invalid UUID table; 5th row should exist")
        .set_cell(cell!(uuid), 2)
        .unwrap();
    table
}
