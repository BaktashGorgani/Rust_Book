use std::slice;

static mut COUNTER: u32 = 0;

unsafe fn dangerous() {}

/// SAFETY: Calling this from more than a single thread at a time is undefined
/// behavior, so you *must* guarantee you only call it from a single thread at
/// a time.
unsafe fn add_to_count(inc: u32) {
    unsafe {
        COUNTER += inc;
    }
}

//fn split_at_mut(values: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
//    let len = values.len();
//
//    assert!(mid <= len);
//
//    (&mut values[..mid], &mut values[mid..])
//}

#[allow(dead_code)]
fn split_at_mut(values: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = values.len();
    let ptr = values.as_mut_ptr();

    assert!(mid <= len);

    unsafe {
        (
            slice::from_raw_parts_mut(ptr, mid),
            slice::from_raw_parts_mut(ptr.add(mid), len - mid),
        )
    }
}

unsafe extern "C" {
    //fn abs(input: i32) -> i32;
    #[allow(dead_code)]
    safe fn abs(input: i32) -> i32;
}

fn main() {
    let mut num = 5;

    let r1 = &raw const num;
    let r2 = &raw mut num;

    //println!("r1 is: {}", *r1);
    //println!("r2 is: {}", *r2);

    //println!("Absolute value of -3 according to C: {}", abs(-3));

    unsafe {
        println!("r1 is: {}", *r1);
        println!("r2 is: {}", *r2);

        dangerous();
        //println!("Absolute value of -3 according to C: {}", abs(-3))
        add_to_count(3);
        // SAFETY: This is only called from a single thread in `main`.
        println!("COUNTER: {}", *(&raw const COUNTER))
    }
}
