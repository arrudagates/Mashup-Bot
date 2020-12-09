use std::process::{Command, Stdio};
mod youtube;
mod rave;


fn main() {

   Command::new("killall")
       .arg("geckodriver")
       .output()
       .expect("failed to kill geckodriver on killall");

    let mut gecko = Command::new("geckodriver")
        .args(&["--port", "4444"])
       .stderr(Stdio::null())
        .stdout(Stdio::null())
        .spawn()
        .expect("failed to run geckodriver");

   let videos = youtube::get_videos();
    //println!("{:?}", videos);
    println!("\nVideos Fecthed: \n{}\n{}", &videos[1], &videos[3]);

    println!("\nMaking Mashup...");
    let name = rave::make_mashup(videos.clone());
    println!("Mashup Done");

    println!("\nFinal Name: {} (Mashup)", name.as_ref().unwrap());

    gecko.kill().expect("Failed to kill geckodriver");

  Command::new("montage")
       .args(&["[0-1].jpg", "-tile", "1x2", "-geometry", "+0+0", "out.png"])
       .output()
       .expect("fail");
  Command::new("ffmpeg")
           .args(&["-y", "-i", "result.mp4", "-f", "image2", "-loop", "1", "-i", "out.png", "-map", "1:v:0", "-map", "0:a:0", "-r", "15", "-s", "640x480", "-c:v", "libx264", "-crf", "18", "-tune", "stillimage", "-preset", "medium", "-shortest", "finished.mp4"])
           .output()
        .expect("failed to execute process");
    let mut tags = videos;
    tags.remove(0);
    tags.remove(1);
    println!("\nUploading Video...");
    youtube::upload(name.unwrap(), tags);
    println!("Video Uploaded!");
}
