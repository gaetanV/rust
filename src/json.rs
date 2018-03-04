extern crate serde_json;

use self::serde_json::{Value};
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;

pub fn json_to_csv() -> String {
    let mut response = "".to_string();
    let mut header = "".to_string();

    let path = "./resources/json.js";

    let mut file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", path, why.description()),
        Ok(file) => file,
    };

    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => panic!("couldn't read {}: {}", path, why.description()),
        Ok(_) => {
            
            let v: Value = serde_json::from_str(&s).unwrap_or_default();
            if v.is_array() {
                let mut cp = true;
                for array in v.as_array().unwrap() {
                    if cp {
                        for (i,_) in  array.as_object().unwrap() {
                            header.push_str(i);
                            header.push_str(";");
                        }
                        cp = false;
                        header.push_str("\n");
                    }
                    for (_,value) in  array.as_object().unwrap() {
                        response.push_str(&value.to_string()); 
                        response.push_str(";");;
                    }
                    response.push_str("\n");

                }
              
            } else{

                for (i,_) in  v.as_object().unwrap() {
                    header.push_str(i);
                    header.push_str(";");
                }
                header.push_str("\n");

                for (_,value) in  v.as_object().unwrap() {
                    response.push_str(&value.to_string()); 
                    response.push_str(";");;
                }
                response.push_str("\n");
            }

            header.push_str(&response); 
          
            let path ="./resources/json.csv";
            
            let mut file = match File::create(&path) {
                Err(why) => panic!("couldn't create {}: {}", path, why.description()),
                Ok(file) => file,
            };

            match file.write_all(header.as_bytes()) {
                Err(why) => {
                    panic!("couldn't write to {}: {}", path, why.description())
                },
                Ok(_) => println!("successfully wrote to {}", path),
            }  
           
        }
    }
    return header;
}