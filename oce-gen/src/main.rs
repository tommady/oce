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
const DEFAULT_DEFINITIONS_FTX_NAME: &str = "ftx";
const DEFAULT_DESTINATION_FTX_RUST_FOLDER_NAME: &str = "oce-ftx-rs";

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

    for (mut pathbuf, definition) in definitions.into_iter() {
        if pathbuf.starts_with(DEFAULT_DEFINITIONS_FTX_NAME) {
            pathbuf.set_extension("rs");
            let path = pathbuf
                .strip_prefix(DEFAULT_DEFINITIONS_FTX_NAME)
                .expect("strip_prefix DEFAULT_DEFINITIONS_FTX_NAME failed");

            let path = Path::new("..")
                .join(DEFAULT_DESTINATION_FTX_RUST_FOLDER_NAME)
                .join("src")
                .join(path);
            let parent = path
                .parent()
                .unwrap_or_else(|| panic!("extract parent from {:?} failed", path));
            if !parent.exists() {
                fs::create_dir(parent)
                    .unwrap_or_else(|e| panic!("create_dir for {:?} failed: {}", parent, e));
            }

            match definition {
                Definition::Schema(schema) => match schema {
                    Schema::Enum(e) => {
                        let content = e
                            .render()
                            .unwrap_or_else(|_| panic!("render {:?} failed", path));
                        fs::write(&path, content)
                            .unwrap_or_else(|_| panic!("write {:?} failed", path));
                    }
                    Schema::Mod(m) => {
                        let content = m
                            .render()
                            .unwrap_or_else(|_| panic!("render {:?} failed", path));
                        fs::write(&path, content)
                            .unwrap_or_else(|_| panic!("write {:?} failed", path));
                    }
                    Schema::Struct(mut s) => {
                        s.name = s.name.to_upper_camel_case();
                        for f in s.fields.iter_mut() {
                            f.org_name = f.name.clone();
                            f.name = f.name.to_snake_case();
                        }
                        let content = s
                            .render()
                            .unwrap_or_else(|_| panic!("render {:?} failed", path));
                        fs::write(&path, content)
                            .unwrap_or_else(|_| panic!("write {:?} failed", path));
                    }
                },
            }
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

fn read_file(path: &Path) -> Result<(PathBuf, Definition)> {
    let data = fs::read(path).with_context(|| format!("fs::read failed, path: {:?}", path))?;
    let ret = toml::from_slice(&data)
        .with_context(|| format!("toml::from_slice failed, path: {:?}", path))?;

    let ret_path = path
        .strip_prefix(DEFAULT_DEFINITIONS_FOLDER_PATH)
        .with_context(|| format!("path.strip_prefix failed, path: {:?}", path))?
        .to_path_buf();

    Ok((ret_path, ret))
}
