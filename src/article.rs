use color_eyre::Result;
use eyre::eyre;
use serde::Deserialize;

use std::ffi::OsStr;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::{Path, PathBuf};

pub struct ArticleInformation {
    pub image_path: PathBuf,
    pub title: String,
    pub tags: Vec<String>,
}

#[derive(PartialEq, Debug, Deserialize)]
struct Article {
    title: String,
    slug: Option<String>,
    taxonomies: Option<Taxonomies>,
}

#[derive(PartialEq, Debug, Deserialize)]
struct Taxonomies {
    tags: Vec<String>,
}

impl ArticleInformation {
    pub fn retrieve(article_file: &str) -> Result<Self> {
        let article_path = Path::new(&article_file);
        if article_path.is_file() {
            let article = parse_article(article_path)?;
            let parent = article_path.parent().unwrap_or_else(|| Path::new("."));
            let article_slug = article
                .slug
                .clone()
                .or_else(|| {
                    article_path
                        .file_stem()
                        .map(OsStr::to_string_lossy)
                        .map(String::from)
                })
                .map(slug::slugify)
                .ok_or_else(|| {
                    eyre!("Could not get the article slug from the article or the path")
                })?;
            let article_image_path = parent.join(article_slug).with_extension("png");

            Ok(Self {
                title: article.title,
                image_path: article_image_path,
                tags: article.taxonomies.map(|t| t.tags).unwrap_or_default(),
            })
        } else {
            Err(eyre!("{} passed in, but it does not exists", article_file))
        }
    }
}

fn parse_article(article_path: &Path) -> Result<Article> {
    let article_file = BufReader::new(File::open(article_path)?);
    let article_contents: String = article_file
        .lines()
        .filter_map(|line_result| line_result.ok())
        .skip_while(|line| !line.starts_with("+++"))
        .skip_while(|line| line.starts_with("+++"))
        .take_while(|line| !line.starts_with("+++"))
        .collect::<Vec<String>>()
        .join("\n");
    let article = toml::from_str(&article_contents)?;
    Ok(article)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_should_process_zola_frontmatter() {
        let article_path = Path::new("tests/blog.md");
        assert_eq!(true, article_path.is_file());
        let article = parse_article(article_path).expect("Could not read article");
        assert_eq!(
            Article {
                title: "Principles of Technology Leadership".to_owned(),
                slug: None,
                taxonomies: Some(Taxonomies {
                    tags: vec![String::from("leadership"), String::from("ethics"),]
                })
            },
            article
        );
    }
}
