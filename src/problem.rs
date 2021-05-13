#![allow(dead_code)]

#[derive(Default, Debug, serde::Serialize)]
pub struct Problem {
    pub id: String,
    pub title: String,
    pub content: String,
    pub code: String,
    pub link: String,
}

// problems
mod problem_0001_two_sum;
mod problem_0002_add_two_numbers;
mod problem_0003_longest_substring_without_repeating_characters;
mod problem_0004_median_of_two_sorted_arrays;
