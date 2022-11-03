// Rust media player
// Plays an audio file using the rodio crate

use rodio::{Decoder, OutputStream, Sink, Source};
use std::fs::File;
use std::io::BufReader;

fn main() {
    // Variables
    let volume = 0.1;
    let stopped = false;

    // Create an output stream
    println!("Creating output stream...");
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let sink = Sink::try_new(&stream_handle).unwrap();

    // Find all files and count them idk how this works
    let sounds = std::fs::read_dir("music")
        .unwrap()
        .map(|res| res.map(|e| e.path()))
        .collect::<Result<Vec<_>, std::io::Error>>()
        .unwrap();
    println!("Files found ({}): \n{:?}", sounds.len(), sounds);

    // Loop through all files
    while stopped == false {
        // Generate a random number between 0 and the number of files
        let random = rand::random::<usize>() % sounds.len();

        // Get the file path
        let file_path = sounds[random].to_str().unwrap();

        // Open the file
        println!("Opening file: {}", file_path);
        let file = File::open(file_path).unwrap();

        println!("\nPLAYING SONG {}: {}", random, file_path);
        println!("Volume: {}%", volume * 100.0);

        // Decode the file
        let source = Decoder::new(BufReader::new(file))
            .unwrap()
            .amplify(volume);

        // Play the file
        sink.append(source);
        
        // Wait for it to finish
        sink.sleep_until_end();
    }
}
