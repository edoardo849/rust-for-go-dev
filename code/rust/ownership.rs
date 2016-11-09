fn ownership_bad() {
    let v = vec![1,2,3,4,5];
    for i in v {
        println!("i is {}",i);
    }
    for i in v {
        println!("i is {}",i);
    }

}

fn ownership_good() {
    let v = vec![1, 2, 3, 4, 5];
    for i in &v {
        println!("This is a reference to {}", i);
    }
    for i in &v {
        println!("This is a reference to {}", i);
    }

}
