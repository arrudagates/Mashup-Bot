use std::process::Command;
mod youtube;
mod rave;


fn main() {
//    youtube::upload();
   let videos = youtube::get_videos();
    println!("{:?}", videos);
   let name = rave::make_mashup(videos.clone());
   //println!("{}", name.unwrap());
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
    tags.remove(0);
    youtube::upload(name.unwrap(), tags);
}
