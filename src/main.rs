mod file;
mod roll;
mod table;
mod ui;

use anyhow::Result;
use file::read_lines;
use roll::Roll;
use std::{collections::HashMap, fs};
use table::{Table, WeightedTable};
use ui::handle_menu;

fn prettify_snake_case(str: String) -> Option<String> {
    str.rsplit(".")
        .last()
        .and_then(|s| Some(s.split('_')))
        .and_then(|split| {
            Some(
                split
                    .map(|s| {
                        let mut v = s.chars().collect::<Vec<char>>();
                        v[0].make_ascii_uppercase();

                        Some(v.into_iter().collect::<String>())
                    })
                    .collect::<Option<Vec<String>>>()?,
            )
        })
        .and_then(|s| Some(s.join(" ")))
}

fn main() -> Result<()> {
    let mut hm = HashMap::<String, Box<dyn Roll<String>>>::new();
    let resource_files = fs::read_dir("./resources/")?
        .filter_map(|f| {
            f.ok().and_then(|e| {
                e.path()
                    .file_name()
                    .and_then(|name| name.to_str().map(|s| String::from(s)))
            })
        })
        .collect::<Vec<String>>();

    for file in resource_files {
        let lines = read_lines(format!("resources/{file}"))?
            .filter_map(|l| l.ok())
            .collect::<Vec<String>>();

        // if let Ok(wt) = WeightedTable::try_from(lines) {
        //     let tab = Box::new(wt);
        // } else {
        //     let tab = Box::new(Table(lines));
        // }
        let tab: Box<dyn Roll<String>> = match WeightedTable::try_from(lines.clone()) {
            Ok(wt) => Box::new(wt),
            Err(_) => Box::new(Table(lines)),
        };
        hm.insert(
            prettify_snake_case(file)
                .ok_or_else(|| anyhow::anyhow!("Couldn't prettify filename!"))?,
            tab,
        );
    }
    hm.insert(
        String::from("Roll d20"),
        Box::new(crate::table::Table(
            (0..20).map(|i| i.to_string()).collect::<Vec<String>>(),
        )),
    );
    handle_menu(&hm)?;

    Ok(())
}
