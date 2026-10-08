use std::fmt::Display;

pub fn describe() {
    // Allocate an integer on the heap
    let _box2 = Box::new(5_i32);

    // A nested scope:
    {
        // Allocate an integer on the heap
        let _box3 = Box::new(4_i32);

        // `_box3` is destroyed here, and memory gets freed
    }

    // Creating lots of boxes just for fun
    // There's no need to manually free memory!
    for _ in 0_u32..1_000 {
        create_box();
    }

    // `_box2` is destroyed here, and memory gets freed

    let x = ToDrop;
    println!("Made a ToDrop!");
}

fn create_box() {
    // Allocate an integer on the heap
    let _box1 = Box::new(3_i32);

    // `_box1` is destroyed here, and memory gets freed
}

struct ToDrop;

impl Drop for ToDrop {
    fn drop(&mut self) {
        println!("ToDrop is being dropped");
    }
}

impl Display for ToDrop {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}
