use serde::{Deserialize, Serialize};
use serde_json::{Result};

use std::io::BufReader;
use std::fs::File;
use std::path::Path;


pub fn get_u8_64(pt: &Path)-> Result<Vec<u8>>{
    match File::open(pt){
        Ok(f)=>{
            let data: Vec<u8> = serde_json::from_reader(BufReader::new(f))?;
            if data.len() != 64 {
                return panic!("Error data len is not 64");
            };
            Ok(data)
        },
        Err(e)=>{
            panic!("Error by open file: {:?}", e)
        }
    }
}