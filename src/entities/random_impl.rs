use rand::{distributions::Standard, prelude::Distribution, seq::SliceRandom, Rng};

use crate::constants::{
    INFECTED_ON_START_PROB, MAX_AGE_ON_START, N_CELLS, RECOVERING_ON_START_PROB, SICK_ON_START_PROB,
};

use super::{
    age::Age, direction::Direction, health::HealthState, position::Position, speed::Speed,
};

impl Distribution<Direction> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Direction {
        let possible_directions = vec![
            (-1, -1),
            (-1, 0),
            (-1, 1),
            (0, -1),
            (0, 1),
            (1, -1),
            (1, 0),
            (1, 1),
        ];
        let (x, y) = possible_directions.choose(rng).unwrap().to_owned();

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
        let healthy_on_start_prob =
            1. - SICK_ON_START_PROB - INFECTED_ON_START_PROB - RECOVERING_ON_START_PROB;
        let random_num = rng.gen::<f64>();

        if random_num < healthy_on_start_prob {
            HealthState::new_healthy()
        } else if random_num < healthy_on_start_prob + SICK_ON_START_PROB {
            HealthState::new_sick()
        } else if random_num < healthy_on_start_prob + SICK_ON_START_PROB + INFECTED_ON_START_PROB {
            HealthState::new_infected()
        } else {
            HealthState::new_recovering()
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
