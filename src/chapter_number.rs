use markdown::Block::Header;
use markdown::Span::Text;
use mdbook::book::{Book, BookItem};
use mdbook::BookItem::Chapter;
use mdbook::errors::Error;
use mdbook::preprocess::{Preprocessor, PreprocessorContext};

pub struct ChapterNumber;

impl ChapterNumber {
    pub fn new() -> Self {
        ChapterNumber
    }

    fn process_chapter(item: &mut BookItem) {
        if let Chapter(ref mut ch) = item {
            if let Some(a) = &ch.number {
                let c = &ch.content;
                let mut tokenized = markdown::tokenize(c);
                if let Some(Header(spans, usize)) = tokenized.first() {
                    let mut new_spans = vec![Text(a.to_string() + " ")];
                    new_spans.append(&mut spans.clone());
                    tokenized[0] = Header(new_spans, *usize);
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
        book.for_each_mut(|item| ChapterNumber::process_chapter(item));
        Ok(book)
    }
}

#[cfg(test)]
mod test {
    use mdbook::book::{Chapter, SectionNumber};
    use mdbook::BookItem;
    use pretty_assertions::assert_eq;

    use crate::ChapterNumber;

    fn sut_chapter(content: &str, chapter_number: Option<SectionNumber>) -> BookItem {
        BookItem::Chapter(Chapter {
            name: "".to_string(),
            content: content.to_string(),
            number: chapter_number,
            sub_items: vec![],
            path: None,
            source_path: None,
            parent_names: vec![],
        })
    }

    fn test_processor(source: &str, section_number: Option<SectionNumber>, expected: &str) {
        let mut sut = sut_chapter(source, section_number);
        ChapterNumber::process_chapter(&mut sut);
        if let BookItem::Chapter(ch) = sut {
            assert_eq!(ch.content, expected);
        }
    }

    #[test]
    fn test_no_change() {
        let content = "
# Hello

this is dummy text.
";
        test_processor(content, None, content);
    }

    #[test]
    fn test_appends_chapter_number() {
        test_processor(
            "# Hello

this is dummy text.",
            Some(SectionNumber(vec![1, 2])),
            "# 1.2. Hello

this is dummy text.",
        );
    }
}
