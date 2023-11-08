peg::parser! {
  pub grammar monument_parser() for str {
       rule century() -> u32
           = n:$(['0'..='9']+) {?
               let century_num = n.parse::<u32>();
               if let Ok(century) = century_num {
                   if century <= 21 &&  century > 0{
                       Ok(century)
                   } else {
                       Err("Century must be between 1 and 21")
                   }
               } else {
                   Err("Invalid century format")
               }
           }

       pub rule monument() -> (String, (u32, u32))
           = name:$(['a'..='z' | 'A'..='Z']+) _ first:century() _ second:century() {
               (name.to_string(), (first * 100 - 99, second  * 100))
           }

       rule _() = [' ' | '\t' | '\n']*
   }
}

fn main() {
   let input = "Church 9 11";
   match monument_parser::monument(input) {
       Ok((name, (first_year, second_year))) => {
           println!("The name of the monument: {}", name);
           println!("Dating: {}-{}", first_year, second_year);
       }
       Err(e) => {
           println!("Error: {}", e);
       }
   }
}
