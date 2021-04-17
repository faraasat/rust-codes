use std::collections::HashMap;

fn main() {

    // Ownership is only applied to heap storage while Ownership is not applied to stack storage

    // Hashmap is a data structure which stores data in key value pair and they are stored in heap like other collections and it is mandatory that all keys have same datatype
    let mut map = HashMap::new();
    map.insert(String::from("Blue"), 10);
    map.insert(String::from("Yellow"), 50);
    println!("{:?}", map);

    // Construction Hashmap using collect method
    let teams = vec![String::from("Blue"), String::from("Yellow"), String::from("White"), String::from("Red")];
    // if we write more values than keys than the leftover values will be ignored
    let initial_scores = vec![10, 50, 150, 60, 90, 100];
    // first we will write the vector which we wanna use as key and then .iter() to iterate and to join with initial score we use .zip()
    // _ in hashmap means that what the datatype comes it is hashmaps datatype
    let maps : HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();
    println!("{:?}", maps);

    // Ownership and Hashmap, Ownership is applied because String is a heap storage
    let field_key = String::from("Favorite color");
    let field_value = String::from("Blue");
    let mut map1 = HashMap::new();
    map1.insert(field_key, field_value);  // Here ownership is transferred so none of the two variables can be used know
    println!("{:?}", map1);

    // Ownership is not applied because String is a stack storage
    let field_key1 = 45;
    let field_value1 = 67;
    let mut map2 = HashMap::new();
    map2.insert(field_key1, field_value1);  // Here ownership is transferred so none of the two variables can be used know
    println!("{:?}", map2);
    println!("{} {}", field_key1, field_value1);

    // To access particular key in Hashmap and returns Option enum
    let access_key = String::from("Yellow");
    let result = map.get(&access_key);
    println!("{:?}", result);
    let access_key1 = String::from("Green");
    let result1 = map.get(&access_key1);
    println!("{:?}", result1);

    // Accessing value through loop
    for (key, value) in &map{
        println!("{}, {}", key, value);
    }

    // To update or overwrite a key in Hashmap
    map.insert(String::from("Blue"), 200);  // Also used to overwrite the value of a key
    println!("{:?}", map);

    // To store value on the basis of previous keys and this method takes key and if the key is present then it wont take action else add the key and value
    map.entry(String::from("Blue")).or_insert(5000);
    map.entry(String::from("Purple")).or_insert(5000);
    println!("{:?}", map);

    // Updating value based on old value
    let mut maps1 = HashMap::new();
    let my_string = "Hello world wonderful world";
    for word in my_string.split_whitespace() {
        let count = maps1.entry(word).or_insert(0);
        *count += 1;
    }
    println!("{:?}", maps1);

}
