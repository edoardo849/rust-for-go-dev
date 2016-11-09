
fn bad() {
    let y: &i32;
    let x = 5;
    y = &x;

    println!("{}", y);
}

fn good() {
    let x = 5;
    let y: &i32;
    y = &x;

    println!("{}", y);
}
