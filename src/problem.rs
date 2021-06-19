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
mod problem_0005_longest_palindromic_substring;
mod problem_0006_zigzag_conversion;
mod problem_0007_reverse_integer;
mod problem_0009_palindrome_number;
mod problem_0011_container_with_most_water;
mod problem_0012_integer_to_roman;
mod problem_0015_3sum;
mod problem_0016_3sum_closest;
mod problem_0019_remove_nth_node_from_end_of_list;
mod problem_0022_generate_parentheses;
mod problem_0023_merge_k_sorted_lists;
