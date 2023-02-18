use std::{io::stdout, thread::sleep, time::Duration};

use crossterm::event::{read, Event, KeyCode};
use tty_interface::{pos, Interface, Position, Result};

fn main() {
    execute().unwrap()
}

fn execute() -> Result<()> {
    let mut stdout = stdout();
    let mut interface = Interface::new_alternate(&mut stdout)?;

    loop {
        if let Some(screen) = main_menu(&mut interface)? {
            match screen {
                Screen::MealPlan => meal_plan(&mut interface)?,
                Screen::Grocery => grocery(&mut interface)?,
                Screen::Recipes => recipes(&mut interface)?,
                Screen::Pantry => pantry(&mut interface)?,
            }
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
                },
                _ => {}
            }
        }
    }
}

fn meal_plan(interface: &mut Interface) -> Result<()> {
    loop {
        interface.clear_rest_of_interface(pos!(0, 0));
        interface.set(pos!(0, 0), "== Meal Plan ==");

        interface.apply()?;

        sleep(Duration::from_millis(2000));

        break;
    }

    Ok(())
}

fn grocery(interface: &mut Interface) -> Result<()> {
    loop {
        interface.clear_rest_of_interface(pos!(0, 0));
        interface.set(pos!(0, 0), "== Grocery ==");

        interface.apply()?;

        sleep(Duration::from_millis(2000));

        break;
    }

    Ok(())
}

fn recipes(interface: &mut Interface) -> Result<()> {
    loop {
        interface.clear_rest_of_interface(pos!(0, 0));
        interface.set(pos!(0, 0), "== Recipes ==");

        interface.apply()?;

        sleep(Duration::from_millis(2000));

        break;
    }

    Ok(())
}

fn pantry(interface: &mut Interface) -> Result<()> {
    loop {
        interface.clear_rest_of_interface(pos!(0, 0));
        interface.set(pos!(0, 0), "== Pantry ==");

        interface.apply()?;

        sleep(Duration::from_millis(2000));

        break;
    }

    Ok(())
}

#[derive(Copy, Clone)]
enum Screen {
    MealPlan,
    Grocery,
    Recipes,
    Pantry,
}

impl Screen {
    fn get_text(self) -> String {
        match self {
            Screen::MealPlan => "Meal Plan",
            Screen::Grocery => "Grocery",
            Screen::Recipes => "Recipes",
            Screen::Pantry => "Pantry",
        }
        .to_string()
    }

    fn all() -> [Screen; 4] {
        [
            Screen::MealPlan,
            Screen::Grocery,
            Screen::Recipes,
            Screen::Pantry,
        ]
    }
}
