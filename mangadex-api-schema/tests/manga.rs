use std::{
    fs::{create_dir_all, File},
    io::{Read, Write},
};

use mangadex_api_schema_rust::v5::MangaData;
use reqwest::Client;
use url::Url;

#[tokio::test]
async fn test_manga_serialization() {
    let client = Client::new();
    let res = client
        .get(
            Url::parse_with_params(
                "https://api.mangadex.org/manga/aa6c76f7-5f5f-46b6-a800-911145f81b9b",
                &[("includes[]", "creator")],
            )
            .unwrap(),
        )
        .send()
        .await
        .unwrap();
    let response_text = res.text().await.unwrap();
    let mangadata: MangaData = serde_json::from_str(response_text.as_str()).unwrap();
    let json_mangadata = serde_json::to_string(&mangadata).unwrap();
    create_dir_all("test-output/manga").unwrap();
    let mut file1: File = File::create("test-output/manga/1.json").unwrap();
    let mut file2: File = File::create("test-output/manga/2.json").unwrap();
    file1.write_all(json_mangadata.as_bytes()).unwrap();
    file2.write_all(response_text.as_bytes()).unwrap();
}

#[tokio::test]
async fn compare_1_2() {
    create_dir_all("test-output/manga").unwrap();
    let mut file1: File = File::open("test-output/manga/1.json").unwrap();
    let mut file2: File = File::open("test-output/manga/2.json").unwrap();
    let mut file1_data = String::default();
    file1.read_to_string(&mut file1_data).unwrap();
    let mut file2_data = String::default();
    file2.read_to_string(&mut file2_data).unwrap();
    assert_eq!(file1_data, file2_data);
}

#[tokio::test]
async fn test_des_and_ser() {
    create_dir_all("test-output/manga").unwrap();
    let mut file1: File = File::open("test-output/manga/1.json").unwrap();
    let mut file1_data = String::default();
    file1.read_to_string(&mut file1_data).unwrap();
    let mangadata: MangaData = serde_json::from_str(file1_data.as_str()).unwrap();
    assert_eq!(serde_json::to_string(&mangadata).unwrap(), file1_data);
}
