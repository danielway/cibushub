use std::{io::stdout, thread::sleep, time::Duration};

use tty_interface::{Interface, Result, Position, pos};

fn main() {
    execute().unwrap()
}

fn execute() -> Result<()> {
    let mut stdout = stdout();
    let mut interface = Interface::new_alternate(&mut stdout)?;

    interface.set(pos!(0, 0), "Hello, world!");
    interface.apply()?;

    sleep(Duration::from_millis(2000));

    interface.exit()?;

    Ok(())
}
