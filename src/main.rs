mod matching_engine;
use matching_engine::orderbook::{Order, BidOrAsk, Orderbook};
use matching_engine::engine::{TradingPair, MatchingEngine};
use rust_decimal_macros::dec;


fn main() {
    // let price = Price::new(50.5);
    // println!("{:?}",price);

    // let mut limit = Limit::new(65.0);
    let buy_order_from_alice = Order::new(BidOrAsk::Bid,5.5);
    let buy_order_from_bob = Order::new(BidOrAsk::Bid,2.45);
    
    // let sell_order = Order::new(BidOrAsk::Ask,5.5);
    // limit.add_order(buy_order);
    // limit.add_order(sell_order);

    let mut orderbook = Orderbook::new();
    orderbook.add_limit_order(dec!(4.4), buy_order_from_alice);
    orderbook.add_limit_order(dec!(4.4), buy_order_from_bob);

    let sell_order = Order::new(BidOrAsk::Ask,6.5);
    orderbook.add_limit_order(dec!(20.0), sell_order);

    // println!("{:?}",limit);
    // println!("{:?}",orderbook);

    let mut engine = MatchingEngine::new();
    let pair = TradingPair::new("BTC".to_string(), "USD".to_string());
    engine.add_new_market(pair.clone());

    let buy_order = Order::new(BidOrAsk::Bid, 6.5);
    // let eth_pair = TradingPair::new("ETH".to_string(), "USD".to_string());
    engine.place_limit_order(pair, dec!(10.000), buy_order).unwrap();
    // engine.place_limit_order(eth_pair, 10.000, buy_order).unwrap();

}
