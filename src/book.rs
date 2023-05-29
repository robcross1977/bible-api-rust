use regex::{Captures, Regex};
use std::collections::HashMap;

/// The ONES, TWOS, and THREES constants are used to build the regex pattern
/// to match the optional book number at the beginning of a bible search.
/// This number can have many forms, such as: 1st, i, one, 1, fst, first, etc.
const ONES: &str = r"(?i)one|fst|first|1(st)?|i\s+";
const TWOS: &str = r"(?i)two|sec(o(n(d)?)?)?|2(nd)?|ii\s+";
const THREES: &str = r"(?i)thr(e(e)?)?|thi(r(d)?)?|3(rd)?|iii\s+";

/// The BOOK_TEXT constant is used to build the regex pattern to match the
/// book title. The book title can be any non-digit character. This is
/// because the book title can be any number of words.
/// (e.g. 1 John, Song of Solomon)
const BOOK_TEXT: &str = r"(?i)(?<book_text>\D+)";

/// The get_title function takes a query passed in by a user and returns either
/// the proper name for the book as it exists in the DB, or None if the query
/// does not match a book.
pub fn get_title(query: &str) -> Option<String> {
    // Get the regex to match the book title
    let matcher = get_book_regex();

    // Get the captures from the regex
    let captures = matcher.captures(query)?;

    // Get the title from the captures
    let title = get_title_from_captures(captures)?;

    // Get the proper title using the search data provided
    let proper_title = get_proper_title(title.as_str());

    // Return the title
    proper_title
}

pub fn get_params(query: &str) -> Option<String> {
    // Get the regex to match the book title
    let matcher = get_book_regex();

    // Get the captures from the regex
    let captures = matcher.captures(query)?.get(0)?;

    // Strip the title from the query to get the remaining params
    let params = query.replace(captures.as_str(), "");

    if !params.is_empty() {
        Some(params)
    } else {
        None
    }
}

fn get_proper_title(title: &str) -> Option<String> {
    // The NON_NAME_CHARS matches any non-name characters at the end of the
    // title. This is used to remove any non-name characters from the title.
    const NON_NAME_CHARS: &str = r"[\d|:|-|_|\s]";

    // This is a map of regex to recognize the proper title of a book
    // and return it upon a match. The key is the proper title and the
    // value is the regex to match the title.
    let book_matcher = HashMap::from([
        (
            "1 Chronicles",
            format!(
                r"(?i)^({})\s*ch(r(o(n(i(c(l(e(s)?)?)?)?)?)?)?)?{}*$",
                ONES, NON_NAME_CHARS
            ),
        ),
        (
            "1 Corinthians",
            format!(
                r"(?i)^({})\s*co(r(i(n(t(h(i(a(n(s)?)?)?)?)?)?)?)?)?{}*$",
                ONES, NON_NAME_CHARS
            ),
        ),
        (
            "1 John",
            format!(r"(?i)^({})\s*j(o(h(n)?)?)?{}*$", ONES, NON_NAME_CHARS),
        ),
        (
            "1 Kings",
            format!(r"(?i)^({})\s*k(i(n(g(s)?)?)?)?{}*$", ONES, NON_NAME_CHARS),
        ),
        (
            "1 Peter",
            format!(r"(?i)^({})\s*p(e(t(e(r)?)?)?)?{}*$", ONES, NON_NAME_CHARS),
        ),
        (
            "1 Samuel",
            format!(r"(?ix)^({})\s*sam(u(e(l)?)?)?{}*$", ONES, NON_NAME_CHARS),
        ),
        (
            "1 Thessalonians",
            format!(
                r"(?i)^({})\s*th(e(s(s(a(l(o(n(i(a(n(s)?)?)?)?)?)?)?)?)?)?)?{}*$",
                ONES, NON_NAME_CHARS
            ),
        ),
        (
            "1 Timothy",
            format!(
                r"(?i)^({})\s*ti(m(o(t(h(y)?)?)?)?)?{}*$",
                ONES, NON_NAME_CHARS
            ),
        ),
        (
            "2 Chronicles",
            format!(
                r"(?i)^({})\s*ch(r(o(n(i(c(l(e(s)?)?)?)?)?)?)?)?{}*$",
                TWOS, NON_NAME_CHARS
            ),
        ),
        (
            "2 Corinthians",
            format!(
                r"(?i)^({})\s*co(r(i(n(t(h(i(a(n(s)?)?)?)?)?)?)?)?)?{}*$",
                TWOS, NON_NAME_CHARS
            ),
        ),
        (
            "2 John",
            format!(r"(?i)^({})\s*j(o(h(n)?)?)?{}*$", TWOS, NON_NAME_CHARS),
        ),
        (
            "2 Kings",
            format!(r"(?i)^({})\s*k(i(n(g(s)?)?)?)?{}*$", TWOS, NON_NAME_CHARS),
        ),
        (
            "2 Peter",
            format!(r"(?i)^({})\s*p(e(t(e(r)?)?)?)?{}*$", TWOS, NON_NAME_CHARS),
        ),
        (
            "2 Samuel",
            format!(
                r"(?i)^({})\s*s(a(m(u(e(l)?)?)?)?)?{}*$",
                TWOS, NON_NAME_CHARS
            ),
        ),
        (
            "2 Thessalonians",
            format!(
                r"(?i)^({})\s*th(e(s(s(a(l(o(n(i(a(n(s)?)?)?)?)?)?)?)?)?)?)?{}*$",
                TWOS, NON_NAME_CHARS
            ),
        ),
        (
            "2 Timothy",
            format!(
                r"(?i)^({})\s*ti(m(o(t(h(y)?)?)?)?)?{}*$",
                TWOS, NON_NAME_CHARS
            ),
        ),
        (
            "3 John",
            format!(r"(?i)^({})\s*j(o(h(n)?)?){}*$", THREES, NON_NAME_CHARS),
        ),
        ("Acts", format!("(?i)^ac(t(s)?)?{}*$", NON_NAME_CHARS)),
        ("Amos", format!("(?i)^am(o(s)?)?{}*$", NON_NAME_CHARS)),
        (
            "Colossians",
            format!(
                "(?i)^co(l(o(s(s(i(a(n(s)?)?)?)?)?)?)?)?{}*$",
                NON_NAME_CHARS
            ),
        ),
        (
            "Daniel",
            format!("(?i)^da(n(i(e(l)?)?)?)?{}*$", NON_NAME_CHARS),
        ),
        (
            "Deuteronomy",
            format!(
                "(?i)^d[e|u]([e|u](t(e(r(o(n(o(m(y)?)?)?)?)?)?)?)?)?{}*$",
                NON_NAME_CHARS
            ),
        ),
        (
            "Ecclesiastes",
            format!(
                "(?i)^ec(c(l(e(s(i(a(s(t(e(s)?)?)?)?)?)?)?)?)?)?{}*$",
                NON_NAME_CHARS
            ),
        ),
        (
            "Ephesians",
            format!("(?i)^ep(h(e(s(i(a(n(s)?)?)?)?)?)?)?{}*$", NON_NAME_CHARS),
        ),
        (
            "Esther",
            format!("(?i)^es(t(h(e(r)?)?)?)?{}*$", NON_NAME_CHARS),
        ),
        (
            "Exodus",
            format!("(?i)^ex(o(d(u(s)?)?)?)?{}*$", NON_NAME_CHARS),
        ),
        (
            "Ezekiel",
            format!("(?i)^eze(k(i(e(l)?)?)?)?{}*$", NON_NAME_CHARS),
        ),
        ("Ezra", format!("(?i)^ezr(a)?{}*$", NON_NAME_CHARS)),
        (
            "Galatians",
            format!("(?i)^ga(l(a(t(i(a(n(s)?)?)?)?)?)?)?{}*$", NON_NAME_CHARS),
        ),
        (
            "Genesis",
            format!("(?i)^ge(n(e(s(i(s)?)?)?)?)?{}*$", NON_NAME_CHARS),
        ),
        (
            "Habakkuk",
            format!("(?i)^hab(a(k(k(u(k)?)?)?)?)?{}*$", NON_NAME_CHARS),
        ),
        (
            "Haggai",
            format!("(?i)^hag(g(a(i)?)?)?{}*$", NON_NAME_CHARS),
        ),
        (
            "Hebrews",
            format!("(?i)^he(b(r(e(w(s)?)?)?)?)?{}*$", NON_NAME_CHARS),
        ),
        ("Hosea", format!("(?i)^ho(s(e(a)?)?)?{}*$", NON_NAME_CHARS)),
        (
            "Isaiah",
            format!("(?i)^is(a(i(a(h)?)?)?)?{}*$", NON_NAME_CHARS),
        ),
        ("James", format!("(?i)^ja(m(e(s)?)?)?{}*$", NON_NAME_CHARS)),
        (
            "Jeremiah",
            format!("(?i)^je(r(e(m(i(a(h)?)?)?)?)?)?{}*$", NON_NAME_CHARS),
        ),
        ("Job", format!("(?i)^job{}*$", NON_NAME_CHARS)),
        ("Joel", format!("(?i)^joe(l)?{}*$", NON_NAME_CHARS)),
        ("John", format!("(?i)^joh(n)?{}*$", NON_NAME_CHARS)),
        ("Jonah", format!("(?i)^jon(a(h)?)?{}*$", NON_NAME_CHARS)),
        (
            "Joshua",
            format!("(?i)^jos(h(u(a)?)?)?{}*$", NON_NAME_CHARS),
        ),
        ("Jude", format!("(?i)^jude{}*$", NON_NAME_CHARS)),
        ("Judges", format!("(?i)^judg(e(s)?)?{}*$", NON_NAME_CHARS)),
        (
            "Lamentations",
            format!(
                "(?i)^la(m(e(n(t(a(t(i(o(n(s)?)?)?)?)?)?)?)?)?)?{}*$",
                NON_NAME_CHARS
            ),
        ),
        (
            "Leviticus",
            format!("(?i)^le(v(i(t(i(c(u(s)?)?)?)?)?)?)?{}*$", NON_NAME_CHARS),
        ),
        ("Luke", format!("(?i)^lu(k(e)?)?{}*$", NON_NAME_CHARS)),
        (
            "Malachi",
            format!("(?i)^mal(a(c(h(i)?)?)?)?{}*$", NON_NAME_CHARS),
        ),
        ("Mark", format!("(?i)^mar(k)?{}*$", NON_NAME_CHARS)),
        (
            "Matthew",
            format!("(?i)^mat(t(h(e(w)?)?)?)?{}*$", NON_NAME_CHARS),
        ),
        ("Micah", format!("(?i)^mi(c(a(h)?)?)?{}*$", NON_NAME_CHARS)),
        ("Nahum", format!("(?i)^na(h(u(m)?)?)?{}*$", NON_NAME_CHARS)),
        (
            "Nehemiah",
            format!("(?i)^ne(h(e(m(i(a(h)?)?)?)?)?)?{}*$", NON_NAME_CHARS),
        ),
        (
            "Numbers",
            format!("(?i)^nu(m(b(e(r(s)?)?)?)?)?{}*$", NON_NAME_CHARS),
        ),
        (
            "Obadiah",
            format!("(?i)^o(b(a(d(i(a(h)?)?)?)?)?)?{}*$", NON_NAME_CHARS),
        ),
        (
            "Philemon",
            format!("(?i)^phile(m(o(n)?)?)?{}*$", NON_NAME_CHARS),
        ),
        (
            "Philippians",
            format!("(?i)^phili(p(p(i(a(n(s)?)?)?)?)?)?{}*$", NON_NAME_CHARS),
        ),
        (
            "Proverbs",
            format!("(?i)^pr(o(v(e(r(b(s)?)?)?)?)?)?{}*$", NON_NAME_CHARS),
        ),
        (
            "Psalms",
            format!("(?i)^ps(a(l(m(s)?)?)?)?{}*$", NON_NAME_CHARS),
        ),
        (
            "Revelation",
            format!(
                "(?i)^re(v(e(l(a(t(i(o(n)?)?)?)?)?)?)?)?{}*$",
                NON_NAME_CHARS
            ),
        ),
        (
            "Romans",
            format!("(?i)^ro(m(a(n(s)?)?)?)?{}*$", NON_NAME_CHARS),
        ),
        ("Ruth", format!("(?i)^ru(t(h)?)?{}*$", NON_NAME_CHARS)),
        (
            "Song of Solomon",
            format!(
                r"(?i)^s(o(n(g\s*(o(f\s*(s(o(l(o(m(o(n)?)?)?)?)?)?)?)?)?)?)?)?{}*$",
                NON_NAME_CHARS
            ),
        ),
        ("Titus", format!("(?i)^ti(t(u(s)?)?)?{}*$", NON_NAME_CHARS)),
        (
            "Zechariah",
            format!("(?i)^zec(h(a(r(i(a(h)?)?)?)?)?)?{}*$", NON_NAME_CHARS),
        ),
        (
            "Zephaniah",
            format!("(?i)^zep(h(a(n(i(a(h)?)?)?)?)?)?{}*$", NON_NAME_CHARS),
        ),
    ]);

    // Iterate over the book_matcher and return the proper title if a match is found
    for (key, value) in book_matcher.into_iter() {
        if Regex::new(value.as_str()).unwrap().is_match(title) {
            return Some(key.to_owned());
        }
    }

    // Return None if no match is found
    None
}

/// The get_regex function exists to make the regex pattern more readable.
/// If we end up trying to add to or take away from the pattern it is much
/// easier to digest chunked up into pieces. The regex pattern is built
/// from the constants defined above.
fn get_book_regex() -> regex::Regex {
    // Combine the book number constants into a single string
    // that looks for all patterns that match the book number.
    let book_num = format!(r"(?<book_num>{}|{}|{})", ONES, TWOS, THREES);

    // Combine the book number string with the book text string
    // Note the book number is marked as optional, and any number
    // of spaces is allowed between the number and the string
    let book_title = format!(r"\s*{}?\s*{}\s*", book_num, BOOK_TEXT);

    // Create the regex matcher string and retun
    Regex::new(&book_title).unwrap()
}

fn get_title_from_captures(captures: Captures) -> Option<String> {
    // The book_num is optional, so we need to check if it exists
    // and if not we want to return an empty string. Note, if
    // a book_number greater than 3 is present it will panic.
    let book_num = match captures.name("book_num") {
        Some(data) => get_book_num_string(data.as_str()),
        None => "",
    };

    // Get the book_text from the captures
    let book_text = captures.name("book_text")?.as_str();

    // Format the book_num and book_text into a single string and return
    format_title(book_num, book_text)
}

fn get_book_num_string(book_num: &str) -> &str {
    // If the book_num matches any of the regex patterns return the
    // corresponding book number string. If no match is found panic.
    if regex::Regex::new(THREES).unwrap().is_match(book_num) {
        "3 "
    } else if regex::Regex::new(TWOS).unwrap().is_match(book_num) {
        "2 "
    } else if regex::Regex::new(ONES).unwrap().is_match(book_num) {
        "1 "
    } else {
        panic!("Invalid book number: {}", book_num);
    }
}

fn format_title(book_num: &str, book_text: &str) -> Option<String> {
    let trimmed = book_text.trim();

    if trimmed.is_empty() {
        None
    } else {
        Some(format!("{}{}", book_num, trimmed))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::prelude::*;
    use std::collections::HashMap;

    // This function will generate a list of test cases for the get_title function to
    // test a regex for a specific book. Each book has a title and a minimum number
    // of charcters it can be recognized by. This function will grab the smallest
    // varation up to the complete title and return a list of test cases.
    fn get_book_title_variations(book_title: &str, minimum_length: usize) -> Vec<String> {
        let mut variants: Vec<String> = Vec::new();
        let splits = book_title.split_at(minimum_length);

        for (i, _) in splits.1.chars().enumerate() {
            let sub_splits = splits.1.split_at(i);
            variants.push(format!("{}{}", splits.0, sub_splits.0).to_owned());
        }

        variants.push(book_title.to_owned());

        variants
    }

    // This will add the book numbers with various spaces to the book title to
    // test out the regex
    fn add_numbered_variants(book_title: &str, variants: Vec<&str>) -> Vec<String> {
        let no_spaces: Vec<String> = variants
            .iter()
            .map(|s| format!("{}{}", s, book_title))
            .collect();
        let center_spaces: Vec<String> = variants
            .iter()
            .map(|s| format!("{}  {}", s, book_title))
            .collect();
        let leading_spaces: Vec<String> = variants
            .iter()
            .map(|s| format!("  {}  {}", s, book_title))
            .collect();
        let trailing_spaces: Vec<String> = variants
            .iter()
            .map(|s| format!("{}  {}  ", s, book_title))
            .collect();
        let all_spaces: Vec<String> = variants
            .iter()
            .map(|s| format!("  {}  {}  ", s, book_title))
            .collect();

        // return all the variants
        [
            no_spaces,
            center_spaces,
            leading_spaces,
            trailing_spaces,
            all_spaces,
        ]
        .concat()
    }

    // Our test cases are all lowercase, but the function under test
    // should be able to handle any case. To test this we will randomly
    // capitalize the input string and check the result.
    fn randomly_capitalize(input: &str) -> String {
        let mut rng = rand::thread_rng();
        let mut output = String::new();

        for c in input.chars() {
            if rng.gen::<bool>() {
                output.push(c.to_uppercase().next().unwrap());
            } else {
                output.push(c);
            }
        }

        output
    }

    // This function will loop through the test queries, randomize the case,
    // call the function under test, and check the result.
    fn run_and_check_result(test_queries: Vec<String>, expected: &str) {
        test_queries.iter().for_each(|test| {
            // randomize the case of the input
            let random_case_input = randomly_capitalize(test);

            // call the function under test
            let result = get_title(random_case_input.as_str()).unwrap();

            // check the result
            println!("{} -> {}", random_case_input, result);
            assert_eq!(result, expected);
        });
    }

    fn run_book_test(
        book_title: &str,
        min_title_chars: usize,
        num_variations: Vec<&str>,
        expected: &str,
    ) {
        let book_title_variations = get_book_title_variations(book_title, min_title_chars);
        let book_variations: Vec<String> = book_title_variations
            .iter()
            .flat_map(|s| {
                let result = add_numbered_variants(s, num_variations.clone());
                let random_case = result
                    .iter()
                    .map(|s| randomly_capitalize(s))
                    .collect::<Vec<String>>();

                random_case
            })
            .collect();
        run_and_check_result(book_variations, expected);
    }

    #[test]
    fn get_title_gets_proper_title_for_one_chronicles() {
        run_book_test(
            "chronicles",
            2,
            vec!["one", "fst", "first", "1", "1st", "i "],
            "1 Chronicles",
        );
    }

    #[test]
    fn get_title_gets_proper_title_for_one_corinthians() {
        run_book_test(
            "corinthians",
            2,
            vec!["one", "fst", "first", "1", "1st", "i "],
            "1 Corinthians",
        );
    }

    #[test]
    fn get_title_gets_proper_title_for_one_john() {
        run_book_test(
            "john",
            2,
            vec!["one", "fst", "first", "1", "1st", "i "],
            "1 John",
        );
    }

    #[test]
    fn get_title_gets_proper_title_for_one_kings() {
        run_book_test(
            "kings",
            1,
            vec!["one", "fst", "first", "1", "1st", "i "],
            "1 Kings",
        );
    }

    #[test]
    fn get_title_gets_proper_title_for_one_peter() {
        run_book_test(
            "peter",
            1,
            vec!["one", "fst", "first", "1", "1st", "i "],
            "1 Peter",
        );
    }

    #[test]
    fn get_title_gets_proper_title_for_one_samuel() {
        run_book_test(
            "samuel",
            3,
            vec!["one", "first", "1", "1st", "i "],
            "1 Samuel",
        );
    }

    #[test]
    fn get_title_gets_proper_title_for_one_thessalonians() {
        run_book_test(
            "thessalonians",
            2,
            vec!["one", "fst", "first", "1", "1st", "i "],
            "1 Thessalonians",
        );
    }

    #[test]
    fn get_title_gets_proper_title_for_one_timothy() {
        run_book_test(
            "timothy",
            2,
            vec!["one", "fst", "first", "1", "1st", "i "],
            "1 Timothy",
        );
    }

    #[test]
    fn get_title_gets_proper_title_for_two_chronicles() {
        run_book_test(
            "chronicles",
            2,
            vec!["two", "sec", "seco", "secon", "second", "2", "2nd", "ii "],
            "2 Chronicles",
        );
    }

    #[test]
    fn get_title_gets_proper_title_for_two_corinthians() {
        run_book_test(
            "corinthians",
            2,
            vec!["two", "sec", "seco", "secon", "second", "2", "2nd", "ii "],
            "2 Corinthians",
        );
    }

    #[test]
    fn get_title_gets_proper_title_for_two_john() {
        run_book_test(
            "john",
            1,
            vec!["two", "sec", "seco", "secon", "second", "2", "2nd", "ii "],
            "2 John",
        );
    }

    #[test]
    fn get_title_gets_proper_title_for_two_kings() {
        run_book_test(
            "kings",
            1,
            vec!["two", "sec", "seco", "secon", "second", "2", "2nd", "ii "],
            "2 Kings",
        );
    }

    #[test]
    fn get_title_gets_proper_title_for_two_peter() {
        run_book_test(
            "peter",
            1,
            vec!["two", "sec", "seco", "secon", "second", "2", "2nd", "ii "],
            "2 Peter",
        );
    }

    #[test]
    fn get_title_gets_proper_title_for_two_samuel() {
        run_book_test(
            "samuel",
            1,
            vec!["two", "sec", "seco", "secon", "second", "2", "2nd", "ii "],
            "2 Samuel",
        );
    }

    #[test]
    fn get_title_gets_proper_title_for_two_thessalonians() {
        run_book_test(
            "thessalonians",
            2,
            vec!["two", "sec", "seco", "secon", "second", "2", "2nd", "ii "],
            "2 Thessalonians",
        );
    }

    #[test]
    fn get_title_gets_proper_title_for_two_timothy() {
        run_book_test(
            "timothy",
            2,
            vec!["two", "sec", "seco", "secon", "second", "2", "2nd", "ii "],
            "2 Timothy",
        );
    }

    #[test]
    fn get_title_gets_proper_title_for_three_john() {
        run_book_test(
            "john",
            2,
            vec![
                "thr", "thre", "three", "thi", "thir", "third", "3", "3rd", "iii ",
            ],
            "3 John",
        );
    }

    #[test]
    fn get_title_gets_proper_title_for_acts() {
        run_book_test("acts", 2, vec![""], "Acts");
    }

    #[test]
    fn get_title_gets_proper_title_for_amos() {
        run_book_test("amos", 2, vec![""], "Amos");
    }

    #[test]
    fn get_title_gets_proper_title_for_colossians() {
        run_book_test("colossians", 2, vec![""], "Colossians");
    }

    #[test]
    fn get_title_gets_proper_title_for_daniel() {
        run_book_test("daniel", 2, vec![""], "Daniel");
    }

    #[test]
    fn get_title_gets_proper_title_for_deuteronomy() {
        run_book_test("deuteronomy", 2, vec![""], "Deuteronomy");
    }

    #[test]
    fn get_title_gets_proper_title_for_ecclesiastes() {
        run_book_test("ecclesiastes", 2, vec![""], "Ecclesiastes");
    }

    #[test]
    fn get_title_gets_proper_title_for_ephesians() {
        run_book_test("ephesians", 2, vec![""], "Ephesians");
    }

    #[test]
    fn get_title_gets_proper_title_for_esther() {
        run_book_test("esther", 2, vec![""], "Esther");
    }

    #[test]
    fn get_title_gets_proper_title_for_exodus() {
        run_book_test("exodus", 2, vec![""], "Exodus");
    }

    #[test]
    fn get_title_gets_proper_title_for_ezekiel() {
        run_book_test("ezekiel", 3, vec![""], "Ezekiel");
    }

    #[test]
    fn get_title_gets_proper_title_for_ezra() {
        run_book_test("ezra", 3, vec![""], "Ezra");
    }

    #[test]
    fn get_title_gets_proper_title_for_galatians() {
        run_book_test("galatians", 2, vec![""], "Galatians");
    }

    #[test]
    fn get_title_gets_proper_title_for_genesis() {
        run_book_test("genesis", 2, vec![""], "Genesis");
    }

    #[test]
    fn get_title_gets_proper_title_for_habakkuk() {
        run_book_test("habakkuk", 3, vec![""], "Habakkuk");
    }

    #[test]
    fn get_title_gets_proper_title_for_haggai() {
        run_book_test("haggai", 3, vec![""], "Haggai");
    }

    #[test]
    fn get_title_gets_proper_title_for_hebrews() {
        run_book_test("hebrews", 2, vec![""], "Hebrews");
    }

    #[test]
    fn get_title_gets_proper_title_for_hosea() {
        run_book_test("hosea", 2, vec![""], "Hosea");
    }

    #[test]
    fn get_title_gets_proper_title_for_isaiah() {
        run_book_test("isaiah", 2, vec![""], "Isaiah");
    }

    #[test]
    fn get_title_gets_proper_title_for_james() {
        run_book_test("james", 2, vec![""], "James");
    }

    #[test]
    fn get_title_gets_proper_title_for_jeremiah() {
        run_book_test("jeremiah", 2, vec![""], "Jeremiah");
    }

    #[test]
    fn get_title_gets_proper_title_for_job() {
        run_book_test("job", 3, vec![""], "Job");
    }

    #[test]
    fn get_title_gets_proper_title_for_joel() {
        run_book_test("joel", 3, vec![""], "Joel");
    }

    #[test]
    fn get_title_gets_proper_title_for_john() {
        run_book_test("john", 3, vec![""], "John");
    }

    #[test]
    fn get_title_gets_proper_title_for_jonah() {
        run_book_test("jonah", 3, vec![""], "Jonah");
    }

    #[test]
    fn get_title_gets_proper_title_for_joshua() {
        run_book_test("joshua", 3, vec![""], "Joshua");
    }

    #[test]
    fn get_title_gets_proper_title_for_jude() {
        run_book_test("jude", 4, vec![""], "Jude");
    }

    #[test]
    fn get_title_gets_proper_title_for_judges() {
        run_book_test("judges", 4, vec![""], "Judges");
    }

    #[test]
    fn get_title_gets_proper_title_for_lamentations() {
        run_book_test("lamentations", 2, vec![""], "Lamentations");
    }

    #[test]
    fn get_title_gets_proper_title_for_leviticus() {
        run_book_test("leviticus", 2, vec![""], "Leviticus");
    }

    #[test]
    fn get_title_gets_proper_title_for_luke() {
        run_book_test("luke", 2, vec![""], "Luke");
    }

    #[test]
    fn get_title_gets_proper_title_for_malachi() {
        run_book_test("malachi", 3, vec![""], "Malachi");
    }

    #[test]
    fn get_title_gets_proper_title_for_mark() {
        run_book_test("mark", 3, vec![""], "Mark");
    }

    #[test]
    fn get_title_gets_proper_title_for_matthew() {
        run_book_test("matthew", 3, vec![""], "Matthew");
    }

    #[test]
    fn get_title_gets_proper_title_for_micah() {
        run_book_test("micah", 3, vec![""], "Micah");
    }

    #[test]
    fn get_title_gets_proper_title_for_nahum() {
        run_book_test("nahum", 2, vec![""], "Nahum");
    }

    #[test]
    fn get_title_gets_proper_title_for_nehemiah() {
        run_book_test("nehemia", 2, vec![""], "Nehemiah");
    }

    #[test]
    fn get_title_gets_proper_title_for_numbers() {
        run_book_test("numbers", 2, vec![""], "Numbers");
    }

    #[test]
    fn get_title_gets_proper_title_for_obadiah() {
        run_book_test("obadiah", 2, vec![""], "Obadiah");
    }

    #[test]
    fn get_title_gets_proper_title_for_philemon() {
        run_book_test("philemon", 5, vec![""], "Philemon");
    }

    #[test]
    fn get_title_gets_proper_title_for_philippians() {
        run_book_test("philippians", 5, vec![""], "Philippians");
    }

    #[test]
    fn get_title_gets_proper_title_for_proverbs() {
        run_book_test("proverbs", 2, vec![""], "Proverbs");
    }

    #[test]
    fn get_title_gets_proper_title_for_psalms() {
        run_book_test("psalms", 2, vec![""], "Psalms");
    }

    #[test]
    fn get_title_gets_proper_title_for_revelation() {
        run_book_test("revelation", 2, vec![""], "Revelation");
    }

    #[test]
    fn get_title_gets_proper_title_for_romans() {
        run_book_test("romans", 2, vec![""], "Romans");
    }

    #[test]
    fn get_title_gets_proper_title_for_ruth() {
        run_book_test("ruth", 2, vec![""], "Ruth");
    }

    #[test]
    fn get_title_gets_proper_title_for_song_of_solomon() {
        run_book_test("song of solomon", 1, vec![""], "Song of Solomon");
    }

    #[test]
    fn get_title_gets_proper_title_for_titus() {
        run_book_test("titus", 2, vec![""], "Titus");
    }

    #[test]
    fn get_title_gets_proper_title_for_zechariah() {
        run_book_test("zechariah", 3, vec![""], "Zechariah");
    }

    #[test]
    fn get_title_gets_proper_title_for_zephaniah() {
        run_book_test("zephaniah", 3, vec![""], "Zephaniah");
    }

    #[test]
    fn get_params_strips_off_everything_after_book_title() {
        let tests = HashMap::from([
            ("John  ", ""),
            ("Job 1", "1"),
            ("  Psalms    1:  2", "1:  2"),
            ("1 Song of Solomon 2 : 3 - 5 : 6", "2 : 3 - 5 : 6"),
        ]);

        for (key, value) in tests.into_iter() {
            let result = get_params(key).unwrap_or(String::from(""));
            assert_eq!(result, value);
        }
    }
}
