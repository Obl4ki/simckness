use rand::{thread_rng, Rng};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum ImmunityRange {
    Low,
    Normal,
    High,
}

impl ImmunityRange {
    pub fn random_in_range(&self) -> Immunity {
        match self {
            ImmunityRange::Low => Immunity(thread_rng().gen_range(1.0..=3.0)),
            ImmunityRange::Normal => Immunity(thread_rng().gen_range(4.0..=6.0)),
            ImmunityRange::High => Immunity(thread_rng().gen_range(7.0..=10.0)),
        }
    }

    pub fn max(&self) -> f32 {
        match self {
            ImmunityRange::Low => 3.0,
            ImmunityRange::Normal => 6.0,
            ImmunityRange::High => 10.0,
        }
    }

    pub fn min(&self) -> f32 {
        match self {
            ImmunityRange::Low => 0.0,
            ImmunityRange::Normal => 3.0,
            ImmunityRange::High => 6.0,
        }
    }
}

#[derive(Debug, PartialEq, PartialOrd, Clone, Copy)]
pub struct Immunity(pub f32);
impl Immunity {
    pub fn is_low(&self) -> bool {
        self.0 > 0.0 && self.0 < 3.0
    }
    pub fn is_medium(&self) -> bool {
        self.0 >= 3.0 && self.0 < 6.0
    }
}
