pub struct ChapterNumber;

use markdown::Block;
use markdown::Block::Header;
use markdown::Span::Text;
use mdbook::book::{Book, BookItem, SectionNumber};
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
                let mut tokenized = markdown::tokenize(c);
                let mut first: Block = tokenized[0].clone();
                if let Header(spans, usize) = first {
                    if let Some(Text(txt)) = spans.first() {
                        let mut new_spans = vec![Text(a.to_string() + " " + txt)];
                        tokenized[0] = Header(new_spans, usize);
                    }
                    ch.content = markdown::generate_markdown(tokenized);
                }

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
