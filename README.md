### Site generator

This is a static website generator, that includes support for a blog, with vanity urls, keyword tags, RSS, sitemap.xml.

Like pretty much everything made in rust, it was built to learn rust...

Note, you'll find the same repository `site-gen` written in Swift, this one however went way further...

### How it works

it takes a .settings.json of this format...

```
{
    "workdir": "/Users/name/Documents/Code/website/public",
    "webroot": "https://website.org",
    "template": "./template/template.html",
    "templateindex": "./template/template-index.html",
    "contenttag": "{{%%content%%}}",
    "titletag": "{{%%title%%}}",
    "descriptiontag" : "{{%%description%%}}",
    "keywordstag": "{{%%keywords%%}}"
}
```

The site generator will then 

- walk recursively the `workdir` looking for `.md` (markdown) files, 
- converts them to HTML
- insert them in `template`
- replacing the `contenttag` with the HTML
- update `descriptiontag` with the contents of `<x-desc>` (custom valid HTML5 tag in the markdown)
- same with the `keywordstag` with the contents of `<x-tags>` (hidden)
- same with the `titletag` with the contents of `<x-title>`
- it will do the same with the templateindex if the markdown contains <x-index/> (as an indicator of being the index)

This custom HTML5 tag gymnastic is to avoid having metadata json files around, or breaking the valid markdown format (like Hugo does). In retrospect, I have mixed feelings about it.
  
NOTE: the content will be placed in situe. So if the website places a markdown in `/folder` it will be `/folder/index.html` so that you're in control of the whole website structure and vanity urls.
  
### Blogging
  
The blogging system works pretty much the same, except I use `<x-blog-title>` instead of `<x-title>` to tell the generator that this is a blog post. 

Blog posts will automatically have a `blog` class on the `<body>` to style the blog differently, as needed.

Additionally we parse `sub` for the RSS `pubDate` and to be sorted on the index page.

It will be included in the RSS and linked from the homepage. Note that this last part is pretty much custom to suit my own needs, but could be abstracted out further.

### Running

#### Debug

`Cargo run`

#### Release 

```
Cargo build --release
./target/release/site-gen-rust
```

### See also

* `site-gen-ocaml` a remake of this in [OCaml](https://github.com/keyle/site-gen-ocaml)
* `site-gen-c` a remake of this in [modern C](https://github.com/keyle/site-gen-c)

### Benchmarks

For fun, I ran some stats. This compares the different builds. Note that both were written pretty naively, I was new to Rust and I am new to OCaml. So, take those with a grain of salt. However the implementation logic is very linear and very similar...

The C version could be much faster, I'm doing a ton of allocations that aren't strictly needed for the sake of getting it done. To be optimised.

#### Method

`hyperfine --warmup 5 ./sitegen...    # ran for approx 400-600 iterations.` 

#### C version

`Time (mean ± σ):       3.0 ms ±   0.2 ms    [User: 1.1 ms, System: 1.8 ms]`

#### Rust version

`Time (mean ± σ):       4.3 ms ±   0.2 ms    [User: 2.1 ms, System: 2.0 ms]`

#### OCaml version

`Time (mean ± σ):       6.4 ms ±   0.2 ms    [User: 3.5 ms, System: 2.8 ms]`

#### OCaml bytecode version

`Time (mean ± σ):      21.2 ms ±   0.2 ms    [User: 17.6 ms, System: 3.3 ms]`

