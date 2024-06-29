use rusty_ytdl::*;
use std::io::{stdin, stdout, Write};

fn input() -> String {
    let mut buffer = String::new();
    stdin().read_line(&mut buffer).unwrap();
    println!("path {:#?}", buffer);
    
    if buffer.chars().last() == Some('\n') {
        buffer.pop();
    }
    if buffer.chars().last() == Some('\r') {
        buffer.pop();
    }
    return buffer
}

#[tokio::main]
async fn main() {
    println!("Paste youtube link you want to download");
    let url = input();

    print!("Enter file in which to save the video: ");
    stdout().flush().unwrap();
    let output_path = input();
    
    let video_options = VideoOptions {
        quality: VideoQuality::Highest,
        filter: VideoSearchOptions::VideoAudio,
        ..Default::default()
    };

    let video = Video::new_with_options(&url, video_options).unwrap();

    // Or direct download to path
    let path = std::path::Path::new(&output_path);

    println!("Donwloading...");
    println!("path {:#?}", path);
    video.download(path).await.unwrap();
    println!("✅ Successfully download youtube video {}", url);
}