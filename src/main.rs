extern crate regex;
extern crate serde;
extern crate toml;

pub mod filelines;

pub mod generator;

pub use crate::filelines::FileLines;
pub use crate::generator::generate_document_for_md;

use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Serialize, Deserialize, Debug)]
struct Setting {
    regex: String,
    out_dir: String,
}


fn main() -> Result<(), Box<dyn std::error::Error>> {
    let setting_str = fs::read_to_string("./easydoc4lib.toml").expect("cant load setting file ./easydoc4lib");
    let setting_toml: Setting = toml::from_str(&setting_str).expect("cant load setting toml");

    let re = regex::Regex::new(&setting_toml.regex)?;
    let current = std::path::Path::new(".");
    let out_dir = std::path::Path::new(&setting_toml.out_dir);
    generate_document_for_md(&re, &current, &out_dir)?;
    Ok(())
}
