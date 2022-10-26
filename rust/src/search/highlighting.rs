// Type to mark characters as highlighted
#[derive(Debug)]
enum Fragment {
    Highlighted(char),
    Normal(char),
}

// This is the function doing the highlighting
// Takes search_query and mpn and returns vector of mpn characters with highlighting
// TODO: fix errors in this function
fn highlight_search_query_in_mpn(search_query: &str, mpn: &str) -> Vec<Fragment> {
    let mut string_fragments: Vec<Fragment> = Vec::new();
    let mut mpn_chars = mpn.chars();

    for query_char in search_query.chars() {
        while let Some(mpn_char) = mpn_chars.next() {
            if mpn_char == query_char {
                string_fragments.push(Fragment::Highlighted(mpn_char));
                break;
            } else {
                string_fragments.push(Fragment::Normal(mpn_char));
            }
        }
    }

    string_fragments
}
