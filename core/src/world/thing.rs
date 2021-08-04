use super::{Field, Location, Npc, Region};
use std::fmt;

#[derive(Debug)]
pub enum Thing {
    Location(Location),
    Npc(Npc),
    Region(Region),
}

pub struct SummaryView<'a>(&'a Thing);

pub struct DetailsView<'a>(&'a Thing);

impl Thing {
    pub fn name(&self) -> &Field<String> {
        match self {
            Thing::Location(location) => &location.name,
            Thing::Npc(npc) => &npc.name,
            Thing::Region(region) => &region.name,
        }
    }

    pub fn display_summary(&self) -> SummaryView {
        SummaryView(self)
    }

    pub fn display_details(&self) -> DetailsView {
        DetailsView(self)
    }
}

impl From<Location> for Thing {
    fn from(location: Location) -> Thing {
        Thing::Location(location)
    }
}

impl From<Npc> for Thing {
    fn from(npc: Npc) -> Thing {
        Thing::Npc(npc)
    }
}

impl From<Region> for Thing {
    fn from(region: Region) -> Thing {
        Thing::Region(region)
    }
}

impl<'a> fmt::Display for SummaryView<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.0 {
            Thing::Location(l) => write!(f, "{}", l.display_summary()),
            Thing::Npc(n) => write!(f, "{}", n.display_summary()),
            Thing::Region(_) => unimplemented!(),
        }
    }
}

impl<'a> fmt::Display for DetailsView<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.0 {
            Thing::Location(l) => write!(f, "{}", l.display_details()),
            Thing::Npc(n) => write!(f, "{}", n.display_details()),
            Thing::Region(_) => unimplemented!(),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn name_test() {
        {
            let mut location = Location::default();
            location.name.replace("The Prancing Pony".to_string());
            assert_eq!(
                Some(&"The Prancing Pony".to_string()),
                Thing::from(location).name().value()
            );
        }

        {
            let mut region = Region::default();
            region.name.replace("Bray".to_string());
            assert_eq!(
                Some(&"Bray".to_string()),
                Thing::from(region).name().value()
            );
        }

        {
            let mut npc = Npc::default();
            npc.name.replace("Frodo Underhill".to_string());
            assert_eq!(
                Some(&"Frodo Underhill".to_string()),
                Thing::from(npc).name().value()
            );
        }
    }

    #[test]
    fn into_test() {
        assert!(matches!(Location::default().into(), Thing::Location(_)));
        assert!(matches!(Npc::default().into(), Thing::Npc(_)));
        assert!(matches!(Region::default().into(), Thing::Region(_)));
    }
}
