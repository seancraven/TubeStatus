pub mod tube {

    use crate::tfl_status;
    use std::collections::HashMap;
    use std::hash::{Hash, Hasher};

    // What are the constraints, I can obviously iterate through a vec
    // rather than indexing a small hashmap, but, the eas of use is quite nice, and
    // Lines should have, get_line_short, get_line_long, update status,
    #[derive(Debug, Eq, Clone)]
    pub struct LineStatus {
        pub name: Line,
        pub short: String,
        pub long: Option<String>,
    }
    /// Implement PartialEq for LineStatus.
    /// Compares line status, such that the same line returs true.
    /// Rather than an identical obj.
    ///
    /// Not sure this is what I want but will leave it for now?.
    impl PartialEq for LineStatus {
        fn eq(&self, other: &Self) -> bool {
            self.name == other.name
        }
    }
    /// Implement hash function such that each line has a
    /// unique hash, defined by the line, this
    impl Hash for LineStatus {
        fn hash<H: Hasher>(&self, h: &mut H) {
            self.name.hash(h)
        }
    }
    pub struct Lines {
        map: HashMap<Line, LineStatus>,
    }
    impl Lines {
        fn update(&mut self) {
            if let Ok(status_vec) = tfl_status::scrape() {
                for line_status in status_vec.iter() {
                    self.map.insert(line_status.name, line_status.clone());
                }
            }
        }
    }

    #[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
    pub enum Line {
        ElizabethLine,
        HammersmithCity,
        Jubilee,
        Metropolitan,
        Bakerloo,
        Central,
        Circle,
        District,
        Northern,
        Piccadilly,
        Victoria,
        WaterlooCity,
        LondonOverground,
        DLR,
        Tram,
    }

    impl Line {
        pub fn name(&self) -> &str {
            match *self {
                Line::ElizabethLine => "Elizabeth",
                Line::Circle => "Circle",
                Line::Jubilee => "Jubilee",
                Line::Tram => "Tram",
                Line::Bakerloo => "Bakerloo",
                Line::Central => "Central",
                Line::District => "District",
                Line::HammersmithCity => "Hammersmith",
                Line::Metropolitan => "Metropolitan",
                Line::Northern => "Northern",
                Line::Piccadilly => "Piccadilly",
                Line::Victoria => "Victoria",
                Line::WaterlooCity => "Waterloo",
                Line::LondonOverground => "Overground",
                Line::DLR => "DLR",
            }
        }
        pub fn build_from_str(name: &str) -> Option<Line> {
            if name.contains("Elizabeth") {
                Some(Line::ElizabethLine)
            } else if name.contains("Jubilee") {
                Some(Line::Jubilee)
            } else if name.contains("Tram") {
                Some(Line::Tram)
            } else if name.contains("Bakerloo") {
                Some(Line::Bakerloo)
            } else if name.contains("Central") {
                Some(Line::Central)
            } else if name.contains("District") {
                Some(Line::District)
            } else if name.contains("Hammersmith") {
                Some(Line::HammersmithCity)
            } else if name.contains("Metropolitan") {
                Some(Line::Metropolitan)
            } else if name.contains("Northern") {
                Some(Line::Northern)
            } else if name.contains("Piccadilly") {
                Some(Line::Piccadilly)
            } else if name.contains("Victoria") {
                Some(Line::Victoria)
            } else if name.contains("Waterloo") {
                Some(Line::WaterlooCity)
            } else if name.contains("Overground") {
                Some(Line::LondonOverground)
            } else if name.contains("DLR") {
                Some(Line::DLR)
            } else if name.contains("Circle") {
                Some(Line::Circle)
            } else {
                None
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn partial_eq_test() {
        let status_a = tube::LineStatus {
            name: tube::Line::WaterlooCity,
            short: "dave".into(),
            long: Some("bertie".into()),
        };
        let status_b = tube::LineStatus {
            name: tube::Line::WaterlooCity,
            short: "ernie".into(),
            long: Some("bertie".into()),
        };
        assert_eq!(status_a, status_b);
    }
}
