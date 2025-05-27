// c:/ProgramX/hider.exe c:/tmp/original.docx 
// read filename, SHA256-hash it (agjhbadwad876dad)
// get file extension, determine SRE (winword.exe)
// randomly choose the whitelisted extension (.exe/.dll/.sys/.dat/.ini)
// randomly choose the whitelisted dirpath 
// C:\Windows\WinSxS\
// C:\Windows\Fonts\
// C:\Windows\SoftwareDistribution\
// C:\Windows\Logs\
// move c:/tmp/original.docx  to C:\Windows\Fonts\agjhbadwad876dad.dll

// create sqlite c:/ProgramX/mapping.db if not exists.
// create sqlite table mapping to relate c:/tmp/original.docx with C:\Windows\Fonts\agjhbadwad876dad.dll by their file hash name
// save the database


// generate shell .LNK file c:/tmp/original.docx.lnk
// .LNK shell command is "c:/ProgramX/launcher.exe  C:\Windows\Fonts\agjhbadwad876dad.dll"


// cargo run --bin hider -- ./Cargo.toml


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
    // println!("Hello, hider!");

    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: hider.exe <original_file_path>");
        let ten_millis = time::Duration::from_millis(5000);
        thread::sleep(ten_millis);

        return;
    }
    let original_path = &args[1];
    let file_data = fs::read(original_path).unwrap();


    let absolute_path: PathBuf = fs::canonicalize(original_path).expect("Failed to get absolute path");
    println!("Absolute path: {}", absolute_path.display());
    println!("Absolute dirpath: {}", absolute_path.parent().unwrap().display()  );









    
    let filename = original_path;
    let path = Path::new(filename);

    let extension = path
        .extension()
        .and_then(|ext| ext.to_str())
        .unwrap_or(""); 

    let ext_string = format!(".{}", extension.to_string()  ) ;

    println!("Ekstensi file nya: {}", ext_string);










    // let whext = format!("{}\\{}", std::env::var("USERPROFILE").ok().unwrap(), "Desktop\\camo\\src\\bin\\whitelisted-extension.txt");
    // let path = Path::new(&whext );
    
    // let file = File::open(&path).unwrap();
    // let reader = BufReader::new(file);
    
    // let extensions: Vec<String> = reader
    //     .lines()
    //     .filter_map(|line| line.ok())
    //     .collect();

    // println!("Ekstensi yang diambil: {:?}", extensions);


    let cursor = Cursor::new(WHITELISTED_EXTENSIONS);
    let reader = BufReader::new(cursor);

    let extensions: Vec<String> = reader
        .lines()
        .filter_map(|line| line.ok())
        .collect();

    println!("Ekstensi yang diambil: {:?}", extensions);













    // let whdir = format!("{}\\{}", std::env::var("USERPROFILE").ok().unwrap(), "Desktop\\camo\\src\\bin\\whitelisted-dirpath.txt");
    // let path = Path::new(&whdir );
    
    // let file = File::open(&path).unwrap();
    // let reader = BufReader::new(file);
    
    // let dirpath: Vec<String> = reader
    //     .lines()
    //     .filter_map(|line| line.ok())
    //     .collect();

    // println!("Dirpath yang diambil: {:?}", dirpath);

    let cursor = Cursor::new(WHITELISTED_DIRPATH);
    let reader = BufReader::new(cursor);

    let dirpath: Vec<String> = reader
        .lines()
        .filter_map(|line| line.ok())
        .collect();

    println!("Dirpath yang diambil: {:?}", dirpath);











    








    // let whsre = format!("{}\\{}", std::env::var("USERPROFILE").ok().unwrap(), "Desktop\\camo\\src\\bin\\whitelisted-SRE.json");

    // let mut jsonstr = String::new();
    // let fnl = File::open(&whsre).unwrap().read_to_string(&mut jsonstr);
    // let fixed_json = jsonstr.replace("\\", "\\\\");


    // let srelist: serde_json::Value = serde_json::from_str(fixed_json.as_str()).unwrap();
    // let srelist: Vec<Value> = serde_json::from_str(jsonstr.as_str()).unwrap();

    // Access parts of the data by indexing with square brackets.
    // println!("Please call {} at the number {}", v[0]["ext"], v[1]["ext"]);
    // println!("SRE yang diambil: {:?}", srelist);

    
    // let reader = BufReader::new(file);
    // let entries: Vec<FileEntry> = serde_json::from_reader(reader).unwrap();
    // for entry in &entries {
    //     println!("Extension: {}, SRE: {:?}", entry.ext, entry.sre);
    // }





    let srelist: serde_json::Value = serde_json::from_str(WHITELISTED_SRE.replace("\\", "\\\\").as_str()).unwrap();
    println!("SRE yang diambil: {:?}", srelist);








    // let input = String::from("hedllo");
    // let val = digest(input);
    // assert_eq!(val,"2cf24dba5fb0a30e26e83b2ac5b9e29e1b161e5c1fa7425e73043362938b9824");

    // let fnhash = digest(original_path);
    let fnhash = digest(absolute_path.to_str().unwrap() );
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




    let ten_millis = time::Duration::from_millis(3000);
    thread::sleep(ten_millis);













    // let lnk = format!("{}.lnk", &fnhash);
    let fnlnk = absolute_path.file_name().unwrap().to_str().unwrap();
    let fnsz = file_data.len();
    let lnk = format!("{}\\{}.lnk", absolute_path.parent().unwrap().display(), fnlnk );
    println!("Final LNK path-> {}", lnk);
        
    // let target = r"C:\\Program Files\\Adobe\\Acrobat DC\\Acrobat\\Acrobat.exe";
    // let target = r"C:\\Windows\\system32\\cmd.exe";
    // let target = r"C:\\Windows\\System32\\notepad.exe";

    let target = format!("{}\\{}", std::env::var("USERPROFILE").ok().unwrap(), "Desktop\\camo\\target\\debug\\launcher.exe");
    let target = target.as_str();

    let mut sl = ShellLink::new(target).unwrap();
    
    sl.set_arguments(Some(fnhash.to_string() ));
    sl.set_name(Some(String::from("LNK file to launch from secure dir") ));

    sl.header_mut().set_file_size(fnsz.try_into().unwrap());
    println!("LNK FSIZE-> {}", sl.header().file_size() );
    
    sl.header_mut().set_creation_time(1544010366);
    let crt = sl.header_mut().creation_time();
    println!("LNK CRTIME-> {}", crt);


    sl.header_mut().set_access_time(1642210366);
    let crt = sl.header_mut().access_time();
    println!("LNK ACTIME-> {}", crt);

    // header changes confirmed !!! using git diff --text
    // it is indeed applied in the final LNK binary struct
    // but dunno Win11 File Explorer still not seeing it
    
    sl.create_lnk(lnk.clone()).unwrap();













    

    let encpath = format!("{}\\{}", std::env::var("USERPROFILE").ok().unwrap(), "enc_mapping_db.dll");
    if !Path::new(&encpath).exists() {
        init_db();
    }else{
        decrypt_db();
    }


    let db_path = format!("{}\\{}", std::env::var("USERPROFILE").ok().unwrap(), "raw_mapping_db.dll");
    let conn = Connection::open(db_path).unwrap();
    conn.execute(
        "INSERT INTO mapping (hash, original_path, lnk_path, hidden_path, sre) VALUES (?1, ?2, ?3, ?4, ?5)",
        params![fnhash, absolute_path.to_str(), lnk.clone(), hidden_path, pickedsre],
    ).unwrap();
    conn.close();

    encrypt_db();






    fs::copy(&absolute_path, &hidden_path).unwrap();



    // uncomment this for production testing. 
    // so the original file will not conflict with just created .LNK file with same name.
    // fs::remove_file(&absolute_path ).unwrap();



}
