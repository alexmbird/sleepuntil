



mod su_parser {
    
    extern crate chrono;
    // use chrono::Duration as ChronoDuration;
    // use chrono::DateTime;
    use chrono::*;
    
    use regex::Regex;
    
    // use std::error::Error;
    
    
    // Represent two error options:
    //  1) Parser was correct for timespec but spec was malformed
    //  2) Wrong parser; go on trying others
    #[derive(Debug)]
    enum SuParseError {
        Fatal(String),
        Invalid,
    }

    // Define for convenience; otherwise fn defs are gigantic
    pub type DtResult = Result<
        chrono::datetime::DateTime<chrono::offset::local::Local>,
        SuParseError
    >;
    
    
    pub type SuParserResult = Result<
        chrono::datetime::DateTime<chrono::offset::local::Local>,
        String
    >;
    
    
    
    pub fn parse_hhmm(timespec: &String) -> DtResult {
        let re      = Regex::new(r"^(\d+):(\d+)$").unwrap();
        let caps    = match re.captures(timespec) {
            Some(c)  => c,
            None     => { return Err(SuParseError::Invalid)}
        };
        let hh = caps.at(1).unwrap().parse:: <u32> ().unwrap();
        match hh {
            0...23 => {},
            _      => { return Err(SuParseError::Fatal("Hours must be between 00 and 23".to_string()))}
        }
        let mm = caps.at(2).unwrap().parse:: <u32> ().unwrap();
        match mm {
            0...59 => {},
            _      => { return Err(SuParseError::Fatal("Minutes must be between 00 and 59".to_string()))}
        }
        let dt_orig   = Local::now();
        let dt_target = dt_orig
            .with_hour(hh).unwrap()
            .with_minute(mm).unwrap()
            .with_second(0).unwrap()
            .with_nanosecond(0).unwrap();
            
        if dt_target < dt_orig {
            return Ok(dt_target + Duration::days(1));
        } else {
            return Ok(dt_target);
        }
    }
    
    // pub fn parse_hhmmss(timespec: &String) -> DtResult {
    //     return DateTime::parse_from_rfc2822("Fri, 28 Nov 2014 21:00:09 +0900");
    // }
    
    // Naive 8601; assume local timezone
    pub fn parse_iso8601(timespec: &String) -> DtResult {
        match timespec.parse::<DateTime<Local>>() {
            Ok(dt) => Ok(dt),
            _      => Err(SuParseError::Fatal("Hours must be between 00 and 23".to_string()))
        }
    }
    

    
    pub fn parse(timespec: String) -> SuParserResult {
        
        let parsers: Vec<fn(timespec: &String) -> DtResult> = vec![
            parse_hhmm,
            // parse_hhmmss,
            parse_iso8601,
        ];
        
        // FIXME: know difference between try-next and abort
        for p in parsers {
            let dt: DtResult = p(&timespec);
            match dt {
                Ok(dtr)  => {return Ok(dtr);},
                _        => {},
            }
        }
        
        return Err("Cannot parse timespec".to_string());
    }
    

}




#[cfg(test)]
mod test {
    
    use chrono::*;
    use super::su_parser;
    
    #[test]
    fn test_parse_iso8601() {
        let dt_expected = Local.ymd(2017, 11, 28).and_hms(21, 0, 09);
        // assert_eq!(
        //     dt_expected,
        //     su_parser::parse_iso8601_naive(&"2017-11-28T21:00:09".to_string()).unwrap()
        // );
        assert_eq!(
            dt_expected, 
            su_parser::parse_iso8601(&"2017-11-28T21:00:09Z".to_string()).unwrap()
        );
        assert_eq!(
            dt_expected, 
            su_parser::parse_iso8601(&"2017-11-28T21:00:09+00:00".to_string()).unwrap()
        );
    }


    #[test]
    fn test_parse_hhmm() {
        let dt_expected = Local::now()
            .with_second(0).unwrap()
            .with_nanosecond(0).unwrap()
             + Duration::days(1);
        
        // Good case
        assert_eq!(
            dt_expected.with_hour(10).unwrap().with_minute(59).unwrap(), 
            su_parser::parse_hhmm(&"10:59".to_string()).unwrap()
        );
    }

    //
    // Bad cases
    //
    #[test]
    #[should_panic(expected = "Hours must be between 00 and 23")]
    fn test_parse_hhmm_bad_hours_1() {
        su_parser::parse_hhmm(&"24:00".to_string()).unwrap();
    }

    #[test]
    #[should_panic(expected = "value: Invalid")]
    fn test_parse_hhmm_bad_hours_2() {
        su_parser::parse_hhmm(&"-4:00".to_string()).unwrap();
    }

    #[test]
    #[should_panic(expected = "Minutes must be between 00 and 59")]
    fn test_parse_hhmm_bad_mins_1() {
        su_parser::parse_hhmm(&"00:61".to_string()).unwrap();
    }

    #[test]
    #[should_panic(expected = "value: Invalid")]
    fn test_parse_hhmm_bad_mins_2() {
        su_parser::parse_hhmm(&"00:-1".to_string()).unwrap();
    }




}