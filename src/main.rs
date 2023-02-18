use std::{io::stdout, thread::sleep, time::Duration};

use crossterm::event::{read, Event, KeyCode};
use tty_interface::{Interface, Result, Position, pos};

fn main() {
    execute().unwrap()
}

fn execute() -> Result<()> {
    let mut stdout = stdout();
    let mut interface = Interface::new_alternate(&mut stdout)?;

    main_menu(&mut interface)?;
    sleep(Duration::from_millis(2000));

    interface.exit()?;

    Ok(())
}

fn main_menu(interface: &mut Interface) -> Result<()> {
    let options = ["Meal Plan", "Grocery", "Recipes", "Pantry"];
    let mut selected = 0;
    loop {
        interface.clear_rest_of_interface(pos!(0, 0));
        interface.set(pos!(0, 0), "== Main Menu ==");
    
        for (item, option) in options.iter().enumerate() {
            if selected == item {
                interface.set(pos!(2, item as u16 + 2), ">");
            }

            interface.set(pos!(4, item as u16 + 2), option);
        }
    
        interface.apply()?;

        loop {
            match read()? {
                Event::Key(evt) => match evt.code {
                    KeyCode::Up => {
                        if selected > 0 {
                            selected -= 1;
                        } else {
                            selected = options.len() - 1;
                        }
                        break;
                    }
                    KeyCode::Down => {
                        if selected < options.len() - 1 {
                            selected += 1;
                        } else {
                            selected = 0;
                        }
                        break;
                    }
                    KeyCode::Esc => return Ok(()),
                    _ => {}
                }
                _ => {}
            }
        }
    }
}
