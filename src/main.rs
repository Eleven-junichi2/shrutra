use std::collections::HashMap;
use std::default::Default;
use std::env;
// use std::ffi::OsString;
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
    recipes_path: String,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            language: Language::Japanese,
            recipes_path: ".".into(),
        }
    }
}

fn load_file_from_candidate_paths<'a>(
    filepath_candidates: &mut impl Iterator<Item = &'a PathBuf>,
) -> Result<String, String> {
    let content = loop {
        if let Some(path_) = filepath_candidates.next() {
            match fs::read_to_string(&path_) {
                Ok(content) => break content,
                Err(_) => {
                    continue;
                }
            };
        } else {
            return Err("file not found in given candidate paths".to_string());
        };
    };
    Ok(content)
}

fn load_config<'a>(filepath_candidates: &mut impl Iterator<Item = &'a PathBuf>) -> Config {
    toml::from_str(&load_file_from_candidate_paths(filepath_candidates).unwrap())
        .expect("invalid config file")
}

fn load_i18ntexts<'a>(
    filepath_candidates: &mut impl Iterator<Item = &'a PathBuf>,
) -> HashMap<String, Value> {
    serde_json::from_str::<HashMap<String, Value>>(
        &load_file_from_candidate_paths(filepath_candidates).unwrap(),
    )
    .expect("invalid json file")
}

fn main() {
    let exe_path = env::current_exe().unwrap();
    let exe_dirpath = exe_path.parent().unwrap();

    let config = load_config(
        &mut [
            exe_dirpath.join(&CONFIG_FILENAME),
            PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(&CONFIG_FILENAME),
            env::current_dir().unwrap().join(&CONFIG_FILENAME),
            env::current_dir()
                .unwrap()
                .join("src")
                .join(&CONFIG_FILENAME),
        ]
        .iter(),
    );

    let i18nfilepath_part = PathBuf::from(I18N_DIRNAME)
        .join("cli".to_string())
        .join(config.language.to_string() + ".json");

    let i18ntexts = load_i18ntexts(
        &mut [
            exe_dirpath.join(&i18nfilepath_part),
            PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(&i18nfilepath_part),
        ]
        .iter(),
    );

    // let i18n_filepath = exe_dirpath
    //     .join(I18N_DIRNAME)
    //     .join(config.language.to_string() + ".json");
    // let content = fs::read_to_string(&i18n_filepath).expect(&format!(
    //     "failed to load localization file (it should be at `{}`)",
    //     &i18n_filepath.display()
    // ));
    // let i18ntexts: HashMap<String, Value> =
    //     serde_json::from_str(&content).expect("invalid json file");
    // println!("-result-");
    // println!("i18ntexts {:?}", i18ntexts);
    // println!("config {:?}", config);
    // let name = Text::new("").prompt();

    // match name {
    //     Ok(name) => println!("Hello {}", name),
    //     Err(_) => println!("An error happened when asking for your name, try again later."),
    // }
}
