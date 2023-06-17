use inquire::{Select, Text};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use std::default::Default;
use std::env;
use std::fs;
use std::path::Path;
use std::path::PathBuf;
use std::str::FromStr;
use strum::IntoEnumIterator;
use strum_macros::Display;
use strum_macros::EnumString;
use toml;

mod shepatra;
use shepatra::{hash_with_recipe, HashFuncNames, Recipe};

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

fn load_recipes<'a>(filepath_candidates: &mut impl Iterator<Item = &'a PathBuf>) {
    todo!("load_recipes")
}

fn save_recipes<'a>(filepath: &Path) {
    todo!("save_recipes")
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

    let mut recipes: HashMap<String, Recipe> = HashMap::new();

    // as_str().unwrap() to ged rid of double quotes
    println!("{}", i18ntexts["welcome"].as_str().unwrap());
    loop {
        let selected_option = Select::new(
            "",
            vec![
                i18ntexts["go_to_recipe_making"].as_str().unwrap(),
                i18ntexts["make_hashed_password"].as_str().unwrap(),
                i18ntexts["exit"].as_str().unwrap(),
            ],
        )
        .with_help_message(i18ntexts["help_msg_Select"].as_str().unwrap())
        .prompt()
        .unwrap();
        if selected_option == i18ntexts["go_to_recipe_making"].as_str().unwrap() {
            let mut recipe = Recipe {
                layers: Vec::<HashFuncNames>::new(),
            };
            loop {
                let mut options = vec![
                    i18ntexts["cancel"].as_str().unwrap().to_string(),
                    i18ntexts["submit"].as_str().unwrap().to_string(),
                ];
                options.extend(
                    HashFuncNames::iter()
                        .map(|name| name.to_string())
                        .collect::<Vec<String>>(),
                );
                let selected_option = Select::new(
                    &recipe
                        .layers
                        .iter()
                        .map(|name| name.to_string())
                        .collect::<Vec<String>>()
                        .join(","),
                    options,
                )
                .with_help_message(i18ntexts["help_msg_Select"].as_str().unwrap())
                .prompt()
                .unwrap();
                if selected_option.as_str() == i18ntexts["cancel"].as_str().unwrap() {
                    break;
                } else if HashFuncNames::iter().any(|name| name.to_string() == selected_option) {
                    recipe
                        .layers
                        .push(HashFuncNames::from_str(selected_option.as_str()).unwrap());
                } else if selected_option.as_str() == i18ntexts["submit"].as_str().unwrap() {
                    let name_for_recipe =
                        Text::new(i18ntexts["make_name_for_recipe"].as_str().unwrap())
                            .prompt()
                            .unwrap();
                    recipes.insert(name_for_recipe, recipe);
                    break;
                }
            }
        } else if selected_option == i18ntexts["make_hashed_password"].as_str().unwrap() {
            if recipes.is_empty() {
                println!(
                    "{}",
                    i18ntexts["recipes_is_empty_plz_make"].as_str().unwrap()
                );
                continue;
            }
            let mut options = vec![i18ntexts["cancel"].as_str().unwrap().to_string()];
            options.extend(recipes.keys().map(|key| key.clone()));
            let selected_recipe = Select::new(
                i18ntexts["which_recipe_would_you_like"].as_str().unwrap(),
                options,
            )
            .prompt()
            .unwrap();
            if selected_recipe == i18ntexts["cancel"].as_str().unwrap() {
                break;
            }
            let str_to_be_hased = Text::new(i18ntexts["input_password"].as_str().unwrap())
                .prompt()
                .unwrap();
            println!(
                "{}",
                hash_with_recipe(&str_to_be_hased, &recipes[&selected_recipe])
            );
        } else if selected_option == i18ntexts["exit"].as_str().unwrap() {
            break;
        }
    }
}
