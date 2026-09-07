pub fn describe() {
    // Closures as input parameters are possible, so returning closures as output parameters should also be possible.
    // However, anonymous closure types are, by definition, unknown, so we have to use impl Trait to return them.
    let fn_plain = create_fn();
    let fn_once = create_fnonce();
    let mut fn_mut = create_fnmut();

    fn_plain();
    fn_once();
    fn_mut();
}

fn create_fn() -> impl Fn() {
    let text = "Fn".to_owned();

    move || println!("This is a: {}", text)
}

fn create_fnmut() -> impl FnMut() {
    let text = "FnMut".to_owned();

    move || println!("This is a: {}", text)
}

fn create_fnonce() -> impl FnOnce() {
    let text = "FnOnce".to_owned();

    move || println!("This is a: {}", text)
}
