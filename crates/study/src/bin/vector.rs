struct Order {
    id: i32,
    symbol: String,
}

fn main() {
    let mut orders = vec![];

    orders.push(Order {
        id: 1,
        symbol: String::from("TSLA"),
    });

    orders.push(Order {
        id: 2,
        symbol: String::from("NVDL"),
    });

    for or in &orders {
        println!("id={}, symbol={}", or.id, or.symbol)
    }
}
