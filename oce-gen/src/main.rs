use std::{fs, path::PathBuf};

use askama::Template;
use serde::Deserialize;

#[derive(Template, Deserialize)]
#[template(path = "schema.template", escape = "none")]
struct Schema {
    name: String,
    #[serde(rename(deserialize = "type"))]
    typ: String,
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

fn main() {
    let config: Config =
        toml::from_str(&fs::read_to_string(DEFAULT_CONFIG_PATH).expect("read config.toml failed"))
            .expect("toml decide failed");

    for project in config.projects.into_iter() {
        let mut f = PathBuf::from(DEFAULT_DEFINITIONS_FOLDER_PATH);
        f.push(project.name);
        fs::read_dir(f);
    }
}
