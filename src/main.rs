use std::{fs::File, io::Write, path::Path, process::exit, sync::Arc};
use serde::{Serialize,Deserialize};
use reqwest;
use serde_json::Value;
use tokio;

#[allow(unused)]
#[allow(non_camel_case_types)]

#[derive(Debug,Serialize,Deserialize,Clone)]
struct urls {
    path : String
}
#[allow(unused_mut)]
#[tokio::main]
async fn main() {
    println!("Hello, world!");
    let mut req = reqwest::get("https://wallhaven.cc/api/v1/search?q=anime").await.unwrap().text().await;
    let mut _file_url = File::create("urls.json").unwrap();
    let req = match req {
        Ok(value) => value,
        Err(e) => format!("Error {e}")
    };
    _file_url.write_all(req.as_bytes()).expect("error on write");
    _file_url.flush().unwrap();

    let mann = extract_url(&req);
    let _data_to_proc = Arc::new(mann);
    download_images(_data_to_proc).await;
}


fn extract_url(url : &String) -> Vec<urls>
{
    let v : Value = serde_json::from_str(url).unwrap();
    let data = &v["data"];
    let mut pat : Vec<urls> = Vec::new();
    if let Some(arr) = data.as_array(){
        for obj in arr {
            if let Some(path) = obj["path"].as_str(){
                pat.push(urls{path : format!("{}",path)});
            }
        }
    
    }
    pat
}

async fn download_images(arr : Arc<Vec<urls>>) {
    let _data_to_proc = Arc::clone(&arr);
    let down_dir = "Downloads";
    if !dir_exists(down_dir) {
        std::fs::create_dir(down_dir).expect("Error at the DOWNLOAD folder creation");
    }
    let handle1 = tokio::spawn(async move {
        let d1 = Arc::clone(&_data_to_proc);
        let d1 = d1.split_at(d1.len()/2);
        for i in d1.0.iter() {
            download_image(1,i).await;
        }
    });

    let handle2 = tokio::spawn(async move{
        let d1 = Arc::clone(&arr);
        let d1 = d1.split_at(d1.len()/2);
        for i in d1.1.iter() {
            download_image(2,i).await;
        }
    });

    let _ = tokio::join!(handle1,handle2);
}


async fn download_image(thrid : usize, url : &urls){
    let img_req = reqwest::get(&url.path).await.unwrap();
    let img_byts = img_req.bytes().await.unwrap();
    let filename = url.path.split('/').last().unwrap_or("unknown.jpg");
    let down_path = format!("Downloads/{}",filename);
    let path = Path::new(&down_path);
    
    if let Err(e) = tokio::fs::write(&path, img_byts).await {
        eprintln!("Failed to save img {} : {} ",filename,e);
        exit(-1);
    }
    else{
        println!("Thread {thrid} -> Saved Image : {} ",down_path);
    }
}



fn dir_exists(val : &str)->bool{
    if let Ok(_meta) = std::fs::metadata(val){
        true
    }
    else {
        false
    }
}
