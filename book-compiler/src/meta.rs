// 解析 Markdown 文件的元数据
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Meta {
    pub title: Option<String>,
    pub date: Option<String>,
    pub abstracts: Option<String>,
    pub category: Option<Vec<String>>,
    pub tags: Option<Vec<String>>,
    pub cover: Option<String>,
    pub feature: Option<bool>,
}

impl Meta {
    pub fn new() -> Self {
        Self {
            title: None,
            date: None,
            abstracts: None,
            category: None,
            tags: None,
            cover: None,
            feature: Some(false),
        }
    }
}

#[test]
fn test_meta() {
    let str = r#"
title = ""
date = ""
abstracts = ""
category = ["", ""]
tags = ["", ""]
cover = ""
feature = false"#;
    let meta = toml::from_str::<Meta>(str).unwrap();
    println!("{:?}", meta);
}