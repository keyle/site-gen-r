use pulldown_cmark::{html, Options, Parser};
use std::{fs, ops::Add, path::Path};
use walkdir::WalkDir;

mod data;
use data::*;

fn main() {
    let settingsjson = fs::read_to_string(".settings.json").expect("unable to read file");
    let settings: Settings = serde_json::from_str(&settingsjson).unwrap();
    println!("Loaded settings");
    let mut posts: Vec<Post> = Vec::new();

    WalkDir::new(&settings.workdir)
        .into_iter()
        .map(walkdir::Result::unwrap)
        .filter(|x| x.file_type().is_file())
        .filter(|x| !x.file_name().to_str().unwrap().starts_with("."))
        .filter(|x| x.path().extension().unwrap().to_ascii_lowercase() == "md")
        .for_each(|x| {
            println!("MD found {:?}", x);
            // fetch meta.json, fill in metadata struct
            let p = Path::new(x.path());
            let folder = p
                .parent()
                .expect("could not resolve folder")
                .to_str()
                .unwrap()
                .to_string();
            let metajson = fs::read_to_string(folder.clone() + "/" + &settings.metadatafilename)
                .expect("could not load metadata");
            let metadata: Metadata =
                serde_json::from_str(&metajson).expect("could not parse metadata json");
            posts.push(Post {
                path: x.path().to_str().unwrap().to_lowercase(),
                folder,
                markdown: String::new(),
                metadata: Some(metadata),
                html: String::new(),
            });
        });

    convert_markdown_html(&mut posts);
    write_html(&settings, &posts);
}

fn write_html(settings: &Settings, posts: &Vec<Post>) {
    let template = fs::read_to_string(settings.template.clone()).expect("could not load template!");
    for post in posts {
        let file_path = post.folder.clone().add("/index.html");
        // template mangling with content
        let mut contents = template.clone();
        let metadata = &post.metadata.as_ref().unwrap();
        contents = contents.replace(&settings.titletag, &metadata.title);
        contents = contents.replace(&settings.descriptiontag, &metadata.description);
        contents = contents.replace(&settings.keywordstag, &metadata.keywords.join(", "));
        contents = contents.replace(&settings.contenttag, &post.html);
        fs::write(file_path, &contents).expect("could not write to html file");
    }
}

fn convert_markdown_html(posts: &mut Vec<Post>) {
    for post in posts {
        // read markdown file contents
        let contents = fs::read_to_string(post.path.clone()).expect("unable to read file");
        post.markdown = contents.clone();
        // convert contents to html
        let mut options = Options::empty();
        options.insert(Options::ENABLE_STRIKETHROUGH);
        options.insert(Options::ENABLE_FOOTNOTES);
        options.insert(Options::ENABLE_TABLES);
        let parser = Parser::new_ext(&contents, options);
        let mut html_out = String::new();
        html::push_html(&mut html_out, parser);
        // html
        post.html = html_out;
    }
}
