// input: "abcdefghijihgfedcba"
// output: true/false

// assume "ascii" only input
pub fn is_palindrome(phrase: &str) -> bool {
    let mut cleaned_phrase = Vec::<char>::with_capacity(phrase.len());
    for ch in phrase.chars() {
        if ch.is_ascii_alphabetic() == false {
            continue;
        };
        cleaned_phrase.push(ch);
    }
    let mut reverse_idx = cleaned_phrase.len() - 1;
    for idx in 0..cleaned_phrase.len() {
        if cleaned_phrase[idx] != cleaned_phrase[reverse_idx] {
            return false;
        }
        reverse_idx -= 1;
    }

    return true;
}
