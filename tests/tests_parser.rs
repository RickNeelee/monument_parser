#[cfg(test)] 
mod tests {
    use parser_rickneelee::monument_parser;
    
    #[test]
    fn test_valid_monument_1() {
        let input = "Church 9 11";
        match monument_parser::monument(input) {
            Ok((name, (first_year, second_year))) => {
                assert_eq!(name, "Church");
                assert_eq!(first_year, 801); 
                assert_eq!(second_year, 1100); 
            }
            Err(_) => {
                panic!("Parsing shouldn't fail here.");
            }
        }
    }
    
    #[test]
    fn test_valid_monument_2() {
        let input = "Crypt 1 4";
        match monument_parser::monument(input) {
            Ok((name, (first_year, second_year))) => {
                assert_eq!(name, "Crypt");
                assert_eq!(first_year, 1); 
                assert_eq!(second_year, 400); 
            }
            Err(_) => {
                panic!("Parsing shouldn't fail here.");
            }
        }
    }

    #[test]
    fn test_invalid_monument_1() {
        let input = "Church 9 22";
        match monument_parser::monument(input) {
            Ok(_) => {
                panic!("Parsing should fail here.");
            }
            Err(_) => {
                //must fail
            }
        }
    }
    
    
    #[test]
    fn test_invalid_monument_2() {
        let input = "Invalid Monument";
        match monument_parser::monument(input) {
            Ok(_) => {
                panic!("Parsing should fail here.");
            }
            Err(_) => {
                // must fail
            }
        }
    }
}
