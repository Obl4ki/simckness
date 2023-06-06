use crate::parameters::{HIGH_IMMUNITY_BARRIER_INCLUSIVE, MAX_IMMUNITY, N};
use crate::{parameters::MAX_AGE, Position};
use crate::{HealthState, Immunity};
use rand::{
    distributions::{Distribution, Standard},
    Rng,
};

pub fn rand_inclusive(min: usize, max: usize) -> usize {
    rand::thread_rng().gen_range(min..=max)
}

// impl Distribution<Immunity> for Standard {
//     fn sample<R: Rng + ?Sized>(&self, _rng: &mut R) -> Immunity {
//         let immunity_value = rand_inclusive(1, MAX_IMMUNITY);
//         match immunity_value {
//             1..=3 => Immunity::Low {
//                 level: immunity_value,
//             },
//             4..=6 => Immunity::Normal {
//                 level: immunity_value,
//             },
//             7..=10 => Immunity::High {
//                 level: immunity_value,
//             },
//             _ => {
//                 panic!(
//                     "Panicked while getting random immunity value: {} not in bounds of <1;{}>",
//                     immunity_value, HIGH_IMMUNITY_BARRIER_INCLUSIVE
//                 );
//             }
//         }
//     }
// }

impl Distribution<Position> for Standard {
    fn sample<R: Rng + ?Sized>(&self, _rng: &mut R) -> Position {
        let x = rand_inclusive(0, N);
        let y = rand_inclusive(0, N);

        Position { x, y }
    }
}

impl Distribution<HealthState> for Standard {
    fn sample<R: Rng + ?Sized>(&self, _rng: &mut R) -> HealthState {
        match rand_inclusive(0, 3) {
            0 => HealthState::Healthy,
            1 => HealthState::Infected,
            2 => HealthState::Recovering,
            _ => HealthState::Sick,
        }
    }
}
