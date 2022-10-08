use askama::Template;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub(crate) enum StructValueType {
    Str,
    Usize,
}

impl Default for StructValueType {
    fn default() -> Self {
        StructValueType::Str
    }
}

#[derive(Debug, Default, Deserialize)]
#[serde(rename_all = "snake_case", default)]
pub(crate) struct StructField {
    pub(crate) name: String,
    pub(crate) is_optional: bool,
    #[serde(rename = "type")]
    pub(crate) typ: StructValueType,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub(crate) enum MethodType {
    Get,
    Post,
}

impl Default for MethodType {
    fn default() -> Self {
        MethodType::Get
    }
}

#[derive(Debug, Template, Deserialize)]
#[serde(rename_all = "snake_case")]
#[template(path = "rust/rest/struct.template", escape = "none")]
pub(crate) struct StructKind {
    pub(crate) name: String,
    pub(crate) method: MethodType,
    pub(crate) path: String,
    pub(crate) fields: Vec<StructField>,
}

#[derive(Debug, Default, Deserialize)]
#[serde(rename_all = "snake_case", default)]
pub(crate) struct ModField {
    pub(crate) name: String,
    pub(crate) pub_name: String,
}

// #[derive(Debug, Template, Deserialize)]
// #[template(path = "rust/schema/mod.template", escape = "none")]
// pub(crate) struct ModKind {
//     pub(crate) description: String,
//     pub(crate) fields: Vec<ModField>,
// }

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case", untagged)]
pub(crate) enum Rest {
    Struct(StructKind),
    // Mod(ModKind),
}
