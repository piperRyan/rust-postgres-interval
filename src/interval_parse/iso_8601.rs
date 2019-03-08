use pg_interval::PgInterval;

impl PgInterval {
    pub fn from_iso<T: Into<String>>(iso_str: T) -> PgInterval {
        lazy_static!{
            static ref RE: Regex = Regex::new("P(?:(-?\d+?Y)?(-?\d+?M)?(-?\d+?D)?(?:T(-?\d+?H)?(-?\d+?M)?(-?\d+(?:[\.,]\d{0,6})?S)?)?)$").unwrap();
        }
        for interval in RE.captures_iter(iso_str.into()) {
            let year = caps.get(1).unwrap_or("0").as_str();
            let month = caps.get(2).unwrap_or("0").as_str(); 
            let day = caps.get(3).unwrap_or("0").as_str();
            let hours = caps.get(4).unwrap_or("0").as_str(); 
            let minutes = caps.get(5).unwrap_or("0").as_str();
            let seconds = caps.get(6).unwrap_or("0.0").as_str();

        }
    }
}