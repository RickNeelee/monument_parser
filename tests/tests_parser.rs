#[cfg(test)]
mod tests {
    use monument_parser::monument_parser;

    #[test]
    fn test_century_valid() {
        assert!(monument_parser::century("21").is_ok());
    }
    
    #[test]
    #[should_panic]
    fn test_century_invalid() {
        assert!(monument_parser::century("22").is_ok());
    }

    #[test]
    fn test_monument_type_valid() {
        assert_eq!(monument_parser::monument_type("Monument").unwrap(), "Monument".to_string());
    }

    #[test]
    #[should_panic]
    fn test_monument_type_invalid() {
        monument_parser::monument_type("123").unwrap();
    }

    #[test]
    fn test_monument_name_valid() {
        assert_eq!(monument_parser::monument_name("St. Paul's Cathedral").unwrap(), "St. Paul's Cathedral".to_string());
    }

    #[test]
    #[should_panic]
    fn test_monument_name_invalid() {
        assert!(monument_parser::monument_name("12345").is_ok());
    }

    #[test]
    fn test_settlement_location_valid() {
        assert_eq!(monument_parser::settlement_location("1 Kyiv").unwrap(), "The city of Kyiv".to_string());
    }

    #[test]
    #[should_panic]
    fn test_settlement_location_invalid() {
        monument_parser::settlement_location("4 Odesa").unwrap();
    }
    
    #[test]
    fn test_purpose_types_valid() {
        assert_eq!(
            monument_parser::purpose_types("1,2,5").unwrap(),
            vec!["Architectural".to_string(), "Historical".to_string(), "Archaeological".to_string()]
        );
    }

    #[test]
    #[should_panic]
    fn test_purpose_types_invalid() {
        monument_parser::purpose_types("9,10").unwrap();
    }

    #[test]
    fn test_monument_valid_input() {
        assert!(monument_parser::monument("Church St.John 1 Kyiv 9-11 1,2,5").is_ok());
    }

    #[test]
    #[should_panic]
    fn test_monument_invalid_location() {
        assert!(monument_parser::monument("Church St.John 4 Kyiv 9-11 1,2,5").is_ok());
    }

    #[test]
    #[should_panic]
    fn test_monument_invalid_century_range() {
        assert!(monument_parser::monument("Church St.John 1 Kyiv 0-11 1,2,5").is_ok());
    }

    #[test]
    #[should_panic]
    fn test_monument_empty_purpose() {
        assert!(monument_parser::monument("Church St.John 1 Kyiv 9-11").is_ok());
    }

}