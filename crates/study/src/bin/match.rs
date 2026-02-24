enum OrderType {
    Market(u32),
    Limit(f64, u32),
    StopLoss(f64),
}

struct Order {
    id: i32,
    symbol: String,
    order_type: OrderType,
}

impl Order {
    // create new function, order id is incremental number
    pub fn new(symbol: String, order_type: OrderType) -> Self {
        let id = Order::next_id();
        Order {
            id,
            symbol,
            order_type,
        }
    }

    fn next_id() -> i32 {
        static mut COUNTER: i32 = 0;
        unsafe {
            COUNTER += 1;
            COUNTER
        }
    }
}

fn main() {
    let orders = vec![
        Order::new(String::from("TSLA"), OrderType::Limit(400.0, 30)),
        Order::new(String::from("MCST"), OrderType::Limit(1400.0, 10)),
        Order::new(String::from("BRKS"), OrderType::Market(80)),
        Order::new(String::from("APPL"), OrderType::StopLoss(900.0)),
    ];

    for order in orders {
        process_order(&order);
    }
}

fn process_order(order: &Order) {
    match order.order_type {
        OrderType::Market(m) => println!(
            "ID [{}]: [{}] [{}]개 시장가 매수 진행",
            order.id, order.symbol, m
        ),
        OrderType::Limit(price, quantity) if price > 1000.0 => {
            println!(
                "ID [{}]: 고가 종목([{}]) 주의 - $[{}]에 [{}]개 지정가 대기",
                order.id, order.symbol, price, quantity
            )
        }

        OrderType::Limit(price, quantity) => {
            println!(
                "ID [{}]: [{}] $[{}]에 [{}]개 지정가 대기",
                order.id, order.symbol, price, quantity
            )
        }
        OrderType::StopLoss(price) => println!(
            "ID [{}]: [{}] $[{}] 도달 시 자동 매도 예약",
            order.id, order.symbol, price
        ),
    }
}
