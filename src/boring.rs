use std::fs;

use chrono::NaiveDate;

use crate::{post::Post, settings::Settings};

pub fn gen_sitemap(posts: &[Post], settings: &Settings) {
    let mut contents = String::from(
        r#"<?xml version="1.0" encoding="utf-8" standalone="yes"?>
<urlset xmlns="http://www.sitemaps.org/schemas/sitemap/0.9"
  xmlns:xhtml="http://www.w3.org/1999/xhtml">"#,
    );

    // note: the order doesn't matter in sitemap.xml
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

pub fn gen_rssfeed(posts: &[Post], settings: &Settings) {
    // @hack we purposefully named our index z-index to be last in the alphabet to have processed ever Post prior!
    // Ideally this should take another pass, rather than rely on the order.
    let mut contents = String::from(
        r#"<?xml version="1.0" encoding="utf-8" standalone="yes"?>
<rss version="2.0" xmlns:atom="http://www.w3.org/2005/Atom">
  <channel>
    <title>NobenLog</title>
    <link>https://noben.org/blog/</link>
    <description>Recent content on NobenLog</description>
    <generator>site-gen-rust -- https://github.com/keyle/site-gen-rust</generator>
    <language>en-us</language>"#,
    );

    let mut sorted = posts.to_vec(); // clone to mutable list of posts for ordering
    sorted.sort_by(|a, b| b.pub_date.clone().cmp(&a.pub_date));

    sorted.into_iter().filter(|p| p.is_blog).for_each(|p| {
    contents = format!(
            "{}<item><title>{}</title><link>{}</link><pubDate>{}</pubDate><guid>{}</guid><description><![CDATA[ {} ]]></description></item>\n",
            contents, p.title, p.url, p.pub_date, p.url, p.description
        );
    });
    contents = format!("{}</channel></rss>\n", contents);
    // save the index.xml (RSS) at the web root
    let file_path = format!("{}/index.xml", &settings.workdir);
    fs::write(file_path, &contents).expect("could not write rss xml!");
}

pub fn gen_blog_index(posts: &[Post], settings: &Settings) {
    let file_path = format!("{}/index.html", &settings.workdir);
    let index_html = fs::read_to_string(&file_path).expect("could not load index html!");

    let mut contents = String::from("<table>");

    let mut sorted = posts.to_vec(); // clone to mutable list of posts for ordering
    sorted.sort_by(|a, b| b.pub_date.clone().cmp(&a.pub_date));

    sorted.into_iter().filter(|p| p.is_blog).for_each(|p| {
        contents = format!(
            "{}<tr><td>{}</td><td><a href='{}'>{}</a></td><td>&nbsp;</td>",
            contents,
            blog_date_from(&p.pub_date),
            p.vanity,
            p.title
        );
    });

    contents = format!("{}</table>", contents);

    let new_index = index_html.replace("<x-blog-index/>", &contents);
    fs::write(&file_path, new_index).expect("could not write rss xml!");
}

fn blog_date_from(ymd: &str) -> String {
    let t = NaiveDate::parse_from_str(ymd, "%Y-%m-%d")
        .expect("ERROR Could not parse date for blog index");
    t.format("%b %d, %Y").to_string() // May 14, 2023
}
