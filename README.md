# parser_rickneelee
The parser was created to update and support information about immovable cultural monuments of Ukraine in a single format.

Each cultural object has a building type, name, location, dating, type and security number.

Building Type: One word describing the building itself.
For example: "church", "house", "grave", "crypt", "fort", "complex", "monument", etc.

Name: the name of the object itself.
For example: "St. Paul", "Residential", "Zalizna Voda", "Shyroka Balka" and so on.

Location: the settlement where the monument is located.
For example: "the city of Kyiv", "the village of Chornomorske", "the village of Hatne", etc.

Dates: Two numbers describing the century (must not exceed 21), the parser formats these dates as the first year of the first specified century and the last year of the second century.
For example: The monument was built between the 9th and 11th centuries. The input data should contain the numbers "9" and "11", the parser will output "801-1100".

Type: type of cultural monument.
In Ukraine, the following types of monuments are distinguished: architectural, historical, urban planning, monumental art, archaeological, garden and park art, landscape, science and technology.

Security number: 5 or 6 digits ending with "-H".

!ONLY TYPE AND DATING ARE REALIZED AT THIS MOMENT"
Example of input information (as of 08/11/2023): "Church 9 11".
