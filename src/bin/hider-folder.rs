// cargo run --release --bin hider-folder -- "C:\Users\IEUser\Desktop\dummyfile\"

use std::fs;
use std::env;
use std::io::Read;
use std::io::Write;
use rand::seq::IndexedRandom;
use sha256::digest;
use std::path::Path;
use std::fs::File;
use std::io::{BufRead, BufReader, Cursor};
use serde_json::{Result, Value};
// use lnk::ShellLink;
use mslnk::ShellLink;
use mslnk::ShellLinkHeader;
use rusqlite::{Connection, params};
use std::path::PathBuf;

use aes_gcm::{
    aead::{Aead, AeadCore, KeyInit, OsRng},
    Aes256Gcm, Nonce, Key // Or `Aes128Gcm`
};

use obfstr::obfstr;

mod common;
use crate::common::*;

use std::{thread, time};



const WHITELISTED_EXTENSIONS: &str = include_str!("./whitelisted-extension.txt");
const WHITELISTED_DIRPATH: &str = include_str!("./whitelisted-dirpath.txt");
const WHITELISTED_SRE: &str = include_str!("./whitelisted-SRE.json");




fn main() {

    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: hider-folder.exe <directory_you_want_the_content_to_be_comouflaged>");
        return;
    }
    let dir_target = &args[1];

    let absolute_path: PathBuf = fs::canonicalize(dir_target).expect("Failed to get absolute path");
    println!("Absolute path: {}", absolute_path.display());





    let cursor = Cursor::new(WHITELISTED_EXTENSIONS);
    let reader = BufReader::new(cursor);

    let extensions: Vec<String> = reader
        .lines()
        .filter_map(|line| line.ok())
        .collect();

    println!("Ekstensi yang diambil: {:?}", extensions);




    

    let cursor = Cursor::new(WHITELISTED_DIRPATH);
    let reader = BufReader::new(cursor);

    let dirpath: Vec<String> = reader
        .lines()
        .filter_map(|line| line.ok())
        .collect();

    println!("Dirpath yang diambil: {:?}", dirpath);



    
    let srelist: serde_json::Value = serde_json::from_str(WHITELISTED_SRE.replace("\\", "\\\\").as_str()).unwrap();
    println!("SRE yang diambil: {:?}", srelist);




    let target = format!("{}\\{}", std::env::var("USERPROFILE").ok().unwrap(), "Desktop\\camo\\target\\debug\\launcher.exe");
    let target = target.as_str();



    let encpath = format!("{}\\{}", std::env::var("USERPROFILE").ok().unwrap(), "enc_mapping_db.dll");
    if !Path::new(&encpath).exists() {
        init_db();
    }else{
        decrypt_db();
    }

    let db_path = format!("{}\\{}", std::env::var("USERPROFILE").ok().unwrap(), "raw_mapping_db.dll");
    let mut conn = Connection::open(db_path).unwrap();
    let tx = conn.transaction().unwrap();
    

    let start_time = std::time::Instant::now();
    let mut fcount = 0;

    for entry in fs::read_dir(absolute_path).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();

        // let metadata = fs::metadata(&path).unwrap();
        // let file_size = metadata.len(); // ukuran file dalam byte


        if path.is_file() {
            if let Some(file_name) = path.file_name() {

                println!("{} == {} -> {}", "file_size", path.display(),  file_name.to_string_lossy());

                

                
                let extension = path
                    .extension()
                    .and_then(|ext| ext.to_str())
                    .unwrap_or(""); 
                let ext_string = format!(".{}", extension.to_string()  ) ;
                println!("Ekstensi file nya: {}", ext_string);


                let fnhash = digest(path.to_str().unwrap() );
                println!("FNhash nya: {}", fnhash);
            
            
                let hidden_path = format!("{}{}{}", dirpath.choose(&mut rand::rng()).unwrap(), fnhash, extensions.choose(&mut rand::rng()).unwrap()  );
                println!("Final HiddenPath nya: {}", hidden_path);
            



                let mut pickedsre = String::new();
                for item in srelist.as_array().unwrap() {
                    if item["ext"] == ext_string {
                        let sre_array = item["sre"].as_array().unwrap();
                        
                        let mut rng = rand::rng();
                        if let Some(p) = sre_array.choose(&mut rng) {
                            let picked_str = p.as_str().unwrap().to_string();
                            pickedsre = picked_str.clone();
                            println!("picked SRE: {}", pickedsre);
                        }
                    }
                }


                
                

                            
                let fnlnk = path.file_name().unwrap().to_str().unwrap();
                let lnk = format!("{}\\{}.lnk", path.parent().unwrap().display(), fnlnk );
                println!("Final LNK path-> {}", lnk);
                    


                let mut sl = ShellLink::new(target).unwrap();
                
                sl.set_arguments(Some(fnhash.to_string() ));
                sl.set_name(Some(String::from("LNK file to launch from secure dir") ));
                sl.create_lnk(lnk.clone()).unwrap();




                            


                // conn.execute(
                //     "INSERT INTO mapping (hash, original_path, lnk_path, hidden_path, sre) VALUES (?1, ?2, ?3, ?4, ?5)",
                //     params![fnhash, path.to_str(), lnk.clone(), hidden_path, pickedsre],
                // ).unwrap();

                tx.execute(
                    "INSERT INTO mapping (hash, original_path, lnk_path, hidden_path, sre) VALUES (?1, ?2, ?3, ?4, ?5)",
                    params![fnhash, path.to_str(), lnk.clone(), hidden_path, pickedsre],
                ).unwrap();
            
                



                // fs::copy(&path, &hidden_path).unwrap();
                fs::rename(&path, &hidden_path).unwrap();



                // fs::remove_file(&path ).unwrap();

                fcount += 1;
            }
        }
    }

    tx.commit().unwrap();
    conn.close();
    encrypt_db();
    

    println!("done processing {} total file", fcount );
    let duration = start_time.elapsed(); // Hitung durasi
    println!("Program selesai dalam {:.2?} detik", duration);

}
