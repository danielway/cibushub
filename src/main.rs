use std::io::stdout;

use crossterm::event::{read, Event, KeyCode};
use tty_interface::{Interface, Result, Position, pos};

fn main() {
    execute().unwrap()
}

fn execute() -> Result<()> {
    let mut stdout = stdout();
    let mut interface = Interface::new_alternate(&mut stdout)?;

    loop {
        if let Some(_screen) = main_menu(&mut interface)? {
            // TODO
        } else {
            break;
        }
    }

    interface.exit()?;

    Ok(())
}

fn main_menu(interface: &mut Interface) -> Result<Option<Screen>> {
    let options = Screen::all();
    let mut selected = 0;
    loop {
        interface.clear_rest_of_interface(pos!(0, 0));
        interface.set(pos!(0, 0), "== Main Menu ==");
    
        for (item, option) in options.iter().enumerate() {
            if selected == item {
                interface.set(pos!(2, item as u16 + 2), ">");
            }

            interface.set(pos!(4, item as u16 + 2), &option.get_text());
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
                    KeyCode::Enter => return Ok(Some(options[selected])),
                    KeyCode::Esc => return Ok(None),
                    _ => {}
                }
                _ => {}
            }
        }
    }
}

#[derive(Copy, Clone)]
enum Screen { MainMenu, MealPlan, Grocery, Recipes, Pantry }

impl Screen {
    fn get_text(self) -> String {
        match self {
            Screen::MainMenu => "Main Menu",
            Screen::MealPlan => "Meal Plan",
            Screen::Grocery => "Grocery",
            Screen::Recipes => "Recipes",
            Screen::Pantry => "Pantry"
        }.to_string()
    }

    fn all() -> [Screen; 5] {
        [Screen::MainMenu, Screen::MealPlan, Screen::Grocery, Screen::Recipes, Screen::Pantry]
    }
}
