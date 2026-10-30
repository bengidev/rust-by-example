pub fn describe() {
    // Lifetimes are annotated below with lines denoting the creation
    // and destruction of each variable.
    // `i` has the longest lifetime because its scope entirely encloses
    // both `borrow1` and `borrow2`. The duration of `borrow1` compared
    // to `borrow2` is irrelevant since they are disjoint.
    let i = 3_u8;

    {
        // `borrow_1` lifetime starts.
        let borrow_1 = &i;
        println!("borrow_1: {}", borrow_1);
    } // `borrow1` ends.

    {
        // `borrow_2` lifetime starts.
        let borrow_2 = &i;
        println!("borrow_2: {}", borrow_2);
    } // `borrow_2` ends.
} // Lifetime of i ends.
