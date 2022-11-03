// Rust media player
// Plays an audio file using the rodio crate

use soloud::*;
use std::io;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Variables
    let volume = 0.1;
    let stopped = false;

    // Create an output stream
    println!("Creating output stream...");
    let mut sl = Soloud::default()?;
    let mut wav = audio::Wav::default();

    // Find all files and count them idk how this works
    let sounds = std::fs::read_dir("music")
        .unwrap()
        .map(|res| res.map(|e| e.path()))
        .collect::<Result<Vec<_>, std::io::Error>>()
        .unwrap();
    println!("Files found ({}): \n{:?}\n", sounds.len(), sounds);

    // Loop through all files
    while stopped == false {
        // Generate a random number between 0 and the number of files
        let random = rand::random::<usize>() % sounds.len();

        // Get the file path
        let file_path = sounds[random].to_str().unwrap();

        // Open the file
        println!("Opening file: {}", file_path);

        // Play the file
        wav.load(&std::path::Path::new(file_path))?;
        let vh = sl.play(&wav);
        sl.set_volume(vh, volume);

        // Print status
        println!("\nPLAYING SONG {}: {}", random, file_path);
        println!("Volume: {}%", volume * 100.0);

        // Wait for the song to finish
        while sl.voice_count() > 0 {
            std::thread::sleep(std::time::Duration::from_millis(100));
        }
    }
    Ok(())
}
