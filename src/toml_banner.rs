use std::{collections::HashMap};
use serde::{Serialize, Deserialize};
use toml;
use crate::banner::{BannerType, Banner};

use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

#[derive(Deserialize, Serialize)]
pub(crate) struct TomlBanner{
    pub(crate) pool: HashMap<String, Vec<String>>,
    pub(crate) rate_up: HashMap<String, Vec<String>>,
    pub(crate) banner_type: BannerType
}

#[allow(dead_code)]
impl TomlBanner {
    pub fn from_banner(banner: &Banner) -> Self {
        let mut pool = HashMap::new();
        for (k, v) in banner.pool.iter() {
            pool.insert(k.to_string(), v.to_owned());
        };

        let mut rate_up = HashMap::new();
        for (k, v) in banner.rate_up.iter() {
            rate_up.insert(k.to_string(), v.to_owned());
        };

        Self {
            pool, rate_up,
            banner_type: banner.banner_type
        }
    }

    pub fn from_toml_file(file_name: String) -> Self {
        let path = Path::new(&file_name);
        let display = path.display();

        // Open the path in read-only mode, returns `io::Result<File>`
        let mut file = match File::open(&path) {
            Err(why) => panic!("couldn't open {}: {}", display, why),
            Ok(file) => file,
        };

        // Read the file contents into a string, returns `io::Result<usize>`
        let mut s = String::new();
        match file.read_to_string(&mut s) {
            Err(why) => panic!("couldn't read {}: {}", display, why),
            Ok(_) => (),
        }

        toml::from_str::<Self>(&s).unwrap()
    }

    pub fn to_string(&self) -> String {
        match toml::to_string(self) {
            Ok(res) => res,
            Err(why) => {
                println!("[Panik!!!] Cannot parse to string: {}", why);
                "".to_owned()
            }
        }
    }
}
