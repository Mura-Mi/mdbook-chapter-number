use mdbook::book::{Book, BookItem};
use mdbook::BookItem::Chapter;
use mdbook::errors::Error;
use mdbook::preprocess::{Preprocessor, PreprocessorContext};
use pulldown_cmark::{CowStr, Event, HeadingLevel};
use pulldown_cmark::Tag::Heading;

pub struct ChapterNumber;

impl ChapterNumber {
    pub fn new() -> Self {
        ChapterNumber
    }

    fn process_chapter(item: &mut BookItem) {
        if let Chapter(ref mut ch) = item {
            if let Some(a) = &ch.number {
                let c = &ch.content;
                let tokenized = mdbook::utils::new_cmark_parser(c, false);

                let mut in_first_heading = false;

                let events = tokenized.map(|event|
                    {
                        match event {
                            Event::Start(Heading(HeadingLevel::H1, _, _)) => {
                                in_first_heading = true;
                                event
                            }
                            Event::Text(s) if in_first_heading => {
                                let mut new_content = String::new();
                                new_content.push_str(&a.to_string());
                                new_content.push_str(" ");
                                new_content.push_str(&s);
                                Event::Text(CowStr::from(new_content))
                            }
                            Event::End(Heading(HeadingLevel::H1, _, _)) => {
                                in_first_heading = false;
                                event
                            }
                            _ => event
                        }
                    }
                );

                let mut buf = String::with_capacity(c.len() + a.to_string().len());
                pulldown_cmark_to_cmark::cmark(events, &mut buf).expect("cmark parsing failed");

                ch.content = buf;
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
            assert_eq!(expected, ch.content);
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

    #[test]
    fn test_supports_ordered_list() {
        test_processor(
            "# Hello

1. ordered
1. list",
            Some(SectionNumber(vec![1, 2])),
            "# 1.2. Hello

1. ordered
1. list",
        );
    }
}
