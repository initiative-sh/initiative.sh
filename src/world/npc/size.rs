use std::fmt;

#[derive(Debug, PartialEq)]
pub enum Size {
    // Tiny { height: u16, weight: u16 },
    // Small { height: u16, weight: u16 },
    Medium { height: u16, weight: u16 },
    // Large { height: u16, weight: u16 },
    // Huge { height: u16, weight: u16 },
    // Gargantuan { height: u16, weight: u16 },
}

impl Size {
    pub fn height_weight(&self) -> (u16, u16) {
        match self {
            Self::Medium { height, weight } => (*height, *weight),
        }
    }

    pub fn height(&self) -> u16 {
        self.height_weight().0
    }

    pub fn height_ft_in(&self) -> (u8, u8) {
        let height = self.height();
        ((height / 12) as u8, (height % 12) as u8)
    }

    pub fn weight(&self) -> u16 {
        self.height_weight().1
    }

    pub fn name(&self) -> &'static str {
        match self {
            Self::Medium { .. } => "medium",
        }
    }
}

impl fmt::Display for Size {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let (height_ft, height_in) = self.height_ft_in();
        write!(
            f,
            "{}'{}\", {} lbs ({})",
            height_ft,
            height_in,
            self.weight(),
            self.name(),
        )
    }
}
