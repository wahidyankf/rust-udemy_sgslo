use crate::content::catalog::Catalog;
use crate::content::media::Media;

mod content;

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
