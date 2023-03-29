use html_parser::Dom;
use reqwest::{Client, Error};

pub mod scrape {
    #[tokio::main]

    pub async fn scrape() -> Result<(), reqwest::Error> {
        //

        let client = reqwest::Client::new();
        let html = client
            .get("https://tfl.gov.uk/tube-dlr-overground/status/")
            .send()
            .await?
            .text()
            .await?;
        let imp_lines = important_lines(&html);

        return Ok(());
    }
    fn important_lines(tfl_status_page_html: &str) -> Vec<&str> {
        // The tfl website is big with lots of html shite, just look for the important,
        // lines which include only the tube lines.

        let mut line_store: Vec<&str> = Vec::new();

        let mut keep = false;
        let mut times_seen = 0;
        for line in tfl_status_page_html.lines() {
            if line.contains("<span>Elizabeth line</span>") {
                if times_seen < 1 {
                    keep = true;
                }
                times_seen += 1;
            } else if line.contains("<span>Burnham (Berks) Rail Station</span>") {
                keep = false;
            }
            if keep {
                line_store.push(line)
            }
        }
        return line_store;
    }


    fn get_delay_description()
}
