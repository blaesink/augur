/*!
    Handle things like arrow keys for selecting menus etc.
*/
use crate::roll::Roll;
use anyhow::{anyhow, Context, Result};
use crossterm::{
    event::{read, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers},
    terminal::{enable_raw_mode, Clear, ClearType},
    ExecutableCommand,
};
use std::{collections::HashMap, io::stdout};

type Menu<T> = HashMap<String, Box<dyn Roll<T>>>;

pub fn handle_menu(menu: &Menu<String>) -> Result<()> {
    enable_raw_mode().with_context(|| "Couldn't enable raw mode!")?;
    Ok(select_menu_arrow_keys(menu)?)
}

fn is_quit_sequence(key: &KeyEvent) -> bool {
    key.code == KeyCode::Char('c') && key.modifiers == KeyModifiers::CONTROL
}

fn is_arrow_key(event: &KeyEvent) -> Option<KeyCode> {
    match event.code {
        KeyCode::Down | KeyCode::Up => Some(event.code),
        _ => None,
    }
}

fn print_menu(cursor_index: usize, menu: &Menu<String>) {
    for (i, opt) in menu.keys().enumerate() {
        if cursor_index == i {
            println!("> {}. {opt}\r", i + 1);
        } else {
            println!("{}. {opt}\r", i + 1);
        }
    }
}

fn show_and_clear(msg: impl Into<String>, cursor_index: usize, menu: &Menu<String>) -> Result<()> {
    stdout().execute(Clear(ClearType::All))?;
    println!("{}", msg.into());
    std::thread::sleep(std::time::Duration::from_millis(700));
    stdout().execute(Clear(ClearType::All))?;
    print_menu(cursor_index, menu);

    Ok(())
}

fn select_menu_arrow_keys(menu: &HashMap<String, Box<dyn Roll<String>>>) -> Result<()> {
    // Show a '>' key next to the current item.
    let options = menu.keys().into_iter().collect::<Vec<&String>>();
    let mut cursor_index = 0usize;
    let mut current_key: &String = options[cursor_index];

    print_menu(cursor_index, menu);

    loop {
        let event = read()?;
        match event {
            Event::Key(event) if event.kind == KeyEventKind::Press => {
                if is_quit_sequence(&event) {
                    return Err(anyhow!("Exiting\r"));
                }

                if let Some(k) = is_arrow_key(&event) {
                    match k {
                        KeyCode::Down => {
                            cursor_index = cursor_index.saturating_add(1);
                            cursor_index = std::cmp::min(cursor_index, menu.len() - 1);
                        }
                        KeyCode::Up => cursor_index = cursor_index.saturating_sub(1),
                        _ => {}
                    }
                    current_key = options[cursor_index];
                    // should go somewhere else
                    stdout().execute(Clear(ClearType::All))?;
                    print_menu(cursor_index, menu);
                }
                match event.code {
                    KeyCode::Enter => {
                        println!("\nRolling {current_key}...\r");
                        let result = menu
                            .get(current_key)
                            .and_then(|k| k.roll())
                            .with_context(|| "Bad roll!")?;
                        show_and_clear(format!("-> {result}\r"), cursor_index, menu)?;
                    }
                    KeyCode::Char('h') => {
                        show_and_clear("Help!\r", cursor_index, menu)?;
                    }
                    _ => {}
                }
            }
            _ => {}
        }
    }
}
