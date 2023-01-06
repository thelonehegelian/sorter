fn main() {
    // sort an int array
    let mut num_array = vec![1, 22, 3, 4, 12, 31];
    num_array.sort();
    assert_ne!(num_array, vec![1, 22, 3, 4, 12, 31]);

    println!("{:?}", num_array);

    // sort a float array
    let mut float_array = vec![1.0, 1010.9932, 212.1214, 2321.2113, 3.0, 4.0];
    // sort_by takes an anonymous function to compare to float values, if the values are not comparable it throws an error
    // that is why unwrap is needed as partial_cmp returns an Option
    float_array.sort_by(|a, b| a.partial_cmp(b).unwrap());

    // sort a vector of Struct of Persons (by name and age)
    #[derive(Debug, Eq, Ord, PartialEq, PartialOrd)]
    struct Person {
        name: String,
        age: u8,
    }

    impl Person {
        fn new(name: String, age: u8) -> Person {
            Person {
                name: name,
                age: age,
            }
        }
    }
    // create a vec of Person objects

    let mut people = vec![
        // string literal must be converted to String
        Person::new("Zoe".to_string(), 22),
        Person::new("John".to_string(), 22),
        Person::new("Jim".to_string(), 12),
        Person::new("Sembene".to_string(), 45),
    ];

    // sort by derived natural order: name then age using the derive macro
    // moves Zoe to the end of the list
    people.sort();
    assert_ne!(
        people,
        vec![
            Person::new("Zoe".to_string(), 22),
            Person::new("John".to_string(), 22),
            Person::new("Jim".to_string(), 12),
            Person::new("Sembene".to_string(), 45),
        ]
    );
    println!("{:?}", people);

    // sort by age, Sembene should be the last
    people.sort_by(|a, b| a.age.partial_cmp(&b.age).unwrap());

    println!("{:?}", people);
}
