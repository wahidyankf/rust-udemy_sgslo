#[derive(Debug)]
enum Media {
    Book { title: String, author: String },
    Movie { title: String, director: String },
    Audiobook { title: String },
}

impl Media {
    fn description(&self) -> String {
        match self {
            Media::Book { title, author } => {
                format!("Book: {} {}", title, author)
            }
            Media::Movie { title, director } => {
                format!("Movie: {} by {}", title, director)
            }
            Media::Audiobook { title } => {
                format!("Audiobook: {}", title)
            }
        }
    }
}

fn print_media(media: Media) {
    println!("{:?}", media);
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

    audiobook.description();
    good_movie.description();
    bad_book.description();

    print_media(good_movie);
    print_media(bad_book);
    print_media(audiobook);
}
