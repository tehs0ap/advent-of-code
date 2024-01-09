// Load the questions module (looking for ./day2/questions.rs)
mod questions;

// Use glob to expose / re-export the question functions (day2::q1 vs day2::questions::q1)
pub use questions::*;

