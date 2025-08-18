use std::thread;

//fn main() {
//    let list = vec![1, 2, 3];
//
//    println!("Before defining closure: {list:?}");
//
//    let only_borrows = || println!("From closure: {list:?}");
//
//    println!("Before calling closure: {list:?}");
//    only_borrows();
//    println!("After calling closure: {list:?}");
//}

fn main() {
    let list = vec![1, 2, 3];
    println!("Before defining closure: {list:?}");

    thread::spawn(move || println!("From thread: {list:?}"))
        .join()
        .unwrap();

    //let mut borrows_mutably = || list.push(7);

    //println!("Between closure def and call: {list:?}");

    //borrows_mutably();
    //println!("After calling closure: {list:?}");
}
