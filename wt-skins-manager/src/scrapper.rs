use std::fmt::Display;
use std::sync::Arc;

use headless_chrome::{Browser, LaunchOptions};
use scraper::{Html, Selector};

#[derive(Debug)]
pub struct Camouflage {
    pub author: String,
    pub date: String,
    pub image: String,
    pub download: String,
}

impl Camouflage {
    pub fn new(author: String, date: String, image: String, download: String) -> Self {
        Self {
            author,
            date,
            image,
            download,
        }
    }
}

impl Display for Camouflage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Author: {}\nDate: {}\nImage: {}\nDownload: {}",
            self.author, self.date, self.image, self.download
        )
    }
}

pub fn get_camo_url() -> String {
    println!("Please enter the camouflage URL:");
    let mut url = String::new();
    std::io::stdin()
        .read_line(&mut url)
        .expect("Failed to read line");

    let url = url.trim();

    // Si l'url n'a pas la structure suivante: https://live.warthunder.com/post/<numéros>/en/, on recommence
    if !url.starts_with("https://live.warthunder.com/post/") || !url.ends_with("/en/") {
        println!("The URL should follow this structure: https://live.warthunder.com/post/<numbers>/en/");
        return get_camo_url();
    }

    url.to_string()
}

pub struct Scrapper {
    browser: Browser,
    tab: Arc<headless_chrome::Tab>,
    document: Html,
    camouflage_selector: Selector,
    nickname_selector: Selector,
    date_selector: Selector,
    image_selector: Selector,
    multiple_image_selector: Selector,
    download_selector: Selector,
}

impl Scrapper {
    pub fn new(url: &str) -> Self {
        let browser = Browser::new(
            LaunchOptions::default_builder()
                .headless(true)
                .build()
                .unwrap(),
        )
            .unwrap();
        let tab = browser.new_tab().unwrap();
        tab.navigate_to(url).unwrap();
        tab.wait_until_navigated().unwrap();

        let html = tab.get_content().unwrap();
        let document = Html::parse_document(&html);
        let camouflage_selector = Selector::parse("div.wrapper.camouflage").unwrap();
        let nickname_selector = Selector::parse("a.nickname").unwrap();
        let date_selector = Selector::parse("a.date").unwrap();
        let image_selector = Selector::parse("div.image img").unwrap();
        let multiple_image_selector = Selector::parse("div.multiple_images.two_lines img").unwrap();
        let download_selector = Selector::parse("a.downloads.button_item").unwrap();

        Self {
            browser,
            tab,
            document,
            camouflage_selector,
            nickname_selector,
            date_selector,
            image_selector,
            multiple_image_selector,
            download_selector,
        }
    }

    pub fn get_camouflage(&mut self) -> Option<Camouflage> {
        let mut camouflage = Camouflage::new("".to_string(), "".to_string(), "".to_string(), "".to_string());

        for camo in self.document.select(&self.camouflage_selector) {
            camouflage.author = camo
                .select(&self.nickname_selector)
                .next()
                .map(|el| el.inner_html())
                .unwrap_or("N/A".to_string());

            camouflage.date = camo
                .select(&self.date_selector)
                .next()
                .map(|el| el.inner_html())
                .unwrap_or("N/A".to_string());

            camouflage.image = camo
                .select(&self.image_selector)
                .next()
                .map(|el| el.value().attr("src").unwrap_or("N/A").to_string())
                .unwrap_or("N/A".to_string());

            camouflage.download = camo
                .select(&self.download_selector)
                .next()
                .map(|el| el.value().attr("href").unwrap_or("N/A").to_string())
                .unwrap_or("N/A".to_string());

            // let multiple_images = camo
            //     .select(&self.multiple_image_selector)
            //     .map(|el| el.value().attr("src").unwrap_or("N/A").to_string())
            //     .collect::<Vec<String>>();
        }
        Some(camouflage)
    }
}
