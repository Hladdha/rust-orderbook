use std::ptr;
use std::sync::Arc;

const MAX_NUM_ORDERS: usize  = 10000000001;
const MAX_PRICE: usize = 100000;
const MIN_PRICE: usize = 0;

type Size = usize;
type Price = usize;
type OderId = usize;

#[derive(Clone, Copy)]
struct OrderBookEntry {
    size: Size,
    next: Option<Arc<OrderBookEntry>>,
    trader: [u8; 4],
}

#[derive(Clone, Copy)]
struct PricePoint {
    list_head: Option<Box<OrderBookEntry>>,
    list_tail: Option<Box<OrderBookEntry>>,
}

static mut PRICE_POINTS: [PricePoint; MAX_PRICE + 1] = [PricePoint {
    list_head: None,
    list_tail: None,
}; MAX_PRICE + 1];

static mut CUR_ORDER_ID: OderId = 0;
static mut ASK_MIN: usize = MAX_PRICE + 1; 
static mut BID_MAX: usize = MIN_PRICE - 1; 


static mut ARENA_BOOK_ENTRIES: [OrderBookEntry; MAX_NUM_ORDERS] = [OrderBookEntry {
    size: 0,
    next: None,
    trader: [0; 4],
}; MAX_NUM_ORDERS];

static mut ARENA_PTR: *mut OrderBookEntry = ptr::null_mut();



fn init() {
    unsafe {
        PRICE_POINTS.iter_mut().for_each(|pp| {
            pp.list_head = None;
            pp.list_tail = None;
        });

        ARENA_BOOK_ENTRIES.iter_mut().for_each(|entry| entry.size = 0);
        ARENA_PTR = ARENA_BOOK_ENTRIES.as_mut_ptr(); // Bring the arena pointer into the cache

        CUR_ORDER_ID = 0;
        ASK_MIN = MAX_PRICE + 1;
        BID_MAX = MIN_PRICE - 1;
    }
}

fn pp_insert_order(pp_entry: &mut PricePoint, entry: &mut OrderBookEntry) {
    unsafe {
        if let Some(mut list_tail) = pp_entry.list_tail.take() {
            list_tail.next = Some(Box::new(entry.clone()));
            pp_entry.list_tail = Some(list_tail);
        } else {
            pp_entry.list_head = Some(Box::new(entry.clone()));
            pp_entry.list_tail = pp_entry.list_head.clone();
        }
    }
}

fn execute_trade(symbol: &str, buy_trader: &str, sell_trader: &str, trade_price: Price, trade_size: Size) {
    let exec = Execution {
        symbol: symbol.to_string(),
        price: trade_price,
        size: trade_size,
        side: 0,
        trader: buy_trader.to_string(),
    };


    let exec = Execution {
        side: 1,
        trader: sell_trader.to_string(),
        ..exec
    };

}

fn limit(order: Order) -> OderId {
    let mut book_entry;
    let mut entry;
    let mut pp_entry;
    let price = order.price;
    let order_size = order.size;

    unsafe {
        if order.side == 0 {
            if price >= ASK_MIN {
                pp_entry = &mut PRICE_POINTS[ASK_MIN];
                while price >= ASK_MIN {
                    book_entry = pp_entry.list_head.take();
                    while let Some(mut be) = book_entry {
                        if be.size < order_size {
                            execute_trade(
                                &order.symbol,
                                &order.trader,
                                &be.trader,
                                price,
                                be.size,
                            );
                            order_size -= be.size;
                            book_entry = be.next.take();
                        } else {
                            execute_trade(
                                &order.symbol,
                                &order.trader,
                                &be.trader,
                                price,
                                order_size,
                            );
                            if be.size > order_size {
                                be.size -= order_size;
                                book_entry = Some(be);
                            } else {
                                book_entry = be.next.take();
                                pp_entry.list_head = book_entry.clone();
                            }
                            return CUR_ORDER_ID + 1;
                        }
                    }

                    pp_entry.list_head = None;
                    pp_entry.list_tail = None;
                    pp_entry = &mut PRICE_POINTS[ASK_MIN + 1];
                    ASK_MIN += 1;
                }
            }

            entry = ARENA_PTR.add(CUR_ORDER_ID) as *mut OrderBookEntry;
            entry.size = order_size;
            entry.trader = order.trader;
            pp_insert_order(&mut PRICE_POINTS[price], entry);

            if BID_MAX < price {
                BID_MAX = price;
            }

            return CUR_ORDER_ID + 1;
        } else {
            if price <= BID_MAX {
                pp_entry = &mut PRICE_POINTS[BID_MAX];
                while price <= BID_MAX {
                    book_entry = pp_entry.list_head.take();
                    while let Some(mut be) = book_entry {
                        if be.size < order_size {
                            execute_trade(
                                &order.symbol,
                                &be.trader,
                                &order.trader,
                                price,
                                be.size,
                            );
                            order_size -= be.size;
                            book_entry = be.next.take();
                        } else {
                            execute_trade(
                                &order.symbol,
                                &be.trader,
                                &order.trader,
                                price,
                                order_size,
                            );
                            if be.size > order_size {
                                be.size -= order_size;
                                book_entry = Some(be);
                            } else {
                                book_entry = be.next.take();
                                pp_entry.list_head = book_entry.clone();
                            }
                            return CUR_ORDER_ID + 1;
                        }
                    }

                    pp_entry.list_head = None;
                    pp_entry.list_tail = None;
                    pp_entry = &mut PRICE_POINTS[BID_MAX - 1];
                    BID_MAX -= 1;
                }
            }

            entry = ARENA_PTR.add(CUR_ORDER_ID) as *mut OrderBookEntry;
            entry.size = order_size;
            entry.trader = order.trader;
            pp_insert_order(&mut PRICE_POINTS[price], entry);

            if ASK_MIN > price {
                ASK_MIN = price;
            }

            return CUR_ORDER_ID + 1;
        }
    }
}

fn cancel(order_id: OderId) {
    unsafe {
        ARENA_BOOK_ENTRIES[order_id].size = 0;
    }
}

struct Execution {
    symbol: String,
    price: Price,
    size: Size,
    side: usize,
    trader: String,
}

struct Order {
    symbol: String,
    price: Price,
    size: Size,
    side: usize,
    trader: String,
}