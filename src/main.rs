use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct Paragraph {
    name: String,
}

#[derive(Serialize, Deserialize)]
struct Article {
    article: String,
    author: String,
    paragraph: Vec<Paragraph>,
}

fn main() {
    let json = r#"
    {
        "article": "How to work with JSON in Rust",
        "author": "Nicolas",
        "paragraph": [
            {
                "name": "Starting sentence"
            },
            {
                "name": "Body of the paragraph"
            },
            {
                "name": "End of the paragraph"
            }
        ]
    }"#;

    let parsed: Result<Article, serde_json::Error> = serde_json::from_str(json);

    match parsed {
        Ok(article) => println!(
            "\n\nThe name of the first article is : {}",
            article.paragraph[0].name
        ),
        Err(e) => println!("Error: {}", e),
    }
}
