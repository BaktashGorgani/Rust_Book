use std::thread;
//use std::time::Duration;

//fn main() {
//    let handle = thread::spawn(|| {
//        for i in 1..10 {
//            println!("hi number {i} from the spawned thread!");
//            thread::sleep(Duration::from_millis(1));
//        }
//    });
//
//    handle.join().unwrap();
//
//    for i in 1..5 {
//        println!("hi number {i} from the main thread!");
//        thread::sleep(Duration::from_millis(1));
//    }
//
//    //handle.join().unwrap();
//}
//
//fn main() {
//    let v = vec![1, 2, 3];
//
//    let handle = thread::spawn(move || {
//        println!("Here's a vector: {v:?}");
//    });
//
//    handle.join().unwrap();
//}

fn main() {
    let mut n = 1;
    let t = thread::spawn(move || {
        n = n + 1;
        println!("{n}");
        thread::spawn(move || {
            n = n + 1;
            println!("{n}");
        })
    });
    n = n + 1;
    t.join().unwrap().join().unwrap();
    println!("{n}")
}
