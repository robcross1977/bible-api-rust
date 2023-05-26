use std::collections::HashMap;

/// The get_chapter_count_by_book function takes a book name and returns the number of
/// chapters in that book in an Option. If the book is not found None is returned.
pub fn get_chapter_count_by_book(book: &str) -> Option<u8> {
    let chapter_counts: HashMap<&str, u8> = HashMap::from([
        ("1 Chronicles", 29),
        ("1 Corinthians", 16),
        ("1 John", 5),
        ("1 Kings", 22),
        ("1 Peter", 5),
        ("1 Samuel", 31),
        ("1 Thessalonians", 5),
        ("1 Timothy", 6),
        ("2 Chronicles", 36),
        ("2 Corinthians", 13),
        ("2 John", 1),
        ("2 Kings", 25),
        ("2 Peter", 3),
        ("2 Samuel", 24),
        ("2 Thessalonians", 3),
        ("2 Timothy", 4),
        ("3 John", 1),
        ("Acts", 28),
        ("Amos", 9),
        ("Colossians", 4),
        ("Daniel", 12),
        ("Deuteronomy", 34),
        ("Ecclesiastes", 12),
        ("Ephesians", 6),
        ("Esther", 10),
        ("Exodus", 40),
        ("Ezekiel", 48),
        ("Ezra", 10),
        ("Galatians", 6),
        ("Genesis", 50),
        ("Habakkuk", 3),
        ("Haggai", 2),
        ("Hebrews", 13),
        ("Hosea", 14),
        ("Isaiah", 66),
        ("James", 5),
        ("Jeremiah", 52),
        ("Job", 42),
        ("Joel", 3),
        ("John", 21),
        ("Jonah", 4),
        ("Joshua", 24),
        ("Jude", 1),
        ("Judges", 21),
        ("Lamentations", 5),
        ("Leviticus", 27),
        ("Luke", 24),
        ("Malachi", 4),
        ("Mark", 16),
        ("Matthew", 28),
        ("Micah", 7),
        ("Nahum", 3),
        ("Nehemiah", 13),
        ("Numbers", 36),
        ("Obadiah", 1),
        ("Philemon", 1),
        ("Philippians", 4),
        ("Proverbs", 31),
        ("Psalms", 150),
        ("Revelation", 22),
        ("Romans", 16),
        ("Ruth", 4),
        ("Song of Solomon", 8),
        ("Titus", 3),
        ("Zechariah", 14),
        ("Zephaniah", 3),
    ]);

    match chapter_counts.get(book) {
        Some(count) => Some(*count),
        None => None,
    }
}

/// The chapter_exists_in_book function takes a book name and a chapter number
/// and returns a bool indicating whether the chapter exists in the book.
pub fn chapter_exists_in_book(book: &str, chapter: u8) -> bool {
    let num_chapters = match get_chapter_count_by_book(book) {
        Some(num_chapters) => num_chapters,
        None => return false,
    };

    return chapter >= 1 && chapter <= num_chapters;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_chapter_count_by_book_returns_50_for_genesis() {
        assert_eq!(get_chapter_count_by_book("Genesis"), Some(50));
    }

    #[test]
    fn get_chapter_count_by_book_returns_none_for_invalid_book() {
        assert_eq!(get_chapter_count_by_book("Book of Robert"), None);
    }

    #[test]
    fn get_chapter_exists_in_book_returns_true_if_book_has_that_chapter() {
        assert_eq!(chapter_exists_in_book("Job", 3), true);
    }

    #[test]
    fn get_chapter_exists_in_book_returns_false_if_that_chapter_not_in_book() {
        assert_eq!(chapter_exists_in_book("Job", 100), false);
    }
}
