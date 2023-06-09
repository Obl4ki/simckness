use crate::constants::{BIRTH_ON_CONTACT_PROB, MAX_AGE, MAX_CHILDREN_PER_BIRTH, N_CELLS};

use super::age::Age;
use super::direction::Direction;
use super::health::HealthState;
use super::immunity::{Immunity, ImmunityRange};
use super::position::Position;
use super::speed::Speed;
use anyhow::Result;
use rand::distributions::Bernoulli;
use rand::prelude::Distribution;
use rand::thread_rng;

#[derive(Debug, Clone)]
pub struct Entity {
    pub position: Position,
    pub speed: Speed,
    pub direction: Direction,
    pub health: HealthState,
    pub age: Age,
    pub immunity: Immunity,
    pub current_children: Vec<Entity>,
}

impl Entity {
    pub fn new_random() -> Result<Self> {
        let position = rand::random();
        let age = rand::random();
        let immunity = Self::get_immunity_for_age(age).random_in_range();
        let health = rand::random();
        let speed = rand::random();
        let direction = rand::random();
        Ok(Self {
            position,
            speed,
            direction,
            health,
            age,
            immunity,
            current_children: vec![],
        })
    }

    pub fn make_move(&mut self) {
        for _ in 0..self.speed.0 {
            self.revert_direction_if_on_border();
            let new_x = (self.position.x as i32 + self.direction.x) as usize;
            let new_y = (self.position.y as i32 + self.direction.y) as usize;

            self.position = Position { x: new_x, y: new_y };
        }

        if self.position.x > N_CELLS {
            self.position.x = N_CELLS - 1;
        }
        if self.position.y > N_CELLS {
            self.position.y = N_CELLS - 1;
        }
    }

    pub fn check_contact(&mut self, other: &Entity) {
        if usize::max(
            self.position.x.abs_diff(other.position.x),
            self.position.y.abs_diff(other.position.y),
        ) <= 2
        {
            self.on_contact(other)
        }
    }

    fn on_contact(&mut self, other: &Entity) {
        self.direction.x *= -1;
        self.direction.y *= -1;

        if Bernoulli::new(BIRTH_ON_CONTACT_PROB)
            .unwrap()
            .sample(&mut thread_rng())
        {
            self.give_birth_if_suitable(other);
        }

        self.health = match (self.health, other.health) {
            (HealthState::Healthy, HealthState::Infected { days_until_sick: _ }) => {
                if self.immunity.is_low() {
                    HealthState::new_sick()
                } else {
                    self.health
                }
            }
            (
                HealthState::Healthy,
                HealthState::Sick {
                    days_until_recovering: _,
                },
            ) => {
                if self.immunity.is_low() || self.immunity.is_medium() {
                    HealthState::new_infected()
                } else {
                    self.immunity.0 -= 3.0;
                    self.health
                }
            }
            (
                HealthState::Recovering {
                    days_until_healthy: _,
                },
                HealthState::Healthy,
            ) => {
                self.immunity.0 += 1.;
                self.health
            }
            (HealthState::Healthy, HealthState::Healthy) => {
                self.immunity = Immunity(self.get_maximum_immunity_for_age());
                self.health
            }
            (
                HealthState::Sick {
                    days_until_recovering: _,
                },
                HealthState::Infected { days_until_sick: _ },
            ) => HealthState::new_sick(),
            (
                HealthState::Infected { days_until_sick: _ },
                HealthState::Sick {
                    days_until_recovering: _,
                },
            ) => {
                if self.immunity.is_low() || self.immunity.is_medium() {
                    HealthState::new_sick()
                } else {
                    self.health
                }
            }
            (
                HealthState::Recovering {
                    days_until_healthy: _,
                },
                HealthState::Sick {
                    days_until_recovering: _,
                },
            ) => {
                if self.immunity.is_low() || self.immunity.is_medium() {
                    HealthState::new_infected()
                } else {
                    self.health
                }
            }
            (
                HealthState::Sick {
                    days_until_recovering: _,
                },
                HealthState::Sick {
                    days_until_recovering: _,
                },
            ) => {
                self.immunity = Immunity(self.get_minimum_immunity_for_age());
                HealthState::new_sick()
            }

            (
                HealthState::Infected { days_until_sick: _ },
                HealthState::Infected { days_until_sick: _ },
            ) => {
                self.immunity.0 -= 1.;
                self.health
            }

            (
                HealthState::Recovering {
                    days_until_healthy: _,
                },
                HealthState::Infected { days_until_sick: _ },
            ) => {
                self.immunity.0 -= 1.;
                self.health
            }

            (HealthState::Infected { days_until_sick: _ }, HealthState::Healthy)
            | (
                HealthState::Sick {
                    days_until_recovering: _,
                },
                HealthState::Healthy,
            )
            | (
                HealthState::Healthy,
                HealthState::Recovering {
                    days_until_healthy: _,
                },
            )
            | (
                HealthState::Sick {
                    days_until_recovering: _,
                },
                HealthState::Recovering {
                    days_until_healthy: _,
                },
            )
            | (
                HealthState::Infected { days_until_sick: _ },
                HealthState::Recovering {
                    days_until_healthy: _,
                },
            )
            | (
                HealthState::Recovering {
                    days_until_healthy: _,
                },
                HealthState::Recovering {
                    days_until_healthy: _,
                },
            ) => self.health,
        };
    }

    fn revert_direction_if_on_border(&mut self) {
        if self.position.x == 0 {
            self.direction.x = 1;
        }
        if self.position.x == N_CELLS {
            self.direction.x = 1;
        }

        if self.position.y == 0 {
            self.direction.y = 1;
        }
        if self.position.y == N_CELLS {
            self.direction.y = -1;
        }
    }

    pub fn get_immunity_for_age(age: Age) -> ImmunityRange {
        match age.0 {
            15..=39 => ImmunityRange::Normal,
            40..=69 => ImmunityRange::High,
            _ => ImmunityRange::Low,
        }
    }

    pub fn should_die(&self) -> bool {
        self.age.0 >= MAX_AGE || self.immunity.0 <= 0.
    }

    pub fn clamp_immunity_by_age(&mut self) {
        self.immunity.0 = f32::max(self.immunity.0, self.get_maximum_immunity_for_age());
    }

    pub fn get_maximum_immunity_for_age(&self) -> f32 {
        Self::get_immunity_for_age(self.age).max()
    }

    pub fn get_minimum_immunity_for_age(&self) -> f32 {
        Self::get_immunity_for_age(self.age).min()
    }

    pub fn apply_health_effect(&mut self) {
        match self.health {
            HealthState::Infected { days_until_sick: _ } => self.immunity.0 -= 0.10,
            HealthState::Sick {
                days_until_recovering: _,
            } => self.immunity.0 -= 0.50,
            HealthState::Recovering {
                days_until_healthy: _,
            } => self.immunity.0 += 0.10,
            HealthState::Healthy => self.immunity.0 += 0.05,
        }

        self.clamp_immunity_by_age();
    }

    pub fn advance_health_by_day(&mut self) {
        self.health = self.health.next();
    }

    pub fn give_birth_if_suitable(&mut self, other: &Entity) {
        if !(20..=40).contains(&self.age.0) || !(20..=40).contains(&other.age.0) {
            return;
        }

        let mut newborns = vec![];

        for _ in 0..MAX_CHILDREN_PER_BIRTH {
            let child_position = Position {
                x: (self.position.x + other.position.x) / 2,
                y: (self.position.y + other.position.y) / 2,
            };

            newborns.push(Entity {
                position: child_position,
                speed: rand::random(),
                direction: rand::random(),
                health: HealthState::Healthy,
                age: Age(0),
                immunity: Immunity(ImmunityRange::Low.max()),
                current_children: vec![],
            })
        }

        self.current_children.extend(newborns);
    }
}
