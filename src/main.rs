use reqwest::blocking::Response;
use serde::Deserialize;
use std::{fs, io::Write};
use bytes::Bytes;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct PictureList {
    err_code: u64,
    err_msg: String,
    dirs: Vec<Directory>
}

#[derive(Deserialize)]
struct Directory {
    name: String,
    files: Vec<String>
}

const API_URL: &str = "http://192.168.0.1/v1/photos";

fn main() {
    // Not my finest code so far, but it works :D
    // TODO: Maybe make a function or two...

    println!("Getting full picture list...");
    match reqwest::blocking::get(API_URL) {
        Ok(response) => {
            handle_picture_list_response(response);
        }
        Err(e) => {
            eprintln!("Could not load picture list. Are you connected to the camera? Is there an SD card in the camera?");
            eprintln!("{e}");
        }
    }
}

fn handle_picture_list_response(response: Response) {
    match response.json::<PictureList>() {
        Ok(picture_list) => {
            match picture_list.err_code {
                200 => {
                    let code = picture_list.err_code;
                    let msg = &picture_list.err_msg;

                    println!("{code} {msg}");

                    fetch_all_pictures(picture_list);
                },
                code => {
                    eprintln!("Could not list pictures (code {code}). Is there an SD card in the camera?");
                }
            }
        },
        Err(e) => {
            eprintln!("Could not parse response body. Check it with `curl http://192.168.0.1/v1/photos`");
            eprintln!("{e}");
        }
    }
}

fn fetch_all_pictures(picture_list: PictureList) {
    match fs::create_dir_all("./grit") {
        Ok(_) => {
            for directory in picture_list.dirs {
                let local_image_directory = format!("./grit/{}", directory.name);

                match fs::create_dir_all(&local_image_directory) {
                    Ok(_) => {
                        for image_name in directory.files {
                            let local_path = format!("{local_image_directory}/{image_name}");

                            match fs::exists(&local_path) {
                                Ok(exists) => {
                                    if !exists {
                                        pull_image(&directory.name, &image_name, &local_path);
                                    }
                                    else {
                                        println!("File already exists: {local_path}");
                                    }
                                },
                                Err(e) => {
                                    eprintln!("Can't check if file exists: {local_path}");
                                    eprintln!("{e}");
                                }
                            }
                        }
                    },
                    Err(e) => {
                        eprintln!("Can't create image directory: {local_image_directory}");
                        eprintln!("{e}");
                    }
                }
            }
        },
        Err(e) => {
            eprintln!("Could not create target directory ./grit");
            eprintln!("{e}");
        }
    }
}

fn pull_image(dir_name: &String, image_name: &String, local_path: &str) {
    println!("Pulling {dir_name}/{image_name}...");
    let url =  format!("{API_URL}/{dir_name}/{image_name}");
    match reqwest::blocking::get(url) {
        Ok(response) => {
            match response.bytes() {
                Ok(bytes) => {
                    fully_write_or_delete(bytes, local_path);
                },
                Err(e) => {
                    eprintln!("Cannot get bytes for picture: {dir_name}/{image_name}");
                    eprintln!("{e}");
                }
            }
        },
        Err(e) => {
            eprintln!("Cannot download picture from server: {dir_name}/{image_name}");
            eprintln!("{e}");
        }
    }
}

fn fully_write_or_delete(bytes: Bytes, local_path: &str) {
    match fs::File::create(&local_path) {
        Ok(mut file) => {
            match file.write(&bytes) {
                Ok(bytes_written) => {
                    if bytes.len() == bytes_written {
                        println!("File transferred: {local_path}");
                    }
                    else {
                        eprintln!("Could not fully write file: {local_path}.");
                        eprintln!("  Removing now.");
                        match fs::remove_file(&local_path) {
                            Ok(_) => {
                                eprintln!("  File deleted.");
                            },
                            Err(e) => {
                                eprintln!("  Could not delete file.");
                                eprintln!("{e}");
                            }
                        }
                    }
                },
                Err(e) => {
                    eprintln!("Could not write file: {local_path}");
                    eprintln!("{e}");
                }
            }
        },
        Err(e) => {
            eprintln!("Can't open file for writing: {local_path}");
            eprintln!("{e}");
        }
    }
}