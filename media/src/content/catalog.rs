use crate::content::media::Media;

#[derive(Debug)]
pub struct Catalog {
    items: Vec<Media>,
}

impl Catalog {
    pub fn new() -> Self {
        Catalog { items: Vec::new() }
    }

    pub fn add(&mut self, media: Media) {
        self.items.push(media);
    }

    pub fn describe(&self) -> String {
        self.items
            .iter()
            .map(|m| m.description())
            .collect::<Vec<_>>()
            .join("\n")
    }

    pub fn get_by_index(&self, index: usize) -> Option<&Media> {
        if self.items.len() > index {
            Some(&self.items[index])
        } else {
            None
        }
    }
}
