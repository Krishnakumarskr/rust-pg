
fn main() {
    let mut vec = Vec::new();
    
    for i in 1..11 {
        vec.push(i)
    }

    print!("{:?}\n", vec);

    print!("\nEven values {:?}", print_even_values(&vec));
    print!("{:?}", do_binary_search(&vec, 7, 0, vec.len()-1));
}

fn print_even_values(values: &Vec<i32>) -> Vec<i32> {
    let mut even_values = Vec::new();

    for num in values.iter() {
        if num % 2 == 0 {
            even_values.push(*num)
        }
    }
    return even_values;
}

fn do_binary_search(values: &Vec<i32>, search_item: i32, from: usize, to: usize) -> (bool, i32) {
    if values.is_empty() {
        return (false, -1);
    }

    print!("from is {} and to is {}\n", from, to);

    let length = to - from + 1;
    print!("{}\n", length/2);
    let mid = length/2 + from;
    print!("{}\n", mid);

    let val = match values.get(mid) {
        None =>  -1,
        Some(x) => *x,
    };

    if val == -1 {
        print!("Invalid index, check the logic");
        return (false, -1);
    }

    if val == search_item {
        return (true, mid.try_into().unwrap())
    }

    if search_item < val {
        return do_binary_search(values, search_item, from, mid-1)
    }
    else if search_item > val {
        return do_binary_search(values, search_item, mid+1, to)
    }

    return (false, -1)

}