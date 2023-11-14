peg::parser! {
    pub grammar monument_parser() for str {
        pub rule century() -> u32
            = n:$(['0'..='9']+) {?
                let century_num = n.parse::<u32>();
                if let Ok(century) = century_num {
                    if century <= 21 && century > 0 {
                        Ok(century)
                    } else {
                        Err("Century must be between 1 and 21")
                    }
                } else {
                    Err("Invalid century format")
                }
            }

        pub rule monument() -> (String, String, String, (u32, u32), Vec<String>)
            = type_:monument_type() _ name:monument_name() _ location:settlement_location()? _ first:century() _ "-" _ second:century() _ purpose:purpose_types() {
                if purpose.is_empty() {
                    panic!("Purpose types cannot be empty");
                }
                let purpose_types = purpose;
                (type_, name.to_string(), location.unwrap_or_else(|| "Invalid input".to_string()), (first * 100 - 99, second * 100), purpose_types)
            }

        pub rule monument_type() -> String
            = type_:$(['a'..='z' | 'A'..='Z']+) { type_.to_string() }

        pub rule monument_name() -> String
            = name:$(['a'..='z' | 'A'..='Z' | '.' | ' '| '\'' ]+) { name.to_string() }

        pub rule settlement_location() -> String
            = code:$(['1' | '2' | '3']) _ name:monument_name() {
                match code {
                    "1" => format!("The city of {}", name),
                    "2" => format!("The town of {}", name),
                    "3" => format!("The village of {}", name),
                    _ => "".to_string() 
                }
            }

        pub rule purpose_types() -> Vec<String>
            = _ purpose:purpose_type() ** "," {
                purpose
            }
        
        pub rule purpose_type() -> String
            = p:$(['1' | '2' | '3' | '4' | '5' | '6' | '7' | '8']) {
                match p {
                    "1" => "Architectural",
                    "2" => "Historical",
                    "3" => "Urban planning",
                    "4" => "Monumental art",
                    "5" => "Archaeological",
                    "6" => "Garden and park art",
                    "7" => "Landscape",
                    "8" => "Science and technology",
                    _ => "Unknown purpose" 
                }.to_string()
            }
        
        rule _() = [' ' | '\t' | '\n']*
    }
}