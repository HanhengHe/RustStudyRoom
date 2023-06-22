use std::error::Error;

use rusty_audio::Audio;

// [dyn](https://doc.rust-lang.org/std/keyword.dyn.html)
fn main() -> Result<(), Box<dyn Error>> {
    // audio runs in a parallel section
    let mut audio = Audio::new();
    audio.add("explode", "../sound/explode.wav");
    audio.add("lose", "../sound/lose.wav");
    audio.add("move", "../sound/move.wav");
    audio.add("pew", "../sound/pew.wav");
    audio.add("startup", "../sound/startup.wav");
    audio.add("win", "../sound/win.wav");
    audio.play("startup");

    // cleanup
    audio.wait();
    return Ok(());
}
