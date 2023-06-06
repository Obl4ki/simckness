use anyhow::{Result, anyhow};
use parameters::{
    HIGH_IMMUNITY_BARRIER_INCLUSIVE, LOW_IMMUNITY_BARRIER_INCLUSIVE, MAX_AGE,
    MID_IMMUNITY_BARRIER_INCLUSIVE,
};
use random::rand_inclusive;

mod parameters;
mod random;

#[derive(Debug, PartialEq, Eq)]
struct Position {
    x: usize,
    y: usize,
}

#[derive(Debug, PartialEq, Eq)]
struct Direction(usize, usize);

#[derive(Debug, PartialEq, Eq)]
enum HealthState {
    Infected,
    Sick,
    Recovering,
    Healthy,
}

#[derive(Debug, PartialEq, Eq)]
struct Age(usize);

#[derive(Debug, PartialEq, Eq)]
enum Immunity {
    Low { level: usize },
    Normal { level: usize },
    High { level: usize },
}

impl TryFrom<Age> for Immunity {
    type Error = anyhow::Error;
    fn try_from(value: Age) -> Result<Self, Self::Error> {
        match value.0 {
            1..=3 => Ok(Immunity::Low {
                level: rand_inclusive(1, LOW_IMMUNITY_BARRIER_INCLUSIVE),
            }),
            4..=6 => Ok(Immunity::Normal {
                level: rand_inclusive(
                    LOW_IMMUNITY_BARRIER_INCLUSIVE + 1,
                    MID_IMMUNITY_BARRIER_INCLUSIVE,
                ),
            }),
            7..=10 => Ok(Immunity::High {
                level: rand_inclusive(
                    MID_IMMUNITY_BARRIER_INCLUSIVE + 1,
                    HIGH_IMMUNITY_BARRIER_INCLUSIVE,
                ),
            }),
            _ => Err(anyhow!("Panicked while getting immunity from age: {}", value.0)),
        }
    }
}

#[derive(Debug)]
struct Entity {
    position: Position,
    speed: usize,
    direction: Direction,
    health: HealthState,
    age: Age,
    immunity: Immunity,
}

impl Entity {
    pub fn new_random() -> Result<Self> {
        let position = rand::random();
        let age = Age(rand_inclusive(0, MAX_AGE));
        let immunity = Immunity::try_from(age)?;
        let health = rand::random();
        Ok(Self {
            position,
            speed: todo!(),
            direction: todo!(),
            health,
            age,
            immunity,
        })
    }
}

fn main() {}
