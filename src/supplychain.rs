use super::*;
use serde::Serialize;
use uuid::Uuid;
use std::sync::Mutex;
use lazy_static::lazy_static;
lazy_static!{
    pub static ref LEDGER: Mutex<Blockchain> = Mutex::new(Blockchain::new());
    pub static ref TXS_POOL: Mutex<Vec<Transaction>> = Mutex::new(vec![]);
}
fn make_tx(transaction:Transaction){
    // let txs = TXS_POOL.lock().unwrap().clone();
    let mut txs_pool_changer = TXS_POOL.lock().unwrap();
    let mut ledger_changer = LEDGER.lock().unwrap();
    {
    {
        txs_pool_changer.push(transaction.clone());
    }
    
    let block_no_of_tx:usize = 3;
    if ledger_changer.blocks.len()==0 {
        ledger_changer.initialize_ledger();
    }
    
        if txs_pool_changer.len() >= block_no_of_tx {
             let previous_block: Block = ledger_changer.blocks.last().unwrap().clone();
             let mut block: Block = Block::new(previous_block.index+1, now(), previous_block.hash(), txs_pool_changer.clone(), previous_block.difficulty);
             block.mine();
             ledger_changer.update_with_block(block.clone()).expect("Failed");
             println!("Mined block {:?}", &block); 
             txs_pool_changer.clear();
        }
    }
}
#[derive(Debug, Clone,Serialize)]
pub struct Order {
    pub id: Uuid, 
    pub order_item: ItemType,
    pub qty: u32,
    pub qty_unit: WeightUnit,
    pub date: u128,
    pub order_direction: OrderType,
    pub order_state: OrderState,
    order_tracking_number: String,
}
impl Order {
    pub fn new(
        order_item: ItemType,
        qty: u32,
        qty_unit: WeightUnit,
        date: u128,
        order_direction: OrderType,
        order_state: OrderState,
        order_tracking_number: String
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            order_item,
            qty,
            qty_unit,
            date,
            order_direction,
            order_state,
            order_tracking_number
        }
    }
    pub fn change_order_state(&mut self, state: OrderState) {
        self.order_state = state;
    }
}
#[derive(Debug,Clone)]
pub struct Crop {
    pub id: Uuid,
    pub crop_type: ItemType,
    pub weight: u32,
    pub weight_unit: WeightUnit,
    pub cropping_date: u128,
    pub order_tracking_number: String,
}
impl Crop {
    pub fn new(
        crop_type: ItemType,
        weight: u32,
        weight_unit: WeightUnit,
        cropping_date: u128,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            crop_type,
            weight,
            weight_unit,
            cropping_date,
            order_tracking_number: Uuid::new_v4().to_string()[..8].to_owned(),
        }
    }
}
pub struct Farmer {
    pub id: Uuid,
    pub wallet: Wallet,
    pub crops: Vec<Crop>,
    pub orders: Vec<Order>,
    pub organization_name: String,
}
impl Farmer {
    pub fn new(
        wallet: Wallet,
        crops: Vec<Crop>,
        orders: Vec<Order>,
        organization_name: String,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            wallet,
            crops,
            orders,
            organization_name,
        }
    }
    pub fn cropped_field(&mut self, crop: Crop) -> String {
        self.crops.push(crop.clone());
        crop.order_tracking_number.clone()
    }
    pub fn update_order_tracking_state(
        &mut self,
        order_id: &Uuid,
        order_state: OrderState,
        manufacturer: &mut Manufacturer,
    ) {
        for index in 0..self.orders.len() {
            if self.orders[index].id == *order_id {
                self.orders[index].change_order_state(order_state);
            }
        }
        for index in 0..manufacturer.orders.len(){
            if manufacturer.orders[index].id == *order_id{
                 manufacturer.orders[index].change_order_state(order_state);
                 make_tx(Transaction::new(App::SUPPLYCHAIN, &self.wallet.public_key, &manufacturer.wallet.public_key, manufacturer.orders[index].clone()));
            }
        }
    }
}
pub struct Manufacturer {
    pub id: Uuid,
    pub wallet: Wallet,
    pub orders: Vec<Order>,
    pub organization_name: String,
}
impl Manufacturer {
    pub fn new(wallet: Wallet, orders: Vec<Order>, organization_name: String) -> Self {
        Self {
            id: Uuid::new_v4(),
            wallet,
            orders,
            organization_name,
        }
    }
   
    pub fn place_order(&mut self, order: &mut Order, farmer: &mut Farmer) {
        order.change_order_state(OrderState::PENDING);
        self.orders.push(order.clone());
        farmer.orders.push(order.clone());
        make_tx(Transaction::new(App::SUPPLYCHAIN, &self.wallet.public_key, &farmer.wallet.public_key, order.clone()))
    }

    pub fn update_order_tracking_state(
        &mut self,
        order_id: &Uuid,
        order_state: OrderState,
        distributor: &mut Distributor,
    ) {

        for index in 0..self.orders.len() {
            if self.orders[index].id == *order_id {
                self.orders[index].change_order_state(order_state);
            }
        }
        for index in 0..distributor.orders.len(){
            if distributor.orders[index].id == *order_id{
                 distributor.orders[index].change_order_state(order_state);
                make_tx(Transaction { dapp: App::SUPPLYCHAIN, from: self.wallet.public_key.clone(), to: distributor.wallet.public_key.clone(), order: distributor.orders[index].clone(), time: now() })
            }
        }
    }
}
pub struct Distributor {
    pub id: Uuid,
    pub wallet: Wallet,
    pub orders: Vec<Order>,
    pub organization_name: String,
}
impl Distributor {
    pub fn new(wallet: Wallet, orders: Vec<Order>, organization_name: String) -> Self {
        Self {
            id: Uuid::new_v4(),
            wallet,
            orders,
            organization_name,
        }
    }
    pub fn place_order(&mut self, order: &mut Order, manufacturer: &mut Manufacturer) {
        order.change_order_state(OrderState::PENDING);
        //record a tx that the order is placed
        self.orders.push(order.clone());
        manufacturer.orders.push(order.clone());
        make_tx(Transaction::new(App::SUPPLYCHAIN, &self.wallet.public_key, &manufacturer.wallet.public_key, order.clone()));
    }


    pub fn update_order_tracking_state(
        &mut self,
        order_id: &Uuid,
        order_state: OrderState,
        retailer: &mut Retailer,
    ) {
        for index in 0..self.orders.len() {
            if self.orders[index].id == *order_id {
                self.orders[index].change_order_state(order_state);
            }
        }
        for index in 0..retailer.orders.len(){
            if retailer.orders[index].id == *order_id{
                 retailer.orders[index].change_order_state(order_state);
                make_tx(Transaction { dapp: App::SUPPLYCHAIN, from: self.wallet.public_key.clone(), to: retailer.wallet.public_key.clone(), order: retailer.orders[index].clone(), time: now() })
            }
        }
    }
}
pub struct Retailer {
    pub id: Uuid,
    pub wallet: Wallet,
    pub orders: Vec<Order>,
    pub organization_name: String,
}

impl Retailer {
    pub fn new(wallet: Wallet, orders: Vec<Order>, organization_name: String) -> Self {
        Self {
            id: Uuid::new_v4(),
            wallet,
            orders,
            organization_name,
        }
    }
    pub fn place_order(&mut self, order: &Order, distributor: &mut Distributor) {
        self.orders.push(order.clone());
        distributor.orders.push(order.clone());
        make_tx(Transaction::new(App::SUPPLYCHAIN, &self.wallet.public_key, &distributor.wallet.public_key, order.clone()));

    }
    
    pub fn update_order_tracking_state(
        &mut self,
        order_id: &Uuid,
        order_state: OrderState,
        consumer: &mut Consumer,
    ) {
        for index in 0..self.orders.len() {
            if self.orders[index].id == *order_id {
                self.orders[index].change_order_state(order_state);
            }
        }
        for index in 0..consumer.orders.len(){
            if consumer.orders[index].id == *order_id{
                 consumer.orders[index].change_order_state(order_state);
                make_tx(Transaction { dapp: App::SUPPLYCHAIN, from: self.wallet.public_key.clone(), to: consumer.wallet.public_key.clone(), order: consumer.orders[index].clone(), time: now() })
            }
        }
    }
}
pub struct Consumer {
    pub id: Uuid,
    pub wallet: Wallet,
    pub orders: Vec<Order>,
}
impl Consumer {
    pub fn new(wallet: Wallet, orders: Vec<Order>) -> Self {
        Self {
            id: Uuid::new_v4(),
            wallet,
            orders,
        }
    }
    pub fn place_order(&mut self, order: &Order, retailer: &mut Retailer) {
        self.orders.push(order.clone());
        retailer.orders.push(order.clone());
        make_tx(Transaction::new(App::SUPPLYCHAIN, &self.wallet.public_key, &retailer.wallet.public_key, order.clone()));
    }
    pub fn check_product_supplychain(&self, otn: String) {
        {
            let ledger = LEDGER.lock().unwrap();
            for block in &ledger.blocks {
                for tx in &block.transactions {
                    if tx.order.order_tracking_number == otn {
                        println!(" Transaction : {:#?}", tx);
                    }
                }
            }
        }
        {
            let pool = TXS_POOL.lock().unwrap();
            for tx in pool.iter() {
                if tx.order.order_tracking_number == otn {
                    println!(" (Pending) Transaction : {:#?}", tx);
                }
            }
        }
    }
}
