// Load the questions module (looking for ./day3/questions.rs)
mod questions;

// Use glob to expose / re-export the question functions (day3::q1 vs day3::questions::q1)
pub use questions::*;

