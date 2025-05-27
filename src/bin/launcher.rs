// cargo run --bin launcher -- eb863975461f8f1888984d603121f73bad89ea2ecc08ae8f9c2350e3d00748c2
// cargo run --bin launcher -- 1dc64e74337f2bfab49d4c9ef832923beed0d6310059ec9a9e823744bfc7adba


use std::fs;
use std::env;
use std::io::Read;
use rand::seq::IndexedRandom;
use sha256::digest;
use std::path::Path;
use std::fs::File;
use std::io::{BufRead, BufReader, Cursor};
use serde_json::{Result, Value};
// use lnk::ShellLink;
use mslnk::ShellLink;
use rusqlite::{Connection, params};
use std::path::PathBuf;
use std::process::Command;
use std::process::Stdio;
use std::{thread, time};



mod common;
use crate::common::*;







fn main() {

    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: launcher.exe <fnhash>");
        let ten_millis = time::Duration::from_millis(5000);
        thread::sleep(ten_millis);
    
        return;
    }
    let fnhash = &args[1];




    let encdb_path = format!("{}\\{}", std::env::var("USERPROFILE").ok().unwrap(), "enc_mapping_db.dll");
    if !Path::new(&encdb_path).exists() {
        init_db();
    }else{
        decrypt_db();
    }




    // inner block scoping to avoid still-open .\mapping.db FILE_HANDLE
    {
        let db_path = format!("{}\\{}", std::env::var("USERPROFILE").ok().unwrap(), "raw_mapping_db.dll");
        let conn = Connection::open(db_path).unwrap();


        let mut stmt = conn.prepare("SELECT * FROM mapping WHERE hash=?1").unwrap();

        let rows = stmt.query_map(params![fnhash], |row| {

            let hash: String = row.get(0)?;
            let original_path: String = row.get(1)?;
            let lnk_path: String = row.get(2)?;
            let hidden_path: String = row.get(3)?;
            let sre: String = row.get(4)?;
            Ok(( hash, original_path, lnk_path, hidden_path, sre ))
        }).unwrap();


        for row in rows {
            let (hash, original_path, lnk_path, hidden_path, sre) = row.unwrap();
            println!(" Hash: {}, Original: {}, LNK: {}, Hidden: {}, SRE: {}", hash, original_path, lnk_path, hidden_path, sre);


            // let file_path = Path::new(hidden_path.as_str());
            // let app_path = Path::new(sre.as_str());

            // println!("file_path: {:?}", file_path );
            // println!("app_path: {:?}", app_path );
                        
            // Command::new("cmd")
            //     .args(&["/C", "echo", "hello" ])
            //     .output()
            //     .expect("failed to execute process");
            
            
            Command::new(sre.clone())
                .args(&[hidden_path.clone()])
                .stdin(Stdio::null())   
                .stdout(Stdio::null())  
                .stderr(Stdio::null())  
                .spawn()                
                .expect("failed to execute process");

            println!("Process started in background!");
            println!("{} {}", sre, hidden_path);
        }
    }







    encrypt_db();

    let ten_millis = time::Duration::from_millis(3000);
    thread::sleep(ten_millis);


}
