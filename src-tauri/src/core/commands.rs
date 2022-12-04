// SPDX-License-Identifier: MIT OR Apache-2.0

use std::io::BufReader;
use rodio;

#[tauri::command]
/// Open the website in the default browser.
pub fn website(url: &str) {
    webbrowser::open(url).unwrap();
}

#[tauri::command]
/// Open the audio directory in the default file manager.
pub fn play_audio(sound: &str) {

    let (_stream, handle) = rodio::OutputStream::try_default().unwrap();
    let sink = rodio::Sink::try_new(&handle).unwrap();
    let path = "./assets/audio/".to_string() + sound;

    let file = std::fs::File::open(path).unwrap();

    sink.append(rodio::Decoder::new(BufReader::new(file)).unwrap());

    sink.sleep_until_end();


}
