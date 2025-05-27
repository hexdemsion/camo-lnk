// cargo run --bin watcher -- .\mapping.db.enc


use std::fs;
use std::env;
use std::io::Read;
use rand::seq::IndexedRandom;
use sha256::digest;
use std::path::Path;
use std::fs::File;
use std::io::{BufRead, BufReader};
use serde_json::{Result, Value};
// use lnk::ShellLink;
use mslnk::ShellLink;
use rusqlite::{Connection, params};
use std::path::PathBuf;
use std::process::Command;
use std::process::Stdio;
use std::{thread, time};

use std::fs::OpenOptions;




mod common;
use crate::common::*;









fn main() {

    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: watcher.exe .\\path\\to\\mapping.db.enc");
        let ten_millis = time::Duration::from_millis(5000);
        thread::sleep(ten_millis);
    
        return;
    }
    let encdb_path = &args[1];
    
    if !Path::new(&encdb_path).exists() {
        panic!("DB path not found");
    }else{
        decrypt_db();
    }




    // inner block scoping to avoid still-open .\mapping.db FILE_HANDLE
    {
        let db_path = format!("{}\\{}", std::env::var("USERPROFILE").ok().unwrap(), "raw_mapping_db.dll");
        let conn = Connection::open(db_path).unwrap();


        let mut stmt = conn.prepare("SELECT * FROM mapping").unwrap();

        let rows = stmt.query_map(params![], |row| {

            let hash: String = row.get(0)?;
            let original_path: String = row.get(1)?;
            let lnk_path: String = row.get(2)?;
            let hidden_path: String = row.get(3)?;
            let sre: String = row.get(4)?;
            Ok(( hash, original_path, lnk_path, hidden_path, sre ))
        }).unwrap();

        let mut hf_path_list: Vec<String> = vec![  String::from(r"test-file-demo.txt")   ];

        for row in rows {
            let (hash, original_path, lnk_path, hidden_path, sre) = row.unwrap();
            // println!(" Hash: {}, Original: {}, LNK: {}, Hidden: {}, SRE: {}", hash, original_path, lnk_path, hidden_path, sre);

            let file_path = Path::new(hidden_path.as_str());

            // println!("{}", file_path.display() );

            let fpt = file_path.to_string_lossy().to_string();
            let fpn = file_path.file_name().unwrap().to_string_lossy().to_string();
            hf_path_list.push(fpn );
        }

        for vv in &hf_path_list {
            println!("vv -> {}", vv);
        }



        let pipe_path = r"\\.\pipe\Rusty";

        println!("Waiting for pipe...");
        let pipe = loop {
            match OpenOptions::new().read(true).open(pipe_path) {
                Ok(p) => break p,
                Err(_) => {
                    thread::sleep(std::time::Duration::from_millis(100));
                }
            }
        };
    
        println!("Connected. Reading stream...");
        let reader = BufReader::new(pipe);
    
        for line in reader.lines() {
            match line {
                Ok(text) => {
                    // println!("Received: {}", &text);

                    let path = Path::new(&text);

                    if let Some(fnx) = path.file_name() {
                        let file_name = fnx.to_str().unwrap().replace('\0', "");
                        if file_name.len() < 1 {
                            continue;
                        }
                        // println!("detected file name: {} | len: {}", file_name, file_name.len());

                        // for (i, b) in file_name.bytes().enumerate() {
                        //     println!("byte[{}] = {:02X} ({:?})", i, b, b as char);
                        // }
                        // break;
                        
                        

                        // let x = String::from("Desktop");
                        // println!("matcher: {} | len: {}", x, x.len());



                        // if file_name.contains("Desktop") {
                        //     panic!("desktop accessedddd !!!!!!!!!!!!!!!!!!!!!!!!1");
                        // }


                        let found = hf_path_list.iter().any(|x| x.contains(&file_name)   );
                        if found {
                            println!("camo file accessed !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!");
                            println!("Received: {}", &text);
                            println!("detected file name: {}", &file_name);
                            
                            // panic!("founded !!!!!!!!!!!!!!!!!!!!!!!!1");
                        }

                        // for vv in &hf_path_list {
                        //     if vv.contains(&file_name) {
                        //         panic!("founded !!!!!!!!!!!!!!!!!!!!!!!!1");
                        //     }
                        // }

                    
                    } 
                    
                }
                Err(e) => {
                    println!("Read error: {:?}", e);
                    break;
                }
            }
        }

        

    
    
    }







    encrypt_db();

    let ten_millis = time::Duration::from_millis(3000);
    thread::sleep(ten_millis);


}
