use std::ops::Div;

use crate::roll::Roll;
use anyhow::Context;
use rand::seq::SliceRandom;

pub struct Table<T>(pub Vec<T>);

impl<T> Roll<T> for Table<T> {
    fn roll(&self) -> Option<&T> {
        self.0.choose(&mut rand::thread_rng())
    }
}

pub struct WeightedTable<T>(Vec<(T, u8)>);

impl<T> Roll<T> for WeightedTable<T> {
    fn roll(&self) -> Option<&T> {
        self.0
            .choose_weighted(&mut rand::thread_rng(), |item| item.1)
            .ok()
            .and_then(|e| Some(&e.0))
    }
}

impl TryFrom<Vec<String>> for WeightedTable<String> {
    type Error = anyhow::Error;

    fn try_from(value: Vec<String>) -> Result<Self, Self::Error> {
        if !value.iter().any(|e| e.contains("::")) {
            return Err(anyhow::anyhow!("Oops!"));
        }

        let mut tuples = value
            .iter()
            .map(|e| {
                let split = e.split("::").collect::<Vec<&str>>();
                let first = split.first().with_context(|| "Barf")?;
                let odds = match split.len() {
                    1 => 0u8,
                    _ => split
                        .last()
                        .and_then(|v| v.parse::<u8>().ok())
                        .unwrap_or_else(|| 0),
                };

                Ok((first.to_string(), odds))
            })
            .collect::<Result<Vec<(String, u8)>, Self::Error>>()?;

        let dist: usize = tuples
            .iter()
            .fold(0, |acc, e| acc + e.1 as usize)
            .div(tuples.len());

        for tuple in &mut tuples {
            if tuple.1 == 0 {
                tuple.1 = dist as u8;
            }
        }
        Ok(Self(tuples))
    }
}
