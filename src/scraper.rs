pub mod tfl_status {
    // This module contains the webscraper for the tfl website.
    //
    //
    use crate::lines::Tube;
    use reqwest::{Client, Error};
    #[tokio::main]

    pub async fn scrape() -> Result<Vec<Tube::LineStatus>, Error> {
        //
        let client = Client::new();
        let html = client
            .get("https://tfl.gov.uk/tube-dlr-overground/status/")
            .send()
            .await?
            .text()
            .await?;
        let imp_lines = important_lines(&html);
        let tag_blocks = split_by_li_tag(imp_lines);
        let mut line_statuses: Vec<Tube::LineStatus> = Vec::new();
        for tag_block in tag_blocks {
            if let Some(status) = tag_block.get_line_tube_line_status() {
                line_statuses.push(status);
            }
        }
        // Find if a line is missing, and then deal with this somehow?
        //
        return Ok(line_statuses);
    }
    #[derive(Debug)]
    struct LiTag<'html> {
        lines_list: Vec<&'html str>,
    }

    fn important_lines(tfl_status_page_html: &str) -> Vec<&str> {
        // The tfl line status page has lots of additonal information,
        // which is not required. And makes the scraping harder.
        //
        // Isolating the lines of interest is slower, however, it
        // is easier, and maintians the same complexity.

        let mut line_store: Vec<&str> = Vec::new();

        let mut keep = false;
        let mut times_seen = 0;
        for line in tfl_status_page_html.lines() {
            // The live map section is just above the delayed lines for the day
            // The times seen flag is used to ensure that more scraping is not
            // done on other metadata, lower down in the page that is not required.
            if line.contains("<span>View live map</span>") {
                if times_seen < 1 {
                    // There are many li divs before the section for the lizzy line
                    // manually append one so that the li tag picks it up.
                    keep = true;
                }
                times_seen += 1;
                //
                //
                //In a drop down menu below the line status, there is a list of
                //Station information, which is not required, so we stop collecting.
            } else if line.contains("<span>Burnham (Berks) Rail Station</span>") {
                keep = false;
            }
            if keep {
                line_store.push(line);
            }
        }
        return line_store;
    }
    fn split_by_li_tag<'html>(line_list: Vec<&'html str>) -> Vec<LiTag> {
        // Pass by reference the lines of htm which are interesting.
        //
        // The <li> tags are used to delimit the information for each line.
        let mut tag_blocks: Vec<LiTag> = Vec::new();
        let mut buffer: Vec<&'html str> = Vec::new();
        let mut is_collecting = false;
        for line in line_list {
            // manage buffer.
            if line.contains("<li") {
                is_collecting = true;
            } else if line.contains("</li>") {
                is_collecting = false;
                tag_blocks.push(LiTag {
                    lines_list: buffer.clone(),
                });
                buffer.clear();
            }
            // collect data.
            if is_collecting {
                buffer.push(&line);
            }
        }
        return tag_blocks;
    }
    impl<'html> LiTag<'html> {
        fn get_line_tube_line_status(&self) -> Option<Tube::LineStatus> {
            // The <li> blocks contain all of the information for a single line.
            //
            // The <span> tag contains the name of the line.
            // The <br /> tag contains the short status of the line.
            // The <p> tag contains the long status of the line.
            //
            // The function strips the html tags and returns a LineStatus struct.
            let mut line_type: Option<Tube::Line> = None;
            let mut short_status = String::new();
            let mut long_status = String::new();
            for line in self.lines_list.iter() {
                if line.contains("<span>") && line.contains("</span>") {
                    line_type = Tube::Line::build_from_str(line);
                } else if line.contains("<br />") {
                    if let Some(tag_ind) = line.find("<br />") {
                        short_status = line[..tag_ind].to_string();
                    } else {
                        return None;
                    };
                } else if line.contains("<p>") {
                    long_status = line.replace("<p>", "").replace("</p>", "");
                };
            }
            match line_type {
                Some(line) => Some(Tube::LineStatus {
                    name: line,
                    short: short_status,
                    long: long_status,
                }),
                None => None,
            }
        }
    }
}
