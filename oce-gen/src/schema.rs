use std::fmt::Display;

use askama::Template;
use rust_decimal::Decimal;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case", untagged)]
pub(crate) enum StructValue {
    Str(String),
    Decimal(Decimal),
    Bool(bool),
    VectorOfDecimal(Vec<Decimal>),
    VectorOfVectorOfDecimal(Vec<Vec<Decimal>>),
}

impl Display for StructValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self {
            StructValue::Str(s) => write!(f, "{}", s),
            StructValue::Decimal(d) => write!(f, "{}", d),
            StructValue::Bool(b) => write!(f, "{}", b),
            StructValue::VectorOfDecimal(vd) => {
                write!(
                    f,
                    "[{}]",
                    vd.iter()
                        .map(|v| v.to_string())
                        .collect::<Vec<String>>()
                        .join(",")
                )
            }
            StructValue::VectorOfVectorOfDecimal(vvd) => {
                write!(
                    f,
                    "[[{}]]",
                    vvd.iter()
                        .map(|vd| vd
                            .iter()
                            .map(|v| v.to_string())
                            .collect::<Vec<String>>()
                            .join(","))
                        .collect::<Vec<String>>()
                        .join(",")
                )
            }
        }
    }
}

impl Default for StructValue {
    fn default() -> Self {
        StructValue::Bool(false)
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub(crate) enum StructValueType {
    ParseFromValue,
    Uint64,
    Timestamp,
}

impl Default for StructValueType {
    fn default() -> Self {
        StructValueType::ParseFromValue
    }
}

#[derive(Debug, Default, Deserialize)]
#[serde(rename_all = "snake_case", default)]
pub(crate) struct StructField {
    pub(crate) org_name: String,
    pub(crate) name: String,
    pub(crate) description: String,
    pub(crate) is_optional: bool,
    pub(crate) value: StructValue,
    #[serde(rename = "type")]
    pub(crate) typ: StructValueType,
}

#[derive(Debug, Template, Deserialize)]
#[serde(rename_all = "snake_case")]
#[template(path = "rust/schema/struct.template", escape = "none")]
pub(crate) struct StructKind {
    pub(crate) name: String,
    pub(crate) fields: Vec<StructField>,
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub(crate) enum EnumValue {
    Str(String),
}

impl Display for EnumValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self {
            EnumValue::Str(s) => write!(f, "{}", s),
        }
    }
}

#[derive(Debug, Template, Deserialize)]
#[template(path = "rust/schema/enum.template", escape = "none")]
pub(crate) struct EnumKind {
    pub(crate) name: String,
    pub(crate) values: Vec<EnumValue>,
}

#[derive(Debug, Default, Deserialize)]
#[serde(rename_all = "snake_case", default)]
pub(crate) struct ModField {
    pub(crate) name: String,
    pub(crate) pub_name: String,
}

#[derive(Debug, Template, Deserialize)]
#[template(path = "rust/schema/mod.template", escape = "none")]
pub(crate) struct ModKind {
    pub(crate) description: String,
    pub(crate) fields: Vec<ModField>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case", untagged)]
pub(crate) enum Schema {
    Enum(EnumKind),
    Struct(StructKind),
    Mod(ModKind),
}
