use std::{error::Error, time::Duration};
use crossterm::{terminal::{self, EnterAlternateScreen, LeaveAlternateScreen}, 
ExecutableCommand, cursor::{Hide, Show}, event::{Event, self, KeyCode}};
use rusty_audio::Audio;
use std::io;


// [dyn](https://doc.rust-lang.org/std/keyword.dyn.html)
fn main() -> Result<(), Box<dyn Error>> {
    // audio runs in a parallel section
    let mut audio = Audio::new();
    audio.add("explode", "sound/explode.wav");
    audio.add("lose", "sound/lose.wav");
    audio.add("move", "sound/move.wav");
    audio.add("pew", "sound/pew.wav");
    audio.add("startup", "sound/startup.wav");
    audio.add("win", "sound/win.wav");

    audio.play("startup");

    let mut stdout = io::stdout();
    terminal::enable_raw_mode()?;  // question mark means program will crash when have an error
    stdout.execute(EnterAlternateScreen);
    stdout.execute(Hide);

    // 'loop_label so we can access it
    'gameloop: loop {
        while event::poll(Duration::default())? {
            if let Event::Key(key_event) = event::read()? {
                match key_event.code {
                    KeyCode::Esc | KeyCode::Char('q') => {
                        audio.play("lose");
                        break 'gameloop;
                    }
                    _ => {}
                }
            }
        }
    }

    // cleanup
    audio.wait();
    stdout.execute(Show)?;  // crash
    stdout.execute(LeaveAlternateScreen)?;  // crash
    terminal::disable_raw_mode()?;
    return Ok(());
}
