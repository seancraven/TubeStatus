use crate::tfl_status;
use std::collections::hash_map::Iter;
use std::collections::HashMap;
use std::fmt;
use std::hash::Hash;

// What are the constraints, I can obviously iterate through a vec
// rather than indexing a small hashmap, but, the eas of use is quite nice, and
// Lines should have, get_line_short, get_line_long, update status,
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct LineStatus {
    pub name: Line,
    pub short: String,
    pub long: Option<String>,
}
impl fmt::Display for LineStatus {
    /// Display the line status in a human readable format.
    /// Only when there is a delay will the long status be displayed.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.long {
            Some(ref long) => write!(
                f,
                "{}:\n\n Short summary: {} \n\n Long summary: {}",
                self.name.name(),
                self.short,
                long
            ),
            None => write!(f, "{}:\n\n Short summary: {}", self.name.name(), self.short),
        }
    }
}

/// Struct to contain all tube information that defines the interface.
pub struct Lines {
    map: HashMap<Line, LineStatus>,
}
impl Lines {
    /// Scrapes the tfl website for status updates to the tube lines,
    /// Populates the map with this information.
    ///
    /// The implementaion is slow and simple, uses a private hashmap.
    pub async fn update(&mut self) {
        if let Ok(status_vec) = tfl_status::fetch().await {
            for line_status in status_vec.iter() {
                self.map.insert(line_status.name, line_status.clone());
            }
        }
    }
    /// Makes new empty Lines struct, call update, to get tube info.
    pub fn new() -> Lines {
        let lines = Lines {
            map: HashMap::with_capacity(15),
        };
        return lines;
    }
    pub fn iter(&mut self) -> Iter<Line, LineStatus> {
        self.map.iter()
    }
    pub fn get(&mut self, line_as_key: &Line) -> Option<&LineStatus> {
        self.map.get(line_as_key)
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
        let name = name.to_string().to_lowercase();
        if name.contains("elizabeth") {
            Some(Line::ElizabethLine)
        } else if name.contains("jubilee") {
            Some(Line::Jubilee)
        } else if name.contains("tram") {
            Some(Line::Tram)
        } else if name.contains("bakerloo") {
            Some(Line::Bakerloo)
        } else if name.contains("central") {
            Some(Line::Central)
        } else if name.contains("district") {
            Some(Line::District)
        } else if name.contains("hammersmith") {
            Some(Line::HammersmithCity)
        } else if name.contains("metropolitan") {
            Some(Line::Metropolitan)
        } else if name.contains("northern") {
            Some(Line::Northern)
        } else if name.contains("piccadilly") {
            Some(Line::Piccadilly)
        } else if name.contains("victoria") {
            Some(Line::Victoria)
        } else if name.contains("waterloo") {
            Some(Line::WaterlooCity)
        } else if name.contains("overground") {
            Some(Line::LondonOverground)
        } else if name.contains("dlr") {
            Some(Line::DLR)
        } else if name.contains("circle") {
            Some(Line::Circle)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod test {
    use crate::tube;

    #[test]
    fn test_lines() {
        let mut lines = tube::Lines::new();
        lines.update();
        for (line, status) in lines.iter() {
            assert_eq!(line, &status.name)
        }
    }
}
