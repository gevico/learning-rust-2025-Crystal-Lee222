// iterators2.rs
//
// In this exercise, you'll learn some of the unique advantages that iterators
// can offer. Follow the steps to complete the exercise.
//
// Execute `rustlings hint iterators2` or use the `hint` watch subcommand for a
// hint.

// iterators2.rs

// Step 1.
pub fn capitalize_first(input: &str) -> String {
    let mut c = input.chars();
    match c.next() {
        None => String::new(),
        Some(first) => first.to_uppercase().collect::<String>() + c.as_str(),
    }
}

// Step 2.
pub fn capitalize_words_vector(words: &[&str]) -> Vec<String> {
    words
        .iter()
        .map(|&word| capitalize_first(word))  // ✅ 解引用
        .collect()
}

// Step 3.
pub fn capitalize_words_string(words: &[&str]) -> String {
    words
        .iter()
        .map(|&word| capitalize_first(word))  // ✅ 解引用
        .collect::<Vec<String>>()
        .join("")
}