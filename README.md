# monument_parser

**The parser was created to update and support information about immovable cultural monuments of Ukraine in a single format.**  

**Installation**  
Add the parser_rickneelee crate as a dependency in your Rust project's Cargo.toml file:  
[dependencies]  
monument_parser = "1.0.0"  

**Each cultural object has a building type, name, location, dating and purpose type.**    

**Building Type:** One word describing the building itself.   
For example: "church", "house", "grave", "crypt", "fort", "complex", "monument", etc.  
Format: This field denotes the type of the monument and should consist of alphabetical characters (uppercase or lowercase).  
Rules: Use only letters from 'A' to 'Z' (uppercase) or 'a' to 'z' (lowercase) to define the type of the monument.  

**Name:** the name of the object itself.  
For example: "St. Paul", "Residential", "Zalizna Voda", "Shyroka Balka" and so on.  
Format: The name of the monument can include letters (uppercase or lowercase), spaces, periods, and apostrophes.  
Rules: Ensure that the name adheres to the character set specified: 'A' to 'Z' (uppercase), 'a' to 'z' (lowercase), '.', ' ', and '''.  

**Location:** the settlement where the monument is located.  
For example: "the city of Kyiv", "the village of Chornomorske", "the village of Hatne", etc.  
Format: The location field specifies the settlement type and name. Settlement types include cities, towns, or villages.  
Rules: Use codes '1', '2', or '3' to represent the settlement type followed by the name of the settlement (letters, spaces, periods, apostrophes).  
"1" represents "The city of [name]"  
"2" represents "The town of [name]"  
"3" represents "The village of [name]"  

**Dating**: Two numbers describing the century (must not exceed 21), the parser formats these dates as the first year of the first specified century and the last year of the second century.  
For example: The monument was built between the 9th and 11th centuries. The input data should contain the numbers "9" and "11", the parser will output "801-1100".  
Format: Dating specifies the time frame in centuries (e.g., 5th to 7th century) and should be represented as a range between two century values.  
Rules: Ensure that the centuries fall within the range of 1 to 21.  
 
**Purpose type:** type of cultural monument.  
In Ukraine, the following types of monuments are distinguished: architectural, historical, urban planning, monumental art, archaeological, garden and park art, landscape, science and technology.  
Format: Purpose types categorize the function or significance of the monument. Multiple purposes can be specified and separated by commas.  
Rules: Use codes '1' to '8' to represent different purposes:  
"1": Architectural  
"2": Historical  
"3": Urban planning  
"4": Monumental art  
"5": Archaeological  
"6": Garden and park art  
"7": Landscape   
"8": Science and technology  
 
**Example of input information: "Church St.John 1 Kyiv 9 11 1,2,5".**    
**To run, use the terminal and enter: "cargo run -- -f test_file.txt"**   
