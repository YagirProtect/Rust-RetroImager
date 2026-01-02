use std::fs::File;
use std::io::{BufReader, BufWriter};
use std::path::PathBuf;
use serde::{Deserialize, Serialize};
use crate::file_system_utils;

#[derive(Serialize, Deserialize, Debug)]
#[serde(default)]
pub struct Config{
    pub colors_palette_size: u16,
    pub image_percent: f32,
}

impl Config {
    pub fn set_colors_count(&mut self, p0: u16) {
        self.colors_palette_size = p0;
    }

    pub fn set_size(&mut self, p0: f32) {
        self.image_percent = p0;
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            colors_palette_size: 8,
            image_percent: 0.8,
        }
    }
}

impl Config {
    pub fn new() -> Config {
        println!("{}", "Config created");
        Config :: default()
    }

    pub fn file_name() -> String{
        return "config.json".to_string()
    }

    pub fn full_file_path() -> PathBuf {
        return file_system_utils::get_app_dir().join("config.json");
    }

    pub fn read_file(&mut self){
        if (file_system_utils::is_file_exist(Self::file_name().as_str())){
            let file = match File::open(Self::full_file_path()){
                Ok(file) => file,
                Err(error) => panic!("There was a problem opening the file: {:?}", error),
            };



            let reader = BufReader::new(file);

            let this: Config = match serde_json::from_reader(reader) {
                Ok(config) => config,
                Err(error) => panic!("There was a problem deserializing the file: {:?}", error),
            };

            *self = this;
            Self::write_file(self);
        }else{
            Self::write_file(self);
        }
    }

    pub fn write_file(&self){
        let file = File::create(Self::full_file_path()).unwrap();
        let writer = BufWriter::new(file);

        println!("{}", "Config saved");
        match serde_json::to_writer_pretty(writer, self){
            Ok(_) => (),
            Err(e) => panic!("{}", e)
        }
    }
}

