use std::fs;
use walkdir::WalkDir;

mod data;
use data::*;

fn main() {
    let settingsjson = fs::read_to_string(".settings.json").expect("unable to read file");
    let settings: Settings = serde_json::from_str(&settingsjson).unwrap();
    dbg!(&settings);

    let template = fs::read_to_string(settings.template.clone())
        .expect("could not load template!")
        .to_string();

    let mut posts: Vec<Post> = Vec::new();
    WalkDir::new(&settings.workdir)
        .into_iter()
        .map(walkdir::Result::unwrap)
        .filter(|x| x.file_type().is_file())
        .filter(|x| !x.file_name().to_str().unwrap().starts_with("."))
        .filter(|x| x.path().extension().unwrap().to_ascii_lowercase() == "md")
        .for_each(|x| {
            dbg!(&x);
            // fetch meta.json, fill in metadata struct
            let folder = x
                .path()
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

    posts.iter_mut().for_each(|x| {
        x.markdown_to_html();
        x.mangle_template(&template, &settings);
        x.save_html();
    });
}
