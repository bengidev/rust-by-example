use std::str::FromStr;

pub fn describe() {
    let count_item = get_count_item("3 chairs");
    println!("count_item result: {:?}", count_item);
}

fn get_count_item(s: &str) -> (u64, &str) {
    let mut it = s.split(' ');

    let (Some(count_str), Some(item)) = (it.next(), it.next()) else {
        panic!("Can't segment count item pair: '{s}'");
    };

    let Ok(count) = u64::from_str(count_str) else {
        panic!("Can't parse integer: '{count_str}'");
    };

    (count, item)
}
