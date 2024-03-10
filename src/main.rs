ue std::{fs::File, io::Write, path::Path, process::exit, sync::Arc, thread};

use futures::TryFutureExt;
use serde::{Serialize,Deserialize};
use reqwest;
use serde_json::Value;
use tokio;


#[derive(Debug,Serialize,Deserialize,Clone)]
struct urls {
    path : String
}

#[tokio::main]
async fn main() {
    println!("Hello, world!");
    let mut req = reqwest::get("https://wallhaven.cc/api/v1/search?q=anime").await.unwrap().text().await;
    println!("{:#?}",req);
    let mut _file_url = File::create("urls.txt").unwrap();
    let req = match req {
        Ok(value) => value,
        Err(e) => format!("Error {e}")
    };
    //let man : urls  = serde_json::from_str(&req).unwrap();
    //dbg!(man);
    

    _file_url.write_all(req.as_bytes());
    _file_url.flush().unwrap();

    let mann = extract_url(&req);
    download_images(mann).await;
}


fn extract_url(url : &String) -> Vec<urls>
{
    let v : Value = serde_json::from_str(url).unwrap();
    let data = &v["data"];
    println!("{:?}",v);
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


async fn download_images(arr : Vec<urls>) {
    //let (arr1,arr2) = Arc::clone(&arr).split_at(arr.len()/2);
    let down_dir = "Downloads";
    if !dir_exists(down_dir) {
        std::fs::create_dir(down_dir);
    }
    let handle1 = tokio::spawn(async move {
        for i in arr {
            download_image(i).await;
        }
    });
    handle1.await.unwrap();
}


async fn download_image(url : urls){
    let img_req = reqwest::get(&url.path).await.unwrap();
    let img_byts = img_req.bytes().await.unwrap();
    let filename = url.path.split('/').last().unwrap_or("unknown.jpg");
    // let mut file = File::create(filename).unwrap();
    let down_path = format!("Downloads/{}",filename);
    let path = Path::new(&down_path);
    
    if let Err(e) = tokio::fs::write(&path, img_byts).await {
        eprintln!("Failed to save img {} : {} ",filename,e);
        exit(-1);
    }
    else{
    //file.write_all(&img_byts).unwrap();
        println!("Saved Image : {} ",down_path);
    }
}



fn dir_exists(val : &str)->bool{
    if let Ok(meta) = std::fs::metadata(val){
        true
    }
    else {
        false
    }
}
