use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct Paragraph {
    name: String,
}

#[derive(Serialize, Deserialize)]
struct Article {
    title: String,
    author: String,
    paragraphs: Vec<Paragraph>,
}

#[allow(dead_code)]
impl Article {
    fn new() -> Self {
        Self {
            title: String::from(""),
            author: String::from(""),
            paragraphs: vec![],
        }
    }
}

fn read_json_typed(json_string: &str) -> Article {
    serde_json::from_str(json_string).unwrap()
}

fn convert_to_json(article: &Article) -> String {
    serde_json::to_string(article).unwrap()
}

fn main() {
    let dummy_json = r#"
        {
            "title": "The Great Gatsby",
            "author": "F. Scott Fitzgerald",
            "paragraphs": [
                {
                    "name": "chapter 1"
                },
                {
                    "name": "chapter 2"
                },
                {
                    "name": "chapter 3"
                }
            ]
        }"#;

    let parsed: Article = read_json_typed(dummy_json);
    println!(
        "\n\nThe name of the first paragraph is: '{}'",
        parsed.paragraphs[0].name
    );

    let json_string: String = convert_to_json(&parsed);
    println!("\n\nThis was the json passed:\n'{}'", json_string);
}
