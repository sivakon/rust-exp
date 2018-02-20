use std::collections::HashMap;

#[allow(dead_code)]
fn average(numbers: &[i32]) -> f32 {
    numbers.iter().sum::<i32>() as f32 / numbers.len() as f32
}

#[allow(dead_code)]
fn median(numbers: &mut[i32]) -> i32 {
    numbers.sort();
    let mid = numbers.len() / 2;
    numbers[mid]
}

fn mode(numbers: &[i32]) -> HashMap<i32,i32> {
    let mut occurences = HashMap::new();

    for &value in numbers {
        // we can directly use *occurences.entry(value).or_insert(0) += 1
        let count = occurences.entry(value).or_insert(0); 
        println!("I am inside mode");
        println!("{}", value);
        *count += 1
    }

    for (key, value) in &occurences {
        println!("{}: {}", key, value);
    }
    // occurences.into_iter()
    occurences
    // 5i32
}

fn mode2(numbers: &[i32]) -> HashMap<&i32,i32> {
    let mut occurences = HashMap::new();

    for value in numbers {
        // we can directly use *occurences.entry(value).or_insert(0) += 1
        let count = occurences.entry(value).or_insert(0); 
        println!("I am inside mode2");
        println!("{}", *value);
        *count += 1
    }

    for (key, value) in &occurences {
        println!("{}: {}", key, value);
    }
    // occurences.into_iter()
    occurences
    // 5i32
}

#[allow(unused_doc_comment)]
fn main() {
    println!("Hello, world!");

    #[derive(Debug)]
    enum Cell {
        Int(i32),
        Float(f32),
    }

    let row = vec![
        Cell::Int(3),
        Cell::Float(1.0)
    ];

    println!("{:?}", row);

    let data = "initial contents";

    let s = data.to_string();

    println!("{}", &s);//s also gives me the same result. Because s is a pointer

    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // Note that s1 has been moved here and can no longer be used

    println!("{}", s3);

    // format macro -> does not take ownership of strings
    let s1 = "tic".to_string();
    let s2 = "tac".to_string();
    let s3 = "toe".to_string();

    let s = format!("{}-{}-{}", s1, s2, s3);
    println!("{}",s);

    for _c in "नमस्ते".chars() {
        // println!("{}", c);
    }

    for _c in "नमस्ते".bytes() {
    }

    /**
     * this is an unused documentation step
     * HASHMAP
     */
    
    //Create hashmap
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);  

    println!("{:?}", scores);

    let teams = vec!["Blue".to_string(), "Yellow".to_string()];
    let initial_scores = vec![10, 50];

    let scores: HashMap<&String,&i32> = teams.iter().zip(initial_scores.iter()).collect();

    println!("ownership of teams is not lost {:?}", teams);
    println!("scores are {:?}", scores);

    // Important note
    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);

    println!("{:?}", map);

    // Get something from hashmap
    // is there a way to directly use inside get
    let team_name = "Blue".to_string();
    println!("the value of Blue is {:?}", scores.get(&team_name));

    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }

    // Updating hashmap value based on old value
    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0); //or_insert returns a &mut V
        // in order to assign to that value we must first dereference count using the asterisk
        *count += 1;
    }

    println!("{:?}", map);

    /**
     * Some exercises
     */
    let v = vec![1,2,3,4,5];

    for i in &v {
        println!("{}", i);
    }

    // print the output of mode function
    let s = mode(&v);
    println!("{:?}", s);

    let s2 = mode2(&v);
    println!("{:?}",s2);

}
