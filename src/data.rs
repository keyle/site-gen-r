use pulldown_cmark::{html, Options, Parser};
use scraper::{Html, Selector};
use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Debug, Serialize, Deserialize)]
pub struct Settings {
    pub workdir: String,
    pub template: String,
    pub contenttag: String,
    pub titletag: String,
    pub descriptiontag: String,
    pub keywordstag: String,
}

#[derive(Debug)]
pub struct Post {
    pub path: String,
    pub folder: String,
    pub markdown: String,
    pub html: String,
}

impl Post {
    pub fn markdown_to_html(&mut self) {
        let contents = fs::read_to_string(self.path.clone()).expect("unable to read file");
        self.markdown = contents.clone();
        // convert contents to html
        let mut options = Options::empty();
        options.insert(Options::ENABLE_STRIKETHROUGH);
        options.insert(Options::ENABLE_FOOTNOTES);
        options.insert(Options::ENABLE_TABLES);
        let parser = Parser::new_ext(&contents, options);
        let mut html_out = String::new();
        html::push_html(&mut html_out, parser);
        // html
        self.html = html_out;
    }

    pub fn mangle_template(&mut self, template: &String, settings: &Settings) {
        let mut contents = template.clone();
        let is_blog_post = self.html.contains("<sub>");
        let html = Html::parse_document(&self.html);

        // TODO to generate the sitemap.xml
        // TODO to generate the index.xml (rss)
        // TODO put the rss in the template
        // TODO put the blog index on the homepage
        // FIXME @hack we purposefully named our index z-index to be last in the alphabet to have processed everything else prior!
        // Ideally this should take another pass, rather than rely on the order.

        let x_title = Selector::parse("x-title")
            .expect("ERROR Could not extract <x-title> from supposed BLOG post");
        let title = html
            .select(&x_title)
            .next()
            .unwrap_or_else(|| {
                panic!(
                    "could not parse <x-title> from html - it is required. HTML: {}",
                    &self.html
                )
            })
            .inner_html();

        let description: String;

        if is_blog_post {
            contents = contents.replace("<body>", "<body class='blog'>"); // apply different css
            let x_date = Selector::parse("sub")
                .expect("ERROR Could not extract <sub> (pubdate) from supposed blog post");
            let _pubdate = html.select(&x_date).next().unwrap().inner_html(); // TODO impl pubdate in RSS and index page
            description = title.clone(); // take the title as description
        } else {
            let x_desc = Selector::parse("x-desc")
                .expect("ERROR Could not extract x-desc from supposed PRODUCT post");
            description = html
                .select(&x_desc)
                .next()
                .expect("could not parse <x-desc> description from html")
                .inner_html();
        }

        let x_tags = Selector::parse("x-tags")
            .expect("ERROR Could not extract <x-tags> from supposed blog post");
        let tags = html.select(&x_tags).next().unwrap().inner_html();

        contents = contents.replace(&settings.titletag, &title);
        contents = contents.replace(&settings.keywordstag, &tags);
        contents = contents.replace(&settings.descriptiontag, &description);
        contents = contents.replace(&settings.contenttag, &self.html);

        self.html = contents;
    }

    pub fn save_html(&mut self) {
        let file_path = format!("{}/index.html", &self.folder);
        fs::write(file_path, &self.html).expect("could not write to html file");
    }
}
