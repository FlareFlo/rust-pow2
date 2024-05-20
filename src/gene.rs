#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Gene {
    G,
    Y,
    H,
    X,
    W,
}

impl Gene {
    pub const fn breed_weight(self) -> u8 {
        match self {
            Gene::G => 4,
            Gene::Y => 4,
            Gene::H => 4,
            Gene::X => 6,
            Gene::W => 6,
        }
    }

    pub const fn score(self) -> i8 {
        match self {
            Gene::G => 16,
            Gene::Y => 16,
            Gene::H => 8,
            Gene::X => 0,
            Gene::W => 0,
        }
    }

    pub fn iter_all() -> impl Iterator<Item = Self> {
        [Self::G, Self::Y, Self::H, Self::X, Self::W].into_iter()
    }

    pub const fn from_char(c: char) -> Self {
        match c.to_ascii_uppercase() {
            'G' => Self::G,
            'Y' => Self::Y,
            'H' => Self::H,
            'X' => Self::X,
            'W' => Self::W,
            _ => {
                unreachable!()
            }
        }
    }

    pub const fn to_char(self) -> char {
        match self {
            Gene::G => 'G',
            Gene::Y => 'Y',
            Gene::H => 'H',
            Gene::X => 'X',
            Gene::W => 'W',
        }
    }
}
