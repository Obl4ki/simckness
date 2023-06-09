use rand::{distributions::Standard, prelude::Distribution, Rng};

use crate::constants::{MAX_AGE_ON_START, N_CELLS};

use super::{
    age::Age, direction::Direction, health::HealthState, position::Position, speed::Speed,
};

impl Distribution<Direction> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Direction {
        let x = match rng.gen_range(0..=2) {
            0 => -1,
            1 => 0,
            _ => 1,
        };

        let y = match rng.gen_range(0..=2) {
            0 => -1,
            1 => 0,
            _ => 1,
        };

        Direction { x, y }
    }
}

impl Distribution<Position> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Position {
        let x = rng.gen_range(0..N_CELLS);
        let y = rng.gen_range(0..N_CELLS);

        Position { x, y }
    }
}

impl Distribution<HealthState> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> HealthState {
        match rng.gen_range(0..=3) {
            0 => HealthState::new_healthy(),
            1 => HealthState::new_infected(),
            2 => HealthState::new_recovering(),
            _ => HealthState::new_sick(),
        }
    }
}

impl Distribution<Age> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Age {
        Age(rng.gen_range(0..=MAX_AGE_ON_START))
    }
}

impl Distribution<Speed> for Standard {
    fn sample<R: rand::Rng + ?Sized>(&self, rng: &mut R) -> Speed {
        Speed(rng.gen_range(1..=3))
    }
}
