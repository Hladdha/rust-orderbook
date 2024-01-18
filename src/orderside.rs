use std::collections::HashMap;
use std::cell::RefCell;
use std::rc::Rc;
use rbtree::RBTree;
use crate::{
    order::Order,
    orderqueue::OrderQueue,
    side::Side,
};


pub struct OrderSide {
    price_tree: Rc<RefCell<RBTree<f64, Side>>>,
    prices: HashMap<String, Rc<RefCell<OrderQueue>>>,
    volume: f64,
    num_orders: usize,
    depth: usize,
}

impl OrderSide {
    pub fn new() -> OrderSide {
        OrderSide {
            price_tree: Rc::new(RefCell::new(RBTree::new())),
            prices: HashMap::new(),
            volume: f64::zero(),
            num_orders: 0,
            depth: 0,
        }
    }

    pub fn len(&self) -> usize {
        self.num_orders
    }

    pub fn depth(&self) -> usize {
        self.depth
    }

    pub fn volume(&self) -> f64 {
        self.volume
    }

    pub fn append(&mut self, o: &Order) -> Option<Rc<RefCell<OrderQueue>>> {
        let price = o.price();
        let str_price = price.to_string();

        let price_queue = self.prices.get(&str_price).cloned();
        match price_queue {
            Some(queue) => {
                let mut queue = queue.borrow_mut();
                queue.append(o);
                self.volume += o.quantity();
                Some(Rc::clone(&queue))
            }
            None => {
                let queue = Rc::new(RefCell::new(OrderQueue::new(price)));
                self.prices.insert(str_price, Rc::clone(&queue));
                self.price_tree.borrow_mut().put(price, Rc::clone(&queue));
                self.depth += 1;
                self.num_orders += 1;
                self.volume += o.quantity();
                Some(queue)
            }
        }
    }

    pub fn remove(&mut self, e: &Rc<RefCell<OrderQueue>>) -> Option<Order> {
        let queue = e.borrow_mut();
        let price = queue.price();
        let str_price = price.to_string();

        if let Some(order) = queue.remove_head() {
            self.num_orders -= 1;
            self.volume -= order.quantity();

            if queue.len() == 0 {
                self.prices.remove(&str_price);
                self.price_tree.borrow_mut().remove(price);
                self.depth -= 1;
            }

            Some(order)
        } else {
            None
        }
    }

    pub fn max_price_queue(&self) -> Option<Rc<RefCell<OrderQueue>>> {
        if self.depth > 0 {
            if let Some(value) = self.price_tree.borrow().get_max() {
                Some(value.clone())
            } else {
                None
            }
        } else {
            None
        }
    }

    pub fn min_price_queue(&self) -> Option<Rc<RefCell<OrderQueue>>> {
        if self.depth > 0 {
            if let Some(value) = self.price_tree.borrow().get_min() {
                Some(value.clone())
            } else {
                None
            }
        } else {
            None
        }
    }

    pub fn less_than(&self, price: f64) -> Option<Rc<RefCell<OrderQueue>>> {
        let tree = &self.price_tree.borrow().tree;
        let mut node = tree.root();

        let mut floor = None;
        while let Some(current_node) = node {
            if tree.comparator()(&price, &current_node.key()) > 0 {
                floor = Some(current_node);
                node = current_node.right();
            } else {
                node = current_node.left();
            }
        }

        floor.map(|node| node.value().unwrap().clone())
    }

    pub fn greater_than(&self, price: f64) -> Option<Rc<RefCell<OrderQueue>>> {
        let tree = &self.price_tree.borrow().tree;
        let mut node = tree.root();

        let mut ceiling = None;
        while let Some(current_node) = node {
            if tree.comparator()(&price, &current_node.key()) < 0 {
                ceiling = Some(current_node);
                node = current_node.left();
            } else {
                node = current_node.right();
            }
        }

        ceiling.map(|node| node.value().unwrap().clone())
    }

    pub fn orders(&self) -> Vec<Rc<RefCell<OrderQueue>>> {
        self.prices.values().cloned().collect()
    }

    pub fn to_string(&self) -> String {
        let mut sb = String::new();
        let mut level = self.max_price_queue();

        while let Some(queue) = level {
            sb.push_str(&format!("\n{} -> {}", queue.borrow().price(), queue.borrow().volume()));
            level = self.less_than(queue.borrow().price());
        }

        sb
    }
}
