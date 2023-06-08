use crate::constants::{INFECTED_DAYS, RECOVERY_DAYS, SICK_DAYS};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum HealthState {
    Infected { days_until_sick: usize },
    Sick { days_until_recovering: usize },
    Recovering { days_until_healthy: usize },
    Healthy,
}

impl HealthState {
    pub fn new_sick() -> Self {
        Self::Sick {
            days_until_recovering: SICK_DAYS - 1,
        }
    }
    pub fn new_infected() -> Self {
        Self::Infected {
            days_until_sick: INFECTED_DAYS,
        }
    }

    pub fn new_recovering() -> Self {
        Self::Recovering {
            days_until_healthy: RECOVERY_DAYS - 1,
        }
    }
    pub fn new_healthy() -> Self {
        HealthState::Healthy
    }
    pub fn next(self) -> Self {
        match self {
            HealthState::Infected { days_until_sick } => {
                if days_until_sick == 0 {
                    Self::new_sick()
                } else {
                    Self::Infected {
                        days_until_sick: days_until_sick - 1,
                    }
                }
            }
            HealthState::Sick {
                days_until_recovering,
            } => {
                if days_until_recovering == 0 {
                    Self::new_recovering()
                } else {
                    Self::Sick {
                        days_until_recovering: days_until_recovering - 1,
                    }
                }
            }
            HealthState::Recovering { days_until_healthy } => {
                if days_until_healthy == 0 {
                    Self::new_healthy()
                } else {
                    HealthState::Recovering {
                        days_until_healthy: days_until_healthy - 1,
                    }
                }
            }
            HealthState::Healthy => HealthState::Healthy,
        }
    }
}
