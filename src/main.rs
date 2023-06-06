use std::fs;
use walkdir::WalkDir;

mod settings;
use settings::*;

mod post;
use post::*;

mod boring;

fn main() {
    let settingsjson = fs::read_to_string(".settings.json").expect("unable to read file");
    let settings: Settings = serde_json::from_str(&settingsjson).unwrap();

    let template = fs::read_to_string(settings.template.clone()).expect("could not load template!");

    let mut posts: Vec<Post> = Vec::new();

    WalkDir::new(&settings.workdir)
        .sort_by_file_name()
        .into_iter()
        .map(walkdir::Result::unwrap)
        .filter(|x| x.file_type().is_file())
        .filter(|x| !x.file_name().to_str().unwrap().starts_with('.'))
        .filter(|x| x.path().extension().unwrap().to_ascii_lowercase() == "md")
        .for_each(|x| {
            let folder = x
                .path()
                .parent()
                .expect("could not resolve folder")
                .to_str()
                .unwrap()
                .to_string();
            println!("Found .md at {}", &folder);
            posts.push(Post {
                path: x.path().to_str().unwrap().to_lowercase(),
                folder,
                markdown: String::new(),
                html: String::new(),
                is_blog: false,
                title: String::new(),
                pub_date: String::new(),
                description: String::new(),
                url: String::new(),
                vanity: String::new(),
                tags: vec![],
            });
        });

    posts.iter_mut().for_each(|x| {
        x.markdown_to_html();
        x.mangle_template(&template, &settings);
        x.save_html();
    });

    // @later  check that the screenshot show up when served online with url ending / and not
    boring::gen_sitemap(&posts, &settings);

    boring::gen_rssfeed(&posts, &settings);

    boring::gen_blog_index(&posts, &settings);

    println!("Site generated.");
}
