use serde::{Deserialize, Serialize};

use crate::{
    chapter::chapter_exists_in_book,
    params::{get_search_params, get_sub_queries, BookParams, SearchType},
    verse::{
        get_verse_count_by_book_and_chapter, get_verse_range_from_params, verse_exists_in_chapter,
    },
};

#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct BibleSearch {
    pub title: String,
    pub chapter: Vec<Chapter>,
}

#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct Chapter {
    pub chapter: u8,
    pub verse: Vec<u8>,
}

pub fn search(query: &str) -> Result<BibleSearch, String> {
    // Get the main query and the sub queries for the search
    let (main, _sub) = get_sub_queries(query);

    // Process the main query
    let search = match main {
        Some(main) => process_query(main),
        None => return Err(String::from("No Results Found")),
    };

    // Process the sub queries

    // Return the results
    search
}

fn process_query(query: &str) -> Result<BibleSearch, String> {
    // Get the typed search parameters for the query
    let book_search_params = get_search_params(query);

    // Turn the typed parameters into a BibleSearch using the handlers
    match book_search_params {
        Some(params) => match params.search_type {
            SearchType::Book => return book_to_bible_search(params),
            SearchType::Chapter => return chapter_to_bible_search(params),
            SearchType::Verse => return verse_to_bible_search(params),
            SearchType::VerseRange => return verse_range_to_bible_search(params),
        },
        None => return Err(String::from("No Matching Search Format Found")),
    }
}

fn book_to_bible_search(params: BookParams) -> Result<BibleSearch, String> {
    let updated_params = BookParams {
        search_type: SearchType::Chapter,
        title: params.title,
        chapter: Some(1),
        verse_start: None,
        verse_end: None,
    };

    return chapter_to_bible_search(updated_params);
}

fn chapter_to_bible_search(params: BookParams) -> Result<BibleSearch, String> {
    // Get the chapter start
    let chapter = match unwrap_chapter(&params.title, params.chapter) {
        Ok(value) => value,
        Err(_) => return revert_to_book_search(params.title),
    };

    // On a chapter search you just include ALL of the verses in the chapter.
    // This should never fail as it should have been checked during the params
    // processing, and the chapter and book are already validated here, so panic if it does.
    let verses_in_chapter = get_verse_count_by_book_and_chapter(&params.title, chapter).unwrap();

    // Build the BibleSearch
    Ok(BibleSearch {
        title: params.title,
        chapter: vec![Chapter {
            chapter: chapter,
            verse: (1..=verses_in_chapter).collect(),
        }],
    })
}

fn verse_to_bible_search(params: BookParams) -> Result<BibleSearch, String> {
    // Get the chapter start
    let chapter = match unwrap_chapter(&params.title, params.chapter) {
        Ok(value) => value,
        Err(_) => return revert_to_book_search(params.title),
    };

    // Get the verse start
    let verses_start = match unwrap_verse(&params.title, chapter, params.verse_start) {
        Ok(value) => value,
        Err(_) => return revert_to_chapter_search(params.title, chapter),
    };

    // Build the BibleSearch
    Ok(BibleSearch {
        title: params.title,
        chapter: vec![Chapter {
            chapter: chapter,
            verse: vec![verses_start],
        }],
    })
}

fn verse_range_to_bible_search(params: BookParams) -> Result<BibleSearch, String> {
    // Get the chapter start
    let chapter = match unwrap_chapter(&params.title, params.chapter) {
        Ok(value) => value,
        Err(_) => return revert_to_book_search(params.title),
    };

    // Get the verse start
    let verses_range =
        match unwrap_verse_range(&params.title, chapter, params.verse_start, params.verse_end) {
            Ok(value) => value,
            Err(_) => return revert_to_chapter_search(params.title, chapter),
        };

    // Build the BibleSearch
    Ok(BibleSearch {
        title: params.title,
        chapter: vec![Chapter {
            chapter: chapter,
            verse: verses_range,
        }],
    })
}

fn revert_to_book_search(title: String) -> Result<BibleSearch, String> {
    let updated_params = BookParams {
        search_type: SearchType::Book,
        title,
        chapter: None,
        verse_start: None,
        verse_end: None,
    };

    return book_to_bible_search(updated_params);
}

fn revert_to_chapter_search(title: String, chapter: u8) -> Result<BibleSearch, String> {
    let updated_params = BookParams {
        search_type: SearchType::Chapter,
        title,
        chapter: Some(chapter),
        verse_start: None,
        verse_end: None,
    };

    return chapter_to_bible_search(updated_params);
}

fn unwrap_chapter(book: &str, chapter: Option<u8>) -> Result<u8, String> {
    match chapter {
        Some(chapter_num) => {
            if chapter_exists_in_book(book, chapter_num) {
                Ok(chapter_num)
            } else {
                Err(String::from("Chapter does not exist in book"))
            }
        }

        None => return Err(String::from("No Chapter Start Found")),
    }
}

fn unwrap_verse(book: &str, chapter: u8, verse: Option<u8>) -> Result<u8, String> {
    match verse {
        Some(verse_num) => {
            if verse_exists_in_chapter(book, chapter, verse_num) {
                Ok(verse_num)
            } else {
                Err(String::from("Verse Does Not Exist In Book"))
            }
        }

        None => return Err(String::from("No Verse Start Found For Verse Search")),
    }
}

fn unwrap_verse_range(
    book: &str,
    chapter: u8,
    verse_start: Option<u8>,
    verse_end: Option<u8>,
) -> Result<Vec<u8>, String> {
    // The start should be checked before it gets here, so panic if it is a none
    let start = verse_start.unwrap();

    // This end should be checked before it gets here, so panic if it is a none
    let end = verse_end.unwrap();

    // Get the clamped range or return an error
    match get_verse_range_from_params(book, chapter, start..=end) {
        Some(range) => Ok(range),
        None => Err(String::from("Verse Range Does Not Exist In Book")),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn search_can_process_a_book_query() {
        let expected = BibleSearch {
            title: String::from("1 John"),
            chapter: vec![Chapter {
                chapter: 1,
                verse: vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10],
            }],
        };

        let result = search("1 John").unwrap();

        assert_eq!(result, expected);
    }

    #[test]
    fn search_can_process_a_chapter_query() {
        let expected = BibleSearch {
            title: String::from("1 John"),
            chapter: vec![Chapter {
                chapter: 1,
                verse: vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10],
            }],
        };

        let result = search("1 John 1").unwrap();
        assert_eq!(result, expected);
    }

    #[test]
    fn search_when_processing_a_failed_chapter_query_will_revert_to_book_query() {
        let expected = BibleSearch {
            title: String::from("1 John"),
            chapter: vec![Chapter {
                chapter: 1,
                verse: vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10],
            }],
        };

        let result = search("1 John").unwrap();
        assert_eq!(result, expected);
    }

    #[test]
    fn search_can_process_a_verse_query() {
        let expected = BibleSearch {
            title: String::from("1 John"),
            chapter: vec![Chapter {
                chapter: 2,
                verse: vec![3],
            }],
        };

        let result = search("1 John 2:3").unwrap();
        assert_eq!(result, expected);
    }

    #[test]
    fn search_when_processing_a_failed_verse_query_due_to_bad_chapter_will_revert_to_book_query() {
        let expected = BibleSearch {
            title: String::from("1 John"),
            chapter: vec![Chapter {
                chapter: 1,
                verse: vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10],
            }],
        };

        let result = search("1 John 223:3").unwrap();
        assert_eq!(result, expected);
    }

    #[test]
    fn search_when_processing_a_failed_verse_query_due_to_bad_verse_will_revert_to_chapter_query() {
        let expected = BibleSearch {
            title: String::from("1 John"),
            chapter: vec![Chapter {
                chapter: 4,
                verse: vec![
                    1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21,
                ],
            }],
        };

        let result = search("1 John 4:345").unwrap();
        assert_eq!(result, expected);
    }

    #[test]
    fn search_can_process_a_verse_range_query() {
        let expected = BibleSearch {
            title: String::from("1 John"),
            chapter: vec![Chapter {
                chapter: 2,
                verse: vec![3, 4, 5],
            }],
        };

        let result = search("1 John 2:3-5").unwrap();
        assert_eq!(result, expected);
    }

    #[test]
    fn search_when_processing_a_failed_verse_range_query_due_to_bad_chapter_will_revert_to_book_query(
    ) {
        let expected = BibleSearch {
            title: String::from("1 John"),
            chapter: vec![Chapter {
                chapter: 1,
                verse: vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10],
            }],
        };

        let result = search("1 John 223:3-4").unwrap();
        assert_eq!(result, expected);
    }

    #[test]
    fn search_when_processing_a_failed_verse_range_query_due_to_bad_verse_range_will_revert_to_chapter_query(
    ) {
        let expected = BibleSearch {
            title: String::from("1 John"),
            chapter: vec![Chapter {
                chapter: 4,
                verse: vec![
                    1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21,
                ],
            }],
        };

        let result = search("1 John 4:98-99").unwrap();
        assert_eq!(result, expected);
    }
}
