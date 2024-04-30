use std::collections::HashMap;

use quick_xml::events::attributes::Attribute;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]

pub enum DBLPEntry {
    Article(GeneralEntry),
    Incollection(GeneralEntry),
    Inproceedings(GeneralEntry),
    Proceedings(GeneralEntry),
    Book(GeneralEntry),
    Phdthesis(GeneralEntry),
    Mastersthesis(GeneralEntry),
    Www(GeneralEntry),
    Person(GeneralEntry),
    Data(GeneralEntry),
    #[serde(other)]
    Unknown,
}

#[derive(Debug, Deserialize)]
pub struct Dblp {
    #[serde(rename = "$value")]
    pub fields: Vec<DBLPEntry>,
}

#[derive(Debug, Deserialize)]
pub struct GeneralEntry {
    #[serde(rename = "@key")]
    pub key: String,
    #[serde(rename = "@mdate")]
    pub mdate: Option<String>,
    #[serde(rename = "@publtype")]
    pub publtype: Option<String>,
    #[serde(rename = "@reviewid")]
    pub reviewid: Option<String>,
    #[serde(rename = "@rating")]
    pub rating: Option<String>,
    #[serde(rename = "@cdate")]
    pub cdate: Option<String>,
    #[serde(rename = "@person")]
    pub person: Option<String>,
    #[serde(rename = "$value")]
    pub fields: Vec<GeneralField>,
}

#[derive(Debug, Deserialize)]
pub struct GeneralField {
    #[serde(rename = "$text")]
    pub content: Option<String>,
    #[serde(flatten)]
    pub attrs: HashMap<String, String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Field {
    Author(Option<GeneralField>),
    Editor(Option<GeneralField>),
    Title(Option<GeneralField>),
    Booktitle(Option<GeneralField>),
    Pages(Option<GeneralField>),
    Year(Option<GeneralField>),
    Address(Option<GeneralField>),
    Journal(Option<GeneralField>),
    Volume(Option<GeneralField>),
    Number(Option<GeneralField>),
    Month(Option<GeneralField>),
    Url(Option<GeneralField>),
    Ee(Option<GeneralField>),
    Cdrom(Option<GeneralField>),
    Cite(Option<GeneralField>),
    Publisher(Option<GeneralField>),
    Note(Option<GeneralField>),
    Crossref(Option<GeneralField>),
    Isbn(Option<GeneralField>),
    Series(Option<GeneralField>),
    School(Option<GeneralField>),
    Chapter(Option<GeneralField>),
    Publnr(Option<GeneralField>),
    Stream(Option<GeneralField>),
    Rel(Option<GeneralField>),
}
