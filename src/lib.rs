pub fn brackets_are_balanced(string: &str) -> bool {
    const BRACKETS: &[char] = &['(', ')', '{', '}', '[', ']'];
    const MATCHED_BRACKETS: &[&[char]] = &[&['(', ')'], &['{', '}'], &['[', ']']];
    let mut brackets: Vec<_> = string.chars().filter(|cd| BRACKETS.contains(cd)).collect();
    let mut result = true;

    while brackets.len() != 0 {
        let brackets_left: Vec<_> = brackets
            .chunks(2)
            // .filter(|pair| !BRACKETS.chunks(2).collect::<Vec<_>>().contains(pair))
            .filter(|pair| !MATCHED_BRACKETS.contains(pair))
            .flatten()
            .cloned()
            .collect();
        // if brackets_left.len() != 2 && brackets_left == tmp_left {
        if brackets.len() == brackets_left.len() {
            result = false;
            break;
        } else {
            brackets = brackets_left;
        }
    }
    result
}



// (()())

// for pair in brackets_only.chunks(2) {
//     let pair: String = pair.iter().collect();
// }

// for _ in string.char() {

// }

// loop and remove from brackets_only any occurrences of any item in MATCH_BRACKETS
// if after look brackets_only still contains anything, return False, else True
// let mut is_lef = brackets_only.iter_mut();
// for bracket in MATCHED_BRACKETS.iter().cycle() {
//     brackets_only.
// }
// ((()))
