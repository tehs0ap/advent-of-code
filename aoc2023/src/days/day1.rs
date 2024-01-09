// Load the questions module (looking for ./day1/questions.rs)
mod questions;

// Use glob to expose / re-export the question functions (day1::q1 vs day1::questions::q1)
pub use questions::*;

