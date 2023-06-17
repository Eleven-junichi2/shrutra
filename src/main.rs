// TODO: improve error handling

use inquire::{Select, Text};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::default::Default;
use std::env;
use std::fs;
use std::fs::File;
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
const RECIPES_FILENAME: &'static str = "recipes.json";

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
    relative_recipes_path_flag: bool,
    recipes_path_from_exedir: bool,
    recipes_path_from_cwd: bool,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            language: Language::Japanese,
            recipes_path: ".".into(),
            relative_recipes_path_flag: true,
            recipes_path_from_exedir: true,
            recipes_path_from_cwd: false,
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

impl Config {
    fn recipes_path_by_rules(&self) -> Result<PathBuf, Box<dyn std::error::Error>> {
        if self.relative_recipes_path_flag {
            if self.recipes_path_from_exedir {
                let exe_path = env::current_exe().unwrap();
                let exe_dirpath = exe_path.parent().unwrap();
                return Ok(exe_dirpath.join(&self.recipes_path).join(RECIPES_FILENAME));
            } else if self.recipes_path_from_cwd {
                return Ok(env::current_dir()
                    .unwrap()
                    .join(&self.recipes_path)
                    .join(RECIPES_FILENAME));
            } else {
                return Ok(PathBuf::from(&self.recipes_path).join(RECIPES_FILENAME));
            }
        } else {
            return Ok(PathBuf::from(&self.recipes_path).join(RECIPES_FILENAME));
        }
    }
}

fn load_toml<'a, T: serde::de::DeserializeOwned>(
    filepath_candidates: &mut impl Iterator<Item = &'a PathBuf>,
) -> T {
    toml::from_str(&load_file_from_candidate_paths(filepath_candidates).unwrap())
        .expect("invalid toml file")
}

fn save_recipes<P: AsRef<Path>>(
    filepath: &P,
    recipes: &HashMap<String, Recipe>,
) -> Result<(), serde_json::Error> {
    let file = File::create(filepath).unwrap();
    serde_json::to_writer_pretty(file, recipes)
}

fn main() {
    let exe_path = env::current_exe().unwrap();
    let exe_dirpath = exe_path.parent().unwrap();

    let config: Config = load_toml(
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

    let i18ntexts: HashMap<String, String> = serde_json::from_str(
        &load_file_from_candidate_paths(
            &mut [
                exe_dirpath.join(&i18nfilepath_part),
                PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(&i18nfilepath_part),
            ]
            .iter(),
        )
        .unwrap(),
    )
    .unwrap();

    let mut recipes: HashMap<String, Recipe> = match serde_json::from_str(
        &load_file_from_candidate_paths(&mut vec![config.recipes_path_by_rules().unwrap()].iter())
            .unwrap(),
    ) {
        Ok(recipes) => recipes,
        Err(_) => HashMap::<String, Recipe>::new(),
    };

    // i18ntexts[""].as_str() to ged rid of double quotes
    println!("{}", i18ntexts["welcome"].as_str());
    loop {
        let selected_option = Select::new(
            "",
            vec![
                i18ntexts["go_to_recipe_making"].as_str(),
                i18ntexts["make_hashed_password"].as_str(),
                i18ntexts["exit"].as_str(),
            ],
        )
        .with_help_message(i18ntexts["help_msg_Select"].as_str())
        .prompt()
        .unwrap();
        if selected_option == i18ntexts["go_to_recipe_making"].as_str() {
            let mut recipe = Recipe {
                layers: Vec::<HashFuncNames>::new(),
            };
            loop {
                let mut options = vec![
                    i18ntexts["cancel"].as_str().to_string(),
                    i18ntexts["submit"].as_str().to_string(),
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
                .with_help_message(i18ntexts["help_msg_Select"].as_str())
                .prompt()
                .unwrap();
                if selected_option.as_str() == i18ntexts["cancel"].as_str() {
                    break;
                } else if HashFuncNames::iter().any(|name| name.to_string() == selected_option) {
                    recipe
                        .layers
                        .push(HashFuncNames::from_str(selected_option.as_str()).unwrap());
                } else if selected_option.as_str() == i18ntexts["submit"].as_str() {
                    let name_for_recipe = Text::new(i18ntexts["make_name_for_recipe"].as_str())
                        .prompt()
                        .unwrap();
                    recipes.insert(name_for_recipe, recipe);
                    let recipes_path = config.recipes_path_by_rules().unwrap();
                    save_recipes(&recipes_path, &recipes).unwrap();
                    break;
                }
            }
        } else if selected_option == i18ntexts["make_hashed_password"].as_str() {
            if recipes.is_empty() {
                println!("{}", i18ntexts["recipes_is_empty_plz_make"].as_str());
                continue;
            }
            let mut options = vec![i18ntexts["cancel"].as_str().to_string()];
            options.extend(recipes.keys().map(|key| key.clone()));
            let selected_recipe =
                Select::new(i18ntexts["which_recipe_would_you_like"].as_str(), options)
                    .prompt()
                    .unwrap();
            if selected_recipe == i18ntexts["cancel"].as_str() {
                break;
            }
            let str_to_be_hased = Text::new(i18ntexts["input_password"].as_str())
                .prompt()
                .unwrap();
            println!(
                "{}",
                hash_with_recipe(&str_to_be_hased, &recipes[&selected_recipe])
            );
        } else if selected_option == i18ntexts["exit"].as_str() {
            break;
        }
    }
}
