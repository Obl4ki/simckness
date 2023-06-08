use rand::{distributions::Standard, prelude::Distribution, Rng};

use crate::constants::{INFECTED_DAYS, MAX_AGE_ON_START, N_CELLS, RECOVERY_DAYS, SICK_DAYS};

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
            0 => HealthState::Healthy,
            1 => HealthState::Infected {
                days_until_sick: rng.gen_range(1..=INFECTED_DAYS),
            },
            2 => HealthState::Recovering {
                days_until_healthy: rng.gen_range(1..=RECOVERY_DAYS),
            },
            _ => HealthState::Sick {
                days_until_recovering: rng.gen_range(1..=SICK_DAYS),
            },
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
