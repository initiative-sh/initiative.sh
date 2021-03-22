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

#[cfg(test)]
mod test_size {
    use super::*;

    #[test]
    fn height_weight_test() {
        assert_eq!((71, 140), size().height_weight());
    }

    #[test]
    fn height_test() {
        assert_eq!(71, size().height());
    }

    #[test]
    fn height_ft_in_test() {
        assert_eq!((5, 11), size().height_ft_in());
    }

    #[test]
    fn weight_test() {
        assert_eq!(140, size().weight());
    }

    #[test]
    fn name_test() {
        assert_eq!("medium", size().name());
    }

    fn size() -> Size {
        Size::Medium {
            height: 71,
            weight: 140,
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

#[cfg(test)]
mod test_display_for_size {
    use super::*;

    #[test]
    fn fmt_test() {
        assert_eq!(
            "5'11\", 140 lbs (medium)",
            format!(
                "{}",
                Size::Medium {
                    height: 71,
                    weight: 140
                },
            ),
        );
    }
}
