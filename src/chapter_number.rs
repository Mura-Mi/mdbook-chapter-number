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
                if let Header(mut spans, usize) = first {
                    let mut new_spans = vec![Text(a.to_string() + " ")];
                    new_spans.append(&mut spans);
                    tokenized[0] = Header(new_spans, usize);
                    ch.content = markdown::generate_markdown(tokenized);
                }
            }
        }
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
