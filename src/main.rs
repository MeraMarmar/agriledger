use std::{sync::Mutex, vec};

use blockchainlib::{Blockchain,Wallet, Farmer, Crop, ItemType, WeightUnit, Manufacturer, Order, OrderType, OrderState, now, Distributor, Retailer, Consumer};
use lazy_static::lazy_static;


fn main() {

    //**************************Supply-Chain 1 lifecycle */
    let mut farmer = Farmer::new(Wallet::new(),vec![],vec![],"7ag Ali".to_owned());
    let mut manufacturer = Manufacturer::new(Wallet::new(),vec![],"Basmaty".to_owned());
    let mut distributor = Distributor::new(Wallet::new(), vec![], "wza3ly".to_owned());
    let mut retailer = Retailer::new(Wallet::new(), vec![], "Carefour".to_owned());
    let mut consumer = Consumer::new(Wallet::new(),vec![]);
    let otcn = farmer.cropped_field(Crop::new(ItemType::RICE,30,WeightUnit::TON,now()));
    let mut order = Order::new(ItemType::RICE,6,WeightUnit::TON,now(),OrderType::INCOMING,OrderState::PENDING,otcn.clone());
    manufacturer.place_order(&mut order, &mut farmer);
    farmer.update_order_tracking_state(&order.id, OrderState::PREPARING, &mut manufacturer);
    farmer.update_order_tracking_state(&order.id, OrderState::SHIPPING, &mut manufacturer);
    farmer.update_order_tracking_state(&order.id, OrderState::DELIVERED, &mut manufacturer);
    let mut order = Order::new(ItemType::RICE, 10000, WeightUnit::PACKET, now(), OrderType::INCOMING, OrderState::PENDING,otcn.clone());
    distributor.place_order(&mut order, &mut manufacturer);
    manufacturer.update_order_tracking_state(&order.id, OrderState::PREPARING, &mut distributor);
    manufacturer.update_order_tracking_state(&order.id, OrderState::SHIPPING, &mut distributor);
    manufacturer.update_order_tracking_state(&order.id, OrderState::DELIVERED, &mut distributor);
    let mut order = Order::new(ItemType::RICE, 5000, WeightUnit::PACKET, now(), OrderType::INCOMING, OrderState::PENDING,otcn.clone());
    retailer.place_order(&mut order, &mut distributor);
    distributor.update_order_tracking_state(&order.id, OrderState::PREPARING, &mut retailer);
    distributor.update_order_tracking_state(&order.id, OrderState::SHIPPING, &mut retailer);
    distributor.update_order_tracking_state(&order.id, OrderState::DELIVERED, &mut retailer);
    let mut order: Order = Order::new(ItemType::RICE, 1, WeightUnit::PACKET, now(), OrderType::INCOMING, OrderState::PENDING,otcn.clone());
    consumer.place_order(&mut order, &mut retailer);
    consumer.check_product_supplychain(otcn);    
}
