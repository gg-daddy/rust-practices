fn main() {
    let mut name = String::from("Alice");
    //The take function will empty the memory and will replace it with the default value for the variable (which is "" for Strings)
    //and returns the actual value in the variable of taken_name.
    let taken_name = std::mem::take(&mut name);
    println!("Taken name: {}", taken_name);
    println!("Remaining name: {}", name);
}
