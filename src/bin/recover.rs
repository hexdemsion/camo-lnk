// cargo run --bin recover -- .\foyo.lnk
// cargo run --bin recover -- .\dongo.lnk


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



mod common;
use crate::common::*;









fn main() {

    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: recover.exe .\\path\\to\\file.ext.lnk");
        let ten_millis = time::Duration::from_millis(5000);
        thread::sleep(ten_millis);
    
        return;
    }
    let lnk_args = &args[1];
    println!("LNK args: {}", lnk_args);
    let abs_lnk_path: PathBuf = fs::canonicalize(lnk_args).expect("Failed to get absolute LNK path");
    println!("Absolute LNK path: {}", abs_lnk_path.display());


    let encdb_path = format!("{}\\{}", std::env::var("USERPROFILE").ok().unwrap(), "enc_mapping_db.dll");
    if !Path::new(&encdb_path).exists() {
        init_db();
    }else{
        decrypt_db();
    }




    // inner block scoping to avoid still-open .\mapping.db FILE_HANDLE
    {
        let db_path = format!("{}\\{}", std::env::var("USERPROFILE").ok().unwrap(), "raw_mapping_db.dll");
        let conn = Connection::open(&db_path).unwrap();


        let mut stmt = conn.prepare("SELECT * FROM mapping WHERE lnk_path=?1").unwrap();

        let rows = stmt.query_map(params![abs_lnk_path.to_str()], |row| {

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

            fs::copy(&hidden_path, &original_path).unwrap();
            fs::remove_file(&hidden_path).unwrap();
            fs::remove_file(&lnk_path).unwrap();

        }
    }







    encrypt_db();

    let ten_millis = time::Duration::from_millis(6000);
    thread::sleep(ten_millis);


}
