use std::{fmt, str::FromStr};

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
pub enum Antigen {
    A,
    AB,
    B,
    O,
}

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
pub enum RhFactor {
    Positive,
    Negative,
}

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
pub struct BloodType {
    pub antigen: Antigen,
    pub rh_factor: RhFactor,
}

impl FromStr for BloodType {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.is_empty() {
            return Err(());
        }
        
        let (antigen_str, rh_str) = s.split_at(s.len() - 1);
        
        let antigen = match antigen_str {
            "A" => Antigen::A,
            "B" => Antigen::B,
            "AB" => Antigen::AB,
            "O" => Antigen::O,
            _ => return Err(()),
        };
        
        let rh_factor = match rh_str {
            "+" => RhFactor::Positive,
            "-" => RhFactor::Negative,
            _ => return Err(()),
        };
        
        Ok(BloodType { antigen, rh_factor })
    }
}

impl fmt::Debug for BloodType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let antigen_str = match self.antigen {
            Antigen::A => "A",
            Antigen::B => "B",
            Antigen::AB => "AB",
            Antigen::O => "O",
        };
        
        let rh_str = match self.rh_factor {
            RhFactor::Positive => "+",
            RhFactor::Negative => "-",
        };
        
        write!(f, "{}{}", antigen_str, rh_str)
    }
}

impl BloodType {
    pub fn can_receive_from(self, other: Self) -> bool {
        let rh_compatible = match (self.rh_factor, other.rh_factor) {
            (RhFactor::Negative, RhFactor::Positive) => false,
            _ => true,
        };

        let antigen_compatible = match (self.antigen, other.antigen) {
            (Antigen::AB, _) => true,
            (Antigen::A, Antigen::A) | (Antigen::A, Antigen::O) => true,
            (Antigen::B, Antigen::B) | (Antigen::B, Antigen::O) => true,
            (Antigen::O, Antigen::O) => true,
            _ => false,
        };

        rh_compatible && antigen_compatible
    }

    pub fn donors(self) -> Vec<Self> {
        let mut valid_donors = Vec::new();
        let antigens = [Antigen::A, Antigen::B, Antigen::AB, Antigen::O];
        let rh_factors = [RhFactor::Positive, RhFactor::Negative];

        for &antigen in &antigens {
            for &rh_factor in &rh_factors {
                let other = BloodType { antigen, rh_factor };
                if self.can_receive_from(other) {
                    valid_donors.push(other);
                }
            }
        }
        
        valid_donors
    }

    pub fn recipients(self) -> Vec<Self> {
        let mut valid_recipients = Vec::new();
        let antigens = [Antigen::A, Antigen::B, Antigen::AB, Antigen::O];
        let rh_factors = [RhFactor::Positive, RhFactor::Negative];

        for &antigen in &antigens {
            for &rh_factor in &rh_factors {
                let other = BloodType { antigen, rh_factor };
                if other.can_receive_from(self) {
                    valid_recipients.push(other);
                }
            }
        }
        
        valid_recipients
    }
}