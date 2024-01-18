use std::collections::{HashMap, LinkedList};

use crate::{
    order::Order,
    orderside::OrderSide,
    side::Side,
};


pub struct OrderBook {
    orders: HashMap<String, LinkedList<Order>>,
    asks: OrderSide,
    bids: OrderSide,
}

impl OrderBook {
    pub fn new() -> OrderBook {
        OrderBook {
            orders: HashMap::new(),
            asks: OrderSide::new(),
            bids: OrderSide::new(),
        }
    }

    pub fn process_market_order(
        &mut self,
        side: Side,
        quantity: f64,
    ) -> (Vec<Order>, Option<Order>, f64, f64) {
        let (done, partial, partial_qty, quantity_left) = match side {
            Side::Buy => self.asks.process_market_order(quantity),
            Side::Sell => self.bids.process_market_order(quantity),
        };

        (done, partial, partial_qty, quantity_left)
    }

    pub fn process_limit_order(
        &mut self,
        side: Side,
        order_id: String,
        quantity: f64,
        price: f64,
    ) -> (Vec<Order>, Option<Order>, f64, Option<Order>, f64) {
        let (done, partial, partial_qty) = match side {
            Side::Buy => self.asks.process_limit_order(order_id, quantity, price),
            Side::Sell => self.bids.process_limit_order(order_id, quantity, price),
        };

        (done, partial, partial_qty, None, f64::zero())
    }

    pub fn order(&self, order_id: &str) -> Option<&Order> {
        self.orders.get(order_id)?.front()
    }

    pub fn depth(&self) -> (Vec<PriceLevel>, Vec<PriceLevel>) {
        let asks = self.asks.depth();
        let bids = self.bids.depth();
        (asks, bids)
    }

    pub fn cancel_order(&mut self, order_id: &str) -> Option<Order> {
        if let Some(orders) = self.orders.get_mut(order_id) {
            let canceled_order = orders.pop_front()?;
            match canceled_order.side {
                Side::Buy => self.asks.cancel_order(order_id),
                Side::Sell => self.bids.cancel_order(order_id),
            };
            Some(canceled_order)
        } else {
            None
        }
    }

    pub fn calculate_market_price(&self, side: Side, quantity: f64) -> Result<f64, &'static str> {
        let price = match side {
            Side::Buy => self.asks.calculate_market_price(quantity),
            Side::Sell => self.bids.calculate_market_price(quantity),
        };
        Ok(price?)
    }
}