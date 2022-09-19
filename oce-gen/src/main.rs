use std::{
    fs::{self, DirEntry},
    path::{Path, PathBuf},
};

use anyhow::{Context, Result};
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

const DEFAULT_DEFINITIONS_FOLDER_PATH: &str = "definitions";

fn main() {
    let f = PathBuf::from(DEFAULT_DEFINITIONS_FOLDER_PATH);

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
                Schema::Mod(m) => {
                    let package = m.package.to_snake_case();
                    let content = m.render().unwrap();
                    fs::create_dir_all(format!("../oce-ftx-rs/src/schema/{}", package)).unwrap();
                    fs::write(
                        format!("../oce-ftx-rs/src/schema/{}/mod.rs", package),
                        content,
                    )
                    .unwrap();
                }
                Schema::Struct(mut s) => {
                    let package = s.package.to_snake_case();
                    let file_name = s.name.to_snake_case();
                    s.name = s.name.to_upper_camel_case();
                    for f in s.fields.iter_mut() {
                        f.org_name = f.name.clone();
                        f.name = f.name.to_snake_case();
                    }
                    let content = s.render().unwrap();
                    fs::create_dir_all(format!("../oce-ftx-rs/src/schema/{}", package)).unwrap();
                    fs::write(
                        format!("../oce-ftx-rs/src/schema/{}/{}.rs", package, file_name),
                        content,
                    )
                    .unwrap();
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
    let data = fs::read(path).with_context(|| format!("fs::read failed, path: {:?}", path))?;
    let ret = toml::from_slice(&data)
        .with_context(|| format!("toml::from_slice failed, path: {:?}", path))?;
    Ok(ret)
}
