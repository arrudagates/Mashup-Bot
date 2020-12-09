use fantoccini::{Locator};
use std::time::Duration;
use tokio::time::delay_for;
use reqwest;

extern crate hyper;
extern crate hyper_rustls;

extern crate google_youtube3 as youtube3;

#[tokio::main]
pub async fn make_mashup(videos: Vec<String>) -> Result<std::string::String, Box<dyn std::error::Error>> {


let mut caps = serde_json::map::Map::new();
let opts = serde_json::json!({ "args": ["--headless"] });
caps.insert("moz:firefoxOptions".to_string(), opts.clone());

let mut client = fantoccini::Client::with_capabilities("http://0.0.0.0:4444", caps).await?;

    client.goto("https://rave.dj/mix").await?;
   let video0 = format!("https://youtube.com/watch?v={}", &videos[0]);
    client.find(Locator::Css(".search-input")).await?
    .send_keys(&video0).await?;

   let search_button = "html > body > div > div > div:nth-of-type(2) > div:nth-of-type(2) > div:nth-of-type(3) > div > div:nth-of-type(1) > div > div:nth-of-type(1) > div:nth-of-type(1) > div:nth-of-type(1) > button";
    let element = client.find(Locator::Css(search_button)).await?;
    element.click().await?;
   let video1 = format!("https://youtube.com/watch?v={}", &videos[2]);
   client.find(Locator::Css(".search-input")).await?
    .send_keys(&video1).await?;

   let search_button = "html > body > div > div > div:nth-of-type(2) > div:nth-of-type(2) > div:nth-of-type(3) > div > div:nth-of-type(1) > div > div:nth-of-type(1) > div:nth-of-type(1) > div:nth-of-type(1) > button";
    let element = client.find(Locator::Css(search_button)).await?;
    element.click().await?;

   delay_for(Duration::from_millis(7000)).await;

    client.wait_for_find(Locator::Css(".mix-button")).await?.click().await?;

       let url = client.current_url().await?;
    client.wait_for_navigation(Some(url)).await?;
    let url = client.current_url().await?;

    //delay_for(Duration::from_secs(300)).await;

    client.goto(url.as_ref()).await?;
      let name = client.wait_for_find(Locator::Css(".player-overlay-info-artist > p:nth-child(1)")).await?.text().await?;
    let result = client.wait_for_find(Locator::Css("#ForegroundPlayer_html5_api")).await?.attr("src").await?;

 let resp = reqwest::get(&result.unwrap()).await?;
     let bytes = resp.bytes().await?;
     let mut out = tokio::fs::File::create("./result.mp4").await?;
    tokio::io::copy(&mut &*bytes, &mut out).await?;

   let thumb0 = format!("https://i.ytimg.com/vi/{}/maxresdefault.jpg", &videos[0]);
  let resp0 = reqwest::get(&thumb0).await?;
     let bytes0 = resp0.bytes().await?;
     let mut out0 = tokio::fs::File::create("./0.jpg").await?;
     tokio::io::copy(&mut &*bytes0, &mut out0).await?;

   let thumb1 = format!("https://i.ytimg.com/vi/{}/maxresdefault.jpg", &videos[2]);
  let resp1 = reqwest::get(&thumb1).await?;
     let bytes1 = resp1.bytes().await?;
     let mut out1 = tokio::fs::File::create("./1.jpg").await?;
     tokio::io::copy(&mut &*bytes1, &mut out1).await?;

    client.close().await.expect("couldn't close client");
    Ok(name.clone())
}
