mod file;
mod roll;
mod ui;

use anyhow::Result;
use file::read_lines;
use std::{collections::HashMap, fs};
use ui::handle_menu;

fn main() -> Result<()> {
    let mut hm = HashMap::<String, Vec<String>>::new();
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
        hm.insert(
            file.clone()
                .rsplit(".")
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
                .ok_or_else(|| anyhow::anyhow!("Non-readable filename {}", file))?
                .join(" "),
            // Values (opening the file and reading its contents to a vec)
            read_lines(format!("resources/{file}"))?
                .filter_map(|l| l.ok().and_then(|s| Some(s)))
                .collect::<Vec<String>>(),
        );
    }
    hm.insert(
        String::from("Roll d20"),
        (0..20).map(|i| i.to_string()).collect::<Vec<String>>(),
    );
    handle_menu(&hm)?;

    Ok(())
}
