use std::{
    fs::{self, DirEntry},
    path::{Path, PathBuf},
};

use askama::Template;
use rust_decimal::Decimal;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case", untagged)]
enum StructValue {
    Str(String),
    Decimal(Decimal),
    Bool(bool),
}

impl Default for StructValue {
    fn default() -> Self {
        StructValue::Bool(false)
    }
}

#[derive(Debug, Deserialize, Default)]
#[serde(rename_all = "snake_case", default)]
struct StructKind {
    name: String,
    value: StructValue,
    description: String,
    is_optional: bool,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case", untagged)]
enum SchemaValue {
    Str(String),
    Struct(StructKind),
}

#[derive(Debug, Template, Deserialize, Default)]
#[serde(rename_all = "snake_case", default)]
#[template(path = "schema.template", escape = "none")]
struct Schema {
    name: String,
    values: Vec<SchemaValue>,
}

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
            // read the dir append into files
        } else {
            // panic
            panic!("{:?} is not a file or dir", file.path());
        }

        if files.is_empty() {
            break;
        }

        // if file.file_type().expect("get file type failed").is_dir() {
        //     // goes into read_dir_recursively
        // } else if file.
    }
    // }
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
