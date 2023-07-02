use std::{error::Error, time::Duration, sync::mpsc, thread};
use crossterm::{terminal::{self, EnterAlternateScreen, LeaveAlternateScreen}, 
ExecutableCommand, cursor::{Hide, Show}, event::{Event, self, KeyCode}};
use invaders::{frame::{self, new_frame}, render};
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

    // Render loop in sub thread
    let (render_tx, render_rx) = mpsc::channel();
    // move converts any variables captured by reference or mutable reference to variables captured by value.
    // here is an example:
    // let data = vec![1, 2, 3];
    // let closure = move || println!("captured {data:?} by value");
    // now data is no longer available, it is owned by the closure
    let render_handle = thread::spawn(move || {
        let mut last_frame = frame::new_frame();
        let mut stdout = io::stdout();
        render::render(&mut stdout, &last_frame, &last_frame, true);
        loop {
            let curr_frame = match render_rx.recv() {
                Ok(x) => x,
                Err(_) => break
            };
            render::render(&mut stdout, &last_frame, &curr_frame, false);
            last_frame = curr_frame;
        }        
    });

    // 'loop_label so we c  an access it
    'gameloop: loop {
        // Pre-frame init
        let curr_frame = new_frame();

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

        // Draw & render
        let _ = render_tx.send(curr_frame);
        thread::sleep(Duration::from_millis(1));
    }

    // cleanup
    drop(render_tx);
    render_handle.join().unwrap();
    audio.wait();
    stdout.execute(Show)?;  // crash
    stdout.execute(LeaveAlternateScreen)?;  // crash
    terminal::disable_raw_mode()?;
    return Ok(());
}
