use core::panic;
use std::collections::HashMap;
use std::default::Default;
use std::env;
use std::ffi::OsString;
use std::fs;
// use std::string::ToString;
use std::path::PathBuf;

use serde::{Deserialize, Serialize};
use serde_json::Value;
use strum_macros::Display;
use strum_macros::EnumString;
use toml;
// use inquire::Text;

mod shepatra;

const CONFIG_FILENAME: &'static str = "config.toml";
const I18N_DIRNAME: &'static str = "i18n";

#[derive(Serialize, Deserialize, EnumString, Display, Debug)]
enum Language {
    #[strum(serialize = "en")]
    English,
    #[strum(serialize = "ja")]
    Japanese,
}

#[derive(Serialize, Deserialize, Debug)]
struct Config {
    language: Language,
    path_to_store_recipes: OsString,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            language: Language::Japanese,
            path_to_store_recipes: "".into(),
        }
    }
}

fn main() {
    todo!("implement process for make config file when first execution of app");
    // --
    
    // --

    let exe_path = match env::current_exe() {
        Ok(exe_path) => {
            println!("{}", exe_path.display());
            exe_path
        }
        Err(e) => {
            panic!("failed to get current exe path: {e}");
        }
    };
    let exe_dirpath = exe_path.parent().unwrap();

    // -try to load config file-
    let mut config_filepathes = [
        exe_dirpath.join(&CONFIG_FILENAME),
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(&CONFIG_FILENAME),
        env::current_dir().unwrap().join(&CONFIG_FILENAME),
        env::current_dir().unwrap().join("src").join(&CONFIG_FILENAME),
    ]
    .into_iter();
    let content = loop {
        if let Some(path_) = config_filepathes.next() {
            match fs::read_to_string(&path_) {
                Ok(content) => {println!("search {:?}", path_); break content},
                Err(_) => {
                    continue;
                }
            };
        } else {
            panic!("failed to load config.toml file");
        };
    };
    let config: Config = toml::from_str(&content).expect("invalid config file");
    // --

    let i18n_filepath = exe_dirpath
        .join(I18N_DIRNAME)
        .join(config.language.to_string() + ".json");
    let content = fs::read_to_string(&i18n_filepath).expect(&format!(
        "failed to load localization file (it should be at `{}`)",
        &i18n_filepath.display()
    ));
    let i18ntexts: HashMap<String, Value> =
        serde_json::from_str(&content).expect("invalid json file");
    println!("-result-");
    println!("i18ntexts {:?}", i18ntexts);
    println!("config {:?}", config);
    // let name = Text::new("").prompt();

    // match name {
    //     Ok(name) => println!("Hello {}", name),
    //     Err(_) => println!("An error happened when asking for your name, try again later."),
    // }
}
