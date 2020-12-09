
extern crate google_youtube3 as youtube3;
use std::default::Default;
use youtube3::YouTube;
use youtube3::Video;
use youtube3::VideoSnippet;
use youtube3::VideoStatus;

use std::path::Path;
use hyper::net::HttpsConnector;
use hyper_native_tls::NativeTlsClient;
extern crate json;
use rand::prelude::*;
use std::fs;
use yup_oauth2::{
    read_application_secret, ApplicationSecret, Authenticator, DefaultAuthenticatorDelegate,
    DiskTokenStorage, FlowType,
};

const CLIENT_SECRET_FILE: &'static str = "./secret.json";

fn read_client_secret(file: String) -> ApplicationSecret {
    read_application_secret(Path::new(&file)).unwrap()
}

pub fn get_videos() -> std::vec::Vec<std::string::String>  {
    let secret = read_client_secret(CLIENT_SECRET_FILE.to_string());
    let client =
        hyper::Client::with_connector(HttpsConnector::new(NativeTlsClient::new().unwrap()));
    let authenticator = Authenticator::new(
        &secret,
        DefaultAuthenticatorDelegate,
        client,
        DiskTokenStorage::new(&"token_store.json".to_string()).unwrap(),
        Some(FlowType::InstalledInteractive),
    );

let hub = YouTube::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), authenticator);

let result = hub.videos().list("snippet")
             .video_category_id("10")
             .region_code("US")
             .max_results(40)
             .chart("mostPopular")
             .doit();

let mut rng = rand::thread_rng();
let mut vec = Vec::new();
match result {
    Err(e) => println!("error: {}", e),
    Ok(res) => {
        let videos = res.1.items.unwrap();

        let index0 = rng.gen_range(0, 40);
        vec.push(videos[index0].id.as_ref().unwrap().clone());
        vec.push(videos[index0].snippet.as_ref().unwrap().title.as_ref().unwrap().clone());

        let index1 = rng.gen_range(0, 40);
        vec.push(videos[index1].id.as_ref().unwrap().clone());
        vec.push(videos[index1].snippet.as_ref().unwrap().title.as_ref().unwrap().clone());

        vec.extend(videos[index0].snippet.as_ref().unwrap().tags.as_ref().unwrap().to_vec());
       vec.extend(videos[index1].snippet.as_ref().unwrap().tags.as_ref().unwrap().to_vec());
}
}

    return vec;



}

pub fn upload(name: String, mut tags: Vec<String>){

   let secret = read_client_secret(CLIENT_SECRET_FILE.to_string());
    let client =
        hyper::Client::with_connector(HttpsConnector::new(NativeTlsClient::new().unwrap()));
    let authenticator = Authenticator::new(
        &secret,
        DefaultAuthenticatorDelegate,
        client,
        DiskTokenStorage::new(&"token_store.json".to_string()).unwrap(),
        Some(FlowType::InstalledInteractive),
    );

let hub = YouTube::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), authenticator);



    let mut snip = VideoSnippet::default();
    let mut status = VideoStatus::default();

    status.privacy_status = Some(String::from("public"));

    snip.title = Some(format!("{} (Mashup)", name));
    snip.category_id = Some(String::from("10"));
   let mut tagsize = tags.join("");
    while tagsize.len() > 400 {
        tags.pop();
        tagsize = tags.join("");
    }

    snip.tags =  Some(tags);

    let mut req = Video::default();
    req.status = Some(status);
    req.snippet = Some(snip);

    let result = hub.videos().insert(req)
             .notify_subscribers(true)
             .upload(fs::File::open("./finished.mp4").unwrap(), "video/mp4".parse().unwrap());

}
