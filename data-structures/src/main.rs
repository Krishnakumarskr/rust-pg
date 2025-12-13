mod stack;
fn main() {
    use stack::Stack;

    let mut stack: Stack<i32> = Stack::new();

    stack.push(45);

    let returned_val = stack.pop();  
    match returned_val {
        None => println!("No item found"),
        Some(x) => println!("Item found, {}", x)
    }
    match stack.pop() {
        None => println!("No item found"),
        Some(x) => println!("Item found, {}", x)
    }

    println!("{}", stack.length());
    
}
