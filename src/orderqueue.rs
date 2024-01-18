use std::collections::LinkedList;
use std::fmt;
use crate::order::Order;

pub struct OrderQueue {
    volume: f64,
    price: f64,
    orders: LinkedList<Order>,
}

impl OrderQueue {
    pub fn new(price: f64) -> OrderQueue {
        OrderQueue {
            price,
            volume: f64::zero(),
            orders: LinkedList::new(),
        }
    }

    pub fn len(&self) -> usize {
        self.orders.len()
    }

    pub fn price(&self) -> f64 {
        self.price
    }

    pub fn volume(&self) -> f64 {
        self.volume
    }

    pub fn head(&self) -> Option<&Order> {
        self.orders.front()
    }

    pub fn tail(&self) -> Option<&Order> {
        self.orders.back()
    }

    pub fn append(&mut self, o: Order) {
        self.volume += o.quantity();
        self.orders.push_back(o);
    }

    pub fn update(&mut self, index: usize, o: Order) {
        if let Some(mut node) = self.orders.iter_mut().nth(index) {
            self.volume -= node.quantity();
            self.volume += o.quantity();
            *node = o;
        }
    }

    pub fn remove(&mut self, index: usize) -> Option<Order> {
        if let Some(node) = self.orders.remove(index) {
            self.volume -= node.quantity();
            Some(node)
        } else {
            None
        }
    }
}

impl fmt::Display for OrderQueue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut result = format!(
            "\nqueue length: {}, price: {}, volume: {}, orders:",
            self.len(),
            self.price(),
            self.volume()
        );

        for order in &self.orders {
            result.push_str(&format!(
                "\n\tid: {}, volume: {}, time: {}",
                order.id(),
                order.quantity(),
                order.price()
            ));
        }

        write!(f, "{}", result)
    }
}
