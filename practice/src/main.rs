use std::{collections::HashMap, sync::mpsc::{self, RecvError}, thread::spawn, vec};


fn main() {
    // let mut vec = Vec::new();
    
    // for i in 1..11 {
    //     vec.push(i)
    // }

    // print!("{:?}\n", vec);

    // print!("\nEven values {:?}", print_even_values(&vec));
    // print!("{:?}", do_binary_search(&vec, 1, 0, vec.len()-1));

    // group_values();
    // multi_thread();
    multi_thread_proj();
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

    if from > values.len() - 1 {
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

fn group_values() {
    let vectors_of_tuple: Vec<(String, i32)> = vec![
        (String::from("key1"), 1),
        (String::from("key2"), 2),
        (String::from("key2"), 2)
    ];

    let hasmapped_values = group_values_impl(&vectors_of_tuple);

    print!("{:?}", hasmapped_values);
    print!("{:?}", vectors_of_tuple);
}

fn group_values_impl(vt: &Vec<(String, i32)>) -> HashMap<String, Vec<i32>> {

    let mut mapped = HashMap::new();
    for items in vt {
        let ( key,  val) = items;
        
        match mapped.get_mut(key) {
            None => {
                let vec_array = vec![*val];
                mapped.insert(key.clone(), vec_array);
            },
            Some(vec_array ) => {
                vec_array.push(*val);
            }
        }
    }

    return mapped;
}

fn multi_thread() {
    let (tx, rx) = mpsc::channel();

    spawn(move || {
        tx.send(String::from("Hello world from a thread")).unwrap();
    });

    let msg = rx.recv().unwrap();

    print!("{}", msg);
}

fn multi_thread_proj() {
    println!("function called");
    let (tx, rx) = mpsc::channel();
    println!("channel created");
    let mut handles = vec![];

    let mut sum: u64 = 0;
    let batch = 10_u64.pow(8) / 8;

    println!("Before for loop, batch={}", batch);

    for i in 0..8 {

        let thread_tx = tx.clone();
        let handle = spawn(move || {
                    print!("Spawned thread {}", i);
            let mut local_sum = 0;
            let from: u64 = (i as u64)* batch + 1;
            let to = ((i as u64) + 1) * batch;

            println!("from is {} and to is {}", from, to);
            
            for val in from..=to {
                local_sum += val;
            }

            thread_tx.send(local_sum).unwrap();
        });

        handles.push(handle);
    }

    println!("After for loop");

    drop(tx);

    for partial_sum in rx {
        sum += partial_sum;
    }


    for handle in handles {
        handle.join().unwrap()
    }

    println!("Total sum is {}", sum);


}