fn main() {
    let x = 3;
    let y = 5;

    if x > y {
        println!("x is greater than y");
    } else {
        if x < y {
            println!("x is less than y");
        } else {
            println!("x is equal than y");
        }
    }

    if x > y {
        println!("x is greater than y");
    } else if x < y {
        println!("x is less than y");
    } else {
        println!("x is equal than y");
    }
}
