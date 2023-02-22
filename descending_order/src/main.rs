use std::collections::BinaryHeap;

fn descending_order(x: u64) -> u64 {
    const RADIX:u32=10;
    
    let mut data: BinaryHeap<_> = x.to_string().chars().map(|c| c.to_digit(RADIX).unwrap()).collect();
    let mut digit = data.pop();
    let mut res: String = "".to_owned();
    while digit != None{
        res.push_str(&(digit.unwrap().to_string()));
        digit = data.pop();
    }

    return res.parse::<u64>().unwrap();
}

#[test]
fn returns_expected() {
    assert_eq!(descending_order(0), 0);
    assert_eq!(descending_order(1), 1);
    assert_eq!(descending_order(15), 51);
    assert_eq!(descending_order(1021), 2110);
    assert_eq!(descending_order(123456789), 987654321);
    assert_eq!(descending_order(145263), 654321);
    assert_eq!(descending_order(1254859723), 9875543221);
}