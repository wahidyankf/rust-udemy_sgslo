#[derive(Debug)]
enum Media {
    Book { title: String, author: String },
    Movie { title: String, director: String },
    Audiobook { title: String },
    Podcast(u32),
    Placeholder,
}

impl Media {
    fn description(&self) -> String {
        match self {
            Media::Book { title, author } => {
                format!("Book: {} {}", title, author)
            }
            Media::Movie { title, director } => {
                format!("Movie: {} {}", title, director)
            }
            Media::Audiobook { title } => {
                format!("Audiobook: {}", title)
            }
            Media::Podcast(id) => {
                format!("Podcast: {}", id)
            }
            Media::Placeholder => {
                format!("Placeholder")
            }
        }
    }
}

#[derive(Debug)]
struct Catalog {
    items: Vec<Media>,
}

impl Catalog {
    fn new() -> Self {
        Catalog { items: Vec::new() }
    }

    fn add(&mut self, media: Media) {
        self.items.push(media);
    }

    fn describe(&self) -> String {
        self.items
            .iter()
            .map(|m| m.description())
            .collect::<Vec<_>>()
            .join("\n")
    }

    fn get_by_index(&self, index: usize) -> Option<&Media> {
        if self.items.len() > index {
            Some(&self.items[index])
        } else {
            None
        }
    }
}

fn main() {
    let audiobook = Media::Audiobook {
        title: "An Audiobook".to_string(),
    };
    let good_movie = Media::Movie {
        title: String::from("Good Movie"),
        director: String::from("Good Director"),
    };
    let bad_book = Media::Book {
        title: "Bad Book".to_string(),
        author: "Bad Author".to_string(),
    };
    let podcast = Media::Podcast(42);

    let mut catalog = Catalog::new();
    let placeholder = Media::Placeholder;

    catalog.add(audiobook);
    catalog.add(good_movie);
    catalog.add(bad_book);
    catalog.add(podcast);
    catalog.add(placeholder);

    println!("Catalog: \n{}", catalog.describe());
    println!("-------");

    let item = catalog.get_by_index(0);

    let placeholder = Media::Placeholder;
    println!("{:#?}", item.unwrap_or(&placeholder));
}
