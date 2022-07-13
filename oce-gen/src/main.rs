use std::{
    fs::{self, DirEntry},
    path::{Path, PathBuf},
};

use askama::Template;
use heck::{ToSnakeCase, ToUpperCamelCase};
use serde::Deserialize;

mod schema;
use schema::*;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
enum Definition {
    Schema(Schema),
}

#[derive(Deserialize)]
#[serde(rename_all = "snake_case")]
struct Config {
    projects: Vec<Project>,
}

#[derive(Deserialize)]
#[serde(rename_all = "snake_case")]
struct Project {
    name: String,
}

const DEFAULT_CONFIG_PATH: &str = "config.toml";
const DEFAULT_DEFINITIONS_FOLDER_PATH: &str = "definitions";

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn main() {
    // TODO: read from env or config file
    // let config: Config =
    //     toml::from_str(&fs::read_to_string(DEFAULT_CONFIG_PATH).expect("read config.toml failed"))
    //         .expect("toml decide failed");

    // for project in config.projects.into_iter() {
    let f = PathBuf::from(DEFAULT_DEFINITIONS_FOLDER_PATH);
    // f.push(project.name);

    // init first round of files
    let mut files = vec![];
    for f in fs::read_dir(f).expect("read definitions folder failed") {
        files.push(f.expect("file unwrap failed"));
    }

    let mut definitions = vec![];
    loop {
        let file = files.pop().expect("files pop failed");
        let file_type = file.file_type().expect("");

        if file_type.is_file() {
            definitions.push(read_file(&file.path()).expect("read_file failed"));
        } else if file_type.is_dir() {
            for f in read_dir(&file.path()).expect("read_dir failed").into_iter() {
                files.push(f);
            }
        } else {
            panic!("{:?} is not a file or dir", file.path());
        }

        if files.is_empty() {
            break;
        }
    }

    for definition in definitions.into_iter() {
        match definition {
            Definition::Schema(schema) => match schema {
                Schema::Enum(e) => {
                    let content = e.render().unwrap();
                    fs::write(format!("./{}.rs", e.name), content).unwrap();
                }
                Schema::Struct(mut s) => {
                    let file_name = s.name.to_snake_case();
                    s.name = s.name.to_upper_camel_case();
                    for f in s.fields.iter_mut() {
                        f.name = f.name.to_snake_case();
                    }
                    let content = s.render().unwrap();
                    fs::write(format!("./{}.rs", file_name), content).unwrap();
                }
            },
        }
    }
}

fn read_dir(path: &Path) -> Result<Vec<DirEntry>> {
    let mut files = vec![];
    for f in fs::read_dir(path)? {
        let f = f?;
        files.push(f);
    }

    Ok(files)
}

fn read_file(path: &Path) -> Result<Definition> {
    let data = fs::read(path)?;
    Ok(toml::from_slice(&data)?)
}
