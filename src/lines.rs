pub mod structs {
    pub struct TubeLine {
        pub name: Line,
        pub status_message: &'static str,
    }
    impl TubeLine {
        pub fn reset(&mut self) {
            self.status_message = ""
        }
    }

    pub enum Line {
        ElizabethLine,
        HammersmithCity,
        Jubilee,
        Metropolitan,
        Bakerloo,
        Central,
        Circle,
        District,
        Norther,
        Piccadilly,
        Victoria,
        WaterlooCity,
        LondonOverground,
        DLR,
        Tram,
    }
    impl Line {
        pub fn build(name: &str) -> Line {
            // Todo make a line constructor from a webscrape
            return Line::ElizabethLine;
        }
    }
}
