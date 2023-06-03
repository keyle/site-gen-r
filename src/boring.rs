use std::fs;

use crate::{post::Post, settings::Settings};

pub fn gen_sitemap(posts: &Vec<Post>, settings: &Settings) {
    let mut contents = String::from(
        r#"<?xml version="1.0" encoding="utf-8" standalone="yes"?>
<urlset xmlns="http://www.sitemaps.org/schemas/sitemap/0.9"
  xmlns:xhtml="http://www.w3.org/1999/xhtml">"#,
    );

    for p in posts {
        contents = format!(
            "{}<url><loc>{}</loc><lastmod>{}</lastmod></url>\n",
            contents, p.url, p.pub_date
        );
    }
    contents = format!("{}</urlset>\n", contents);
    // save the sitemap.xml at the web root
    let file_path = format!("{}/sitemap.xml", &settings.workdir);
    fs::write(file_path, &contents).expect("could not write sitemap.xml!");
}

pub fn gen_rssfeed(posts: &Vec<Post>, settings: &Settings) {
    //
    let mut contents = String::from(
        r#"<?xml version="1.0" encoding="utf-8" standalone="yes"?>
<rss version="2.0" xmlns:atom="http://www.w3.org/2005/Atom">
  <channel>
    <title>NobenLog</title>
    <link>https://noben.org/blog/</link>
    <description>Recent content on NobenLog</description>
    <generator>site-gen-rust -- https://github.com/keyle/site-gen-r</generator>
    <language>en-us</language>"#,
    );

    for p in posts {
        contents = format!(
            "{}<item><title>{}</title><link>{}</link><pubDate>{}</pubDate><guid>{}</guid><description>{}</description></item>\n",
            contents, p.title, p.url, p.pub_date, p.url, p.description
        );
    }
    contents = format!("{}</channel></rss>\n", contents);
    // save the index.xml (RSS) at the web root
    let file_path = format!("{}/index.xml", &settings.workdir);
    fs::write(file_path, &contents).expect("could not write rss xml!");
}
