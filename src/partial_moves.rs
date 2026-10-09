pub fn describe() {
    let person = Person {
        name: String::from("Alice"),
        age: Box::new(20),
    };

    // `name` is moved out of person, but `age` is referenced
    let Person { name, ref age } = person;
    println!("the person's age is: {}", age);
    println!("the person's name is: {}", name);

    // Error! borrow of partially moved value: `person` partial move occurs
    //println!("The person struct is {:?}", person);

    // `person` cannot be used but `person.age` can be used as it is not moved
    println!("the person's age from person struct is: {}", person.age);
}

#[derive(Debug)]
struct Person {
    name: String,
    age: Box<u8>,
}

// Error! cannot move out of a type which implements the `Drop` trait
//impl Drop for Person {
//    fn drop(&mut self) {
//        println!("Dropping the person struct {:?}", self)
//    }
//}
