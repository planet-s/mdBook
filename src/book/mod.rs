pub mod bookitem;
pub mod bookconfig;
pub mod metadata;
pub mod book;

pub use self::bookitem::{BookItem, BookItems};
pub use self::bookconfig::BookConfig;

use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::fs::{self, File};
use std::error::Error;
use std::io;
use std::io::Write;
use std::io::ErrorKind;
use std::process::Command;

use {theme, markdown, utils};
use renderer::{Renderer, HtmlHandlebars};


pub struct MDBook {
    root: PathBuf,
    dest: PathBuf,
    src: PathBuf,

    title: String,
    author: String,
    description: String,

    default_language: &'static str,
    books: HashMap<&'static str, book::Book>,

    pub content: Vec<BookItem>,
    renderer: Box<Renderer>,

    livereload: Option<String>,
}

impl MDBook {
    /// Create a new `MDBook` struct with root directory `root`
    ///
    /// - The default source directory is set to `root/src`
    /// - The default output directory is set to `root/book`
    ///
    /// They can both be changed by using [`set_src()`](#method.set_src) and [`set_dest()`](#method.set_dest)

    pub fn new(root: &Path) -> MDBook {

        if !root.exists() || !root.is_dir() {
            output!("{:?} No directory with that name", root);
        }

        MDBook {
            root: root.to_owned(),
            dest: PathBuf::from("book"),
            src: PathBuf::from("src"),

            title: String::new(),
            author: String::new(),
            description: String::new(),

            default_language: "en",
            books: HashMap::new(),

            content: vec![],
            renderer: Box::new(HtmlHandlebars::new()),

            livereload: None,
        }
    }

    /// Returns a flat depth-first iterator over the elements of the book, it returns an [BookItem enum](bookitem.html):
    /// `(section: String, bookitem: &BookItem)`
    ///
    /// ```no_run
    /// # extern crate mdbook;
    /// # use mdbook::MDBook;
    /// # use mdbook::BookItem;
    /// # use std::path::Path;
    /// # fn main() {
    /// # let mut book = MDBook::new(Path::new("mybook"));
    /// for item in book.iter() {
    ///     match item {
    ///         &BookItem::Chapter(ref section, ref chapter) => {},
    ///         &BookItem::Affix(ref chapter) => {},
    ///         &BookItem::Spacer => {},
    ///     }
    /// }
    ///
    /// // would print something like this:
    /// // 1. Chapter 1
    /// // 1.1 Sub Chapter
    /// // 1.2 Sub Chapter
    /// // 2. Chapter 2
    /// //
    /// // etc.
    /// # }
    /// ```

    pub fn iter(&self) -> BookItems {
        BookItems {
            items: &self.content[..],
            current_index: 0,
            stack: Vec::new(),
        }
    }

    /// `init()` creates some boilerplate files and directories to get you started with your book.
    ///
    /// ```text
    /// book-test/
    /// ????????? book
    /// ????????? src
    ///     ????????? chapter_1.md
    ///     ????????? SUMMARY.md
    /// ```
    ///
    /// It uses the paths given as source and output directories and adds a `SUMMARY.md` and a
    /// `chapter_1.md` to the source directory.

    pub fn init(&mut self) -> Result<(), Box<Error>> {

        debug!("[fn]: init");

        if !self.root.exists() {
            fs::create_dir_all(&self.root).unwrap();
            output!("{:?} created", &self.root);
        }

        {

            if !self.dest.exists() {
                debug!("[*]: {:?} does not exist, trying to create directory", self.dest);
                try!(fs::create_dir(&self.dest));
            }

            if !self.src.exists() {
                debug!("[*]: {:?} does not exist, trying to create directory", self.src);
                try!(fs::create_dir(&self.src));
            }

            let summary = self.src.join("SUMMARY.md");

            if !summary.exists() {

                // Summary does not exist, create it

                debug!("[*]: {:?} does not exist, trying to create SUMMARY.md", src.join("SUMMARY.md"));
                let mut f = try!(File::create(&self.src.join("SUMMARY.md")));

                debug!("[*]: Writing to SUMMARY.md");

                try!(writeln!(f, "# Summary"));
                try!(writeln!(f, ""));
                try!(writeln!(f, "- [Chapter 1](./chapter_1.md)"));
            }
        }

        // parse SUMMARY.md, and create the missing item related file
        try!(self.parse_summary());

        debug!("[*]: constructing paths for missing files");
        for item in self.iter() {
            debug!("[*]: item: {:?}", item);
            match *item {
                BookItem::Spacer => continue,
                BookItem::Chapter(_, ref ch) | BookItem::Affix(ref ch) => {
                    if ch.path != PathBuf::new() {
                        let path = self.src.join(&ch.path);

                        if !path.exists() {
                            debug!("[*]: {:?} does not exist, trying to create file", path);
                            try!(::std::fs::create_dir_all(path.parent().unwrap()));
                            let mut f = try!(File::create(path));

                            // debug!("[*]: Writing to {:?}", path);
                            try!(writeln!(f, "# {}", ch.name));
                        }
                    }
                },
            }
        }

        debug!("[*]: init done");
        Ok(())
    }

    pub fn create_gitignore(&self) {
        let gitignore = self.get_gitignore();

        if !gitignore.exists() {
            // Gitignore does not exist, create it

            // Because of `src/book/mdbook.rs#L37-L39`, `dest` will always start with `root`. If it
            // is not, `strip_prefix` will return an Error.
            if !self.get_dest().starts_with(&self.root) {
                return;
            }

            let relative = self.get_dest()
                               .strip_prefix(&self.root)
                               .expect("Destination is not relative to root.");
            let relative = relative.to_str()
                                   .expect("Path could not be yielded into a string slice.");

            debug!("[*]: {:?} does not exist, trying to create .gitignore", gitignore);

            let mut f = File::create(&gitignore).expect("Could not create file.");

            debug!("[*]: Writing to .gitignore");

            writeln!(f, "{}", relative).expect("Could not write to file.");
        }
    }

    /// The `build()` method is the one where everything happens. First it parses `SUMMARY.md` to
    /// construct the book's structure in the form of a `Vec<BookItem>` and then calls `render()`
    /// method of the current renderer.
    ///
    /// It is the renderer who generates all the output files.
    pub fn build(&mut self) -> Result<(), Box<Error>> {
        debug!("[fn]: build");

        try!(self.init());

        // Clean output directory
        try!(utils::fs::remove_dir_content(&self.dest));

        try!(self.renderer.render(&self));

        Ok(())
    }


    pub fn get_gitignore(&self) -> PathBuf {
        self.root.join(".gitignore")
    }

    pub fn copy_theme(&self) -> Result<(), Box<Error>> {
        debug!("[fn]: copy_theme");

        let theme_dir = self.src.join("theme");

        if !theme_dir.exists() {
            debug!("[*]: {:?} does not exist, trying to create directory", theme_dir);
            try!(fs::create_dir(&theme_dir));
        }

        // index.hbs
        let mut index = try!(File::create(&theme_dir.join("index.hbs")));
        try!(index.write_all(theme::INDEX));

        // book.css
        let mut css = try!(File::create(&theme_dir.join("book.css")));
        try!(css.write_all(theme::CSS));

        // favicon.png
        let mut favicon = try!(File::create(&theme_dir.join("favicon.png")));
        try!(favicon.write_all(theme::FAVICON));

        // book.js
        let mut js = try!(File::create(&theme_dir.join("book.js")));
        try!(js.write_all(theme::JS));

        // highlight.css
        let mut highlight_css = try!(File::create(&theme_dir.join("highlight.css")));
        try!(highlight_css.write_all(theme::HIGHLIGHT_CSS));

        // highlight.js
        let mut highlight_js = try!(File::create(&theme_dir.join("highlight.js")));
        try!(highlight_js.write_all(theme::HIGHLIGHT_JS));

        Ok(())
    }

    /// Parses the `book.json` file (if it exists) to extract the configuration parameters.
    /// The `book.json` file should be in the root directory of the book.
    /// The root directory is the one specified when creating a new `MDBook`
    ///
    /// ```no_run
    /// # extern crate mdbook;
    /// # use mdbook::MDBook;
    /// # use std::path::Path;
    /// # fn main() {
    /// let mut book = MDBook::new(Path::new("root_dir"));
    /// # }
    /// ```
    ///
    /// In this example, `root_dir` will be the root directory of our book and is specified in function
    /// of the current working directory by using a relative path instead of an absolute path.

    pub fn read_config(mut self) -> Self {

        let config = BookConfig::new(&self.root)
                                .read_config(&self.root)
                                .to_owned();

        // Temporary
        let mut english = book::Book::new(&config.title);

        english.mut_metadata()
               .set_description(&config.description)
               .add_author(metadata::Author::new(&config.author));

        self.books.insert("en", english);
        self.default_language = "en";

        self.title = config.title;
        self.description = config.description;
        self.author = config.author;

        self.dest = config.dest;
        self.src = config.src;

        self
    }

    /// You can change the default renderer to another one by using this method. The only requirement
    /// is for your renderer to implement the [Renderer trait](../../renderer/renderer/trait.Renderer.html)
    ///
    /// ```no_run
    /// extern crate mdbook;
    /// use mdbook::MDBook;
    /// use mdbook::renderer::HtmlHandlebars;
    /// # use std::path::Path;
    ///
    /// fn main() {
    ///     let mut book = MDBook::new(Path::new("mybook"))
    ///                         .set_renderer(Box::new(HtmlHandlebars::new()));
    ///
    ///     // In this example we replace the default renderer by the default renderer...
    ///     // Don't forget to put your renderer in a Box
    /// }
    /// ```
    ///
    /// **note:** Don't forget to put your renderer in a `Box` before passing it to `set_renderer()`

    pub fn set_renderer(mut self, renderer: Box<Renderer>) -> Self {
        self.renderer = renderer;
        self
    }

    pub fn test(&mut self) -> Result<(), Box<Error>> {
        // read in the chapters
        try!(self.parse_summary());
        for item in self.iter() {

            match *item {
                BookItem::Chapter(_, ref ch) => {
                    if ch.path != PathBuf::new() {

                        let path = self.get_src().join(&ch.path);

                        println!("[*]: Testing file: {:?}", path);

                        let output_result = Command::new("rustdoc")
                                                .arg(&path)
                                                .arg("--test")
                                                .output();
                        let output = try!(output_result);

                        if !output.status.success() {
                            return Err(Box::new(io::Error::new(ErrorKind::Other, format!(
                                            "{}\n{}",
                                            String::from_utf8_lossy(&output.stdout),
                                            String::from_utf8_lossy(&output.stderr)))) as Box<Error>);
                        }
                    }
                },
                _ => {},
            }
        }
        Ok(())
    }

    pub fn get_root(&self) -> &Path {
        &self.root
    }

    pub fn set_dest(mut self, dest: &Path) -> Self {

        // Handle absolute and relative paths
        match dest.is_absolute() {
            true => {
                self.dest = dest.to_owned();
            },
            false => {
                let dest = self.root.join(dest).to_owned();
                self.dest = dest;
            },
        }

        self
    }

    pub fn get_dest(&self) -> &Path {
        &self.dest
    }

    pub fn set_src(mut self, src: &Path) -> Self {

        // Handle absolute and relative paths
        match src.is_absolute() {
            true => {
                self.src = src.to_owned();
            },
            false => {
                let src = self.root.join(src).to_owned();
                self.src = src;
            },
        }

        self
    }

    pub fn get_src(&self) -> &Path {
        &self.src
    }

    pub fn set_title(mut self, title: &str) -> Self {
        self.title = title.to_owned();
        self
    }

    pub fn get_title(&self) -> &str {
        &self.title
    }

    pub fn set_author(mut self, author: &str) -> Self {
        self.author = author.to_owned();
        self
    }

    pub fn get_author(&self) -> &str {
        &self.author
    }

    pub fn set_description(mut self, description: &str) -> Self {
        self.description = description.to_owned();
        self
    }

    pub fn get_description(&self) -> &str {
        &self.description
    }

    pub fn set_livereload(&mut self, livereload: String) -> &mut Self {
        self.livereload = Some(livereload);
        self
    }

    pub fn unset_livereload(&mut self) -> &Self {
        self.livereload = None;
        self
    }

    pub fn get_livereload(&self) -> Option<&String> {
        match self.livereload {
            Some(ref livereload) => Some(&livereload),
            None => None,
        }
    }

    // Construct book
    fn parse_summary(&mut self) -> Result<(), Box<Error>> {
        // When append becomes stable, use self.content.append() ...
        self.content = try!(markdown::summary::construct_bookitems(&self.src.join("SUMMARY.md")));
        Ok(())
    }
}
