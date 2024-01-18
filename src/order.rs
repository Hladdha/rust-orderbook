use std::fmt;
use crate::side::Side;
use std::time::SystemTime;

pub struct Order {
    side: Side,
    id: String,
    timestamp: SystemTime,
    quantity: f64,
    price: f64,
}

impl Order {
    pub fn new(order_id: &str, side: Side, quantity: f64, price: f64, timestamp: SystemTime) -> Order {
        Order {
            id: order_id.to_string(),
            side,
            quantity,
            price,
            timestamp,
        }
    }

    pub fn id(&self) -> &str {
        &self.id
    }

    pub fn side(&self) -> &Side {
        &self.side
    }

    pub fn quantity(&self) -> f64 {
        self.quantity
    }

    pub fn price(&self) -> f64 {
        self.price
    }

    pub fn time(&self) -> SystemTime {
        self.timestamp
    }
}

impl fmt::Display for Order {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            r#""{}":
            side: {}
            quantity: {}
            price: {}
            time: {}"#,
            self.id(),
            self.side(),
            self.quantity(),
            self.price(),
            "TODO: Format timestamp",
        )
    }
}