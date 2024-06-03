/*!
    This module "rolls the dice" on tables.
*/
use rand::seq::SliceRandom;

pub fn roll(table: &Vec<String>) -> Option<&String> {
    table.choose(&mut rand::thread_rng())
}
