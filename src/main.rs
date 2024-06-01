fn main() {
    let response = reqwest::blocking::get("_bla_url_");
    let html_content = response.unwrap().text().unwrap();
    let document = scraper::Html::parse_document(&html_content);
    let html_questions_selector = scraper::Selector::parse("span.notable-td.prompt").unwrap();
    let html_questions = document.select(&html_questions_selector);
    for question in html_questions {
        println!("{:?}", question)
    }
}
