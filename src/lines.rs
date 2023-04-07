pub mod Tube {
    #[derive(Debug)]
    pub struct LineStatus {
        pub name: Line,
        pub short: String,
        pub long: String,
    }
    #[derive(Debug)]
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
