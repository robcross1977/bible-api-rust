use crate::book::{get_params, get_title};
use regex::{Captures, Regex};

/// The SearchType enum exists to identify the type of a bible search.
/// - Book (ex: Job)
/// - Chapter (ex: Job 1)
/// - Verse (ex: Job 1:2)
/// - VerseRange (ex: Job 1:2-3)
#[derive(Debug, PartialEq)]
pub enum SearchType {
    Book,
    Chapter,
    Verse,
    VerseRange,
}

#[derive(Debug, PartialEq)]
pub struct BookParams {
    pub search_type: SearchType,
    pub title: String,
    pub chapter: Option<u8>,
    pub verse_start: Option<u8>,
    pub verse_end: Option<u8>,
}

/// The get_search_params function takes the search query, gets the params
/// portion of the query (takes off the book), and then runs the regex to
/// determine the search type and finally builds an dreturns a BookParams
pub fn get_search_params(query: &str) -> Option<BookParams> {
    // Get the title of the book
    let title = get_title(query)?;

    // Get the params portion of the query. If there are no params, then
    // return the book. We know the title is here if we get this far
    // so we know that it is safe to build and return a book object.
    let params = match get_params(query) {
        Some(p) => p,
        None => return Some(get_book(&title)),
    };

    // If the search matches a verse range, then return a verse range type BookParams
    if let Some(verse_range) = get_verse_range(&title, &params) {
        return Some(verse_range);
    }

    // If the search matches a verse, then return a verse type BookParams
    if let Some(verse) = get_verse(&title, &params) {
        return Some(verse);
    }

    // If the search matches a chapter, then return a chapter type BookParams
    if let Some(chapter) = get_chapter(&title, &params) {
        return Some(chapter);
    }

    // If nothing has matched this far return a None
    Some(get_book(&title))
}

// The get_match_data runs the regex and grabs the data from the captures.
fn get_match_data(
    title: &str,
    params: &str,
    search_type: SearchType,
    regex: &str,
) -> Option<BookParams> {
    // Build the regex matcher
    let matcher = Regex::new(regex).ok()?;

    // If there is a match (Some) then extract and set the data
    if let Some(captures) = matcher.captures(params) {
        return Some(BookParams {
            search_type,
            title: title.to_owned(),
            chapter: match_or_none(&captures, "chapter"),
            verse_start: match_or_none(&captures, "verse_start"),
            verse_end: match_or_none(&captures, "verse_end"),
        });
    }

    None
}

// The match_or_none function is a helper function to match a capture group
// then parse it into a u8. If the parse fails, it returns None.
fn match_or_none(captures: &Captures, name: &str) -> Option<u8> {
    captures.name(name)?.as_str().parse::<u8>().ok()
}

// Ex: Job
fn get_book(title: &str) -> BookParams {
    BookParams {
        search_type: SearchType::Book,
        title: title.to_owned(),
        chapter: None,
        verse_start: None,
        verse_end: None,
    }
}

// Ex: Job 1
fn get_chapter(title: &str, params: &str) -> Option<BookParams> {
    let re: &str = r"^\s*(?<chapter>\d{1,3}).*$";
    get_match_data(title, params, SearchType::Chapter, re)
}

// Ex: Job 1:2
fn get_verse(title: &str, params: &str) -> Option<BookParams> {
    let re: &str = r"^\s*(?<chapter>\d{1,3})\s*:\s*(?<verse_start>\d{1,3}).*$";
    get_match_data(title, params, SearchType::Verse, re)
}

// Ex: Job 1:2-3
fn get_verse_range(title: &str, params: &str) -> Option<BookParams> {
    let re: &str =
        r"^\s*(?<chapter>\d{1,3})\s*:\s*(?<verse_start>\d{1,3})\s*-\s*(?<verse_end>\d{1,3}).*$";
    get_match_data(title, params, SearchType::VerseRange, re)
}

pub fn get_sub_queries(query: &str) -> (Option<&str>, Vec<&str>) {
    let v: Vec<&str> = query.trim().split(',').map(|s| s.trim()).collect();

    let head = match v.first().copied() {
        Some("") => None,
        Some(s) => Some(s),
        None => None,
    };
    let tail = v[1..].to_vec();

    (head, tail)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_search_params_for_book_query() {
        assert_eq!(
            get_search_params(" 3 John ").unwrap(),
            BookParams {
                search_type: SearchType::Book,
                title: String::from("3 John"),
                chapter: None,
                verse_start: None,
                verse_end: None,
            }
        );
    }

    #[test]
    fn get_search_params_for_chapter_query() {
        assert_eq!(
            get_search_params(" 3 John 5").unwrap(),
            BookParams {
                search_type: SearchType::Chapter,
                title: String::from("3 John"),
                chapter: Some(5),
                verse_start: None,
                verse_end: None,
            }
        );
    }

    #[test]
    fn get_search_params_for_verse_query() {
        assert_eq!(
            get_search_params(" 3 John 125:221").unwrap(),
            BookParams {
                search_type: SearchType::Verse,
                title: String::from("3 John"),
                chapter: Some(125),
                verse_start: Some(221),
                verse_end: None,
            }
        );
    }

    #[test]
    fn get_search_params_for_verse_range_query() {
        assert_eq!(
            get_search_params(" 3 John 125:221-225").unwrap(),
            BookParams {
                search_type: SearchType::VerseRange,
                title: String::from("3 John"),
                chapter: Some(125),
                verse_start: Some(221),
                verse_end: Some(225),
            }
        );
    }

    #[test]
    fn get_search_params_returns_none_on_invalid_format() {
        assert_eq!(get_search_params(" 3 John *125-:225"), None);
    }

    #[test]
    fn get_sub_queries_from_input_returns_main_and_sub_queries() {
        assert_eq!(
            get_sub_queries(" John 1  ,  2,  3  "),
            (Some("John 1"), vec!["2", "3"])
        );
    }

    #[test]
    fn get_sub_queries_from_input_returns_some_and_empty_array_if_no_sub_queries() {
        assert_eq!(get_sub_queries("1 John"), (Some("1 John"), vec![]));
    }

    #[test]
    fn get_sub_queries_from_input_returns_none_and_empty_array_if_empty() {
        assert_eq!(get_sub_queries(""), (None, vec![]));
    }
}
