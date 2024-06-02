use std::{
    error::Error,
    fs::File,
    io::{BufWriter, Write},
};

use reqwest::Client;
use scraper::Html;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let document = get_document().await?;
    let questions_part1 = get_questions(&document, 1).await?;
    let questions_part2 = get_questions(&document, 2).await?;
    let questions = vec![questions_part1, questions_part2];
    write_questions_to_file(questions).await?;
    Ok(())
}

async fn get_document() -> Result<Html, Box<dyn Error>> {
    let client = Client::new();
    let res = client
        .get("https://{some-adhd-website-with-questions}")
        .header(
            "User-Agent",
            "Thunder Client (https://www.thunderclient.com)",
        )
        .send()
        .await?;
    let html_content = res.text().await?;
    let document = Html::parse_document(&html_content);
    Ok(document)
}

async fn get_questions(document: &scraper::Html, part: i32) -> Result<Vec<String>, Box<dyn Error>> {
    let questions_selector = format!(
        "div.notable-table.vertical:nth-child({part}) span.notable-td.prompt > span:nth-child(2)"
    );
    let questions_selector = scraper::Selector::parse(&questions_selector).unwrap();
    let questions_html = document.select(&questions_selector);
    let mut questions: Vec<String> = vec![];
    for question in questions_html {
        let question_text = question.inner_html();
        questions.push(question_text);
    }
    Ok(questions)
}

async fn write_questions_to_file(questions: Vec<Vec<String>>) -> Result<(), Box<dyn Error>> {
    let file = File::create("questions.json")?;
    let mut writer = BufWriter::new(file);
    serde_json::to_writer(&mut writer, &questions)?;
    writer.flush()?;
    Ok(())
}
