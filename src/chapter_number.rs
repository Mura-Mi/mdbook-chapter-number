pub struct ChapterNumber;

use mdbook::book::{Book,BookItem, SectionNumber};
use mdbook::BookItem::Chapter;
use mdbook::errors::Error;
use mdbook::preprocess::{Preprocessor, PreprocessorContext};

impl ChapterNumber {
    pub fn new() -> Self {
        ChapterNumber
    }

    fn process_chapter(item: &mut BookItem) {
        if let Chapter(ref mut ch) = item {
            if let Some(a) = &ch.number {
                let c = &ch.content;
                ch.content = Self::modify_content(a, c);
            }
        }
    }

    fn modify_content(a: &SectionNumber, c: &str) -> String {
        a.to_string() + " " + c
    }
}

impl Preprocessor for ChapterNumber {
    fn name(&self) -> &str {
        "chapter-number"
    }

    fn run(&self, _ctx: &PreprocessorContext, mut book: Book) -> Result<Book, Error> {
        book.for_each_mut(|item| {
            ChapterNumber::process_chapter(item)
        });
        Ok(book)
    }
}

#[cfg(test)]
mod tests {
    use mdbook::book::SectionNumber;
    use super::*;

    #[test]
    fn test_process_chapter() {
        println!("{}", ChapterNumber::modify_content(&SectionNumber(vec![1,2,3]), "test"));
    }
}
