/*!
    This module "rolls the dice" on tables.
*/

pub trait Roll<T> {
    fn roll(&self) -> Option<&T>;
}
