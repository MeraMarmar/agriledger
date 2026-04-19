use serde::Serialize;

#[derive(Debug, Clone,Serialize)]
pub enum App{
    SUPPLYCHAIN
}
pub enum Role{
    FARMER,
    MANUFACTURER,
    DISTRIBUTOR,
    RETAILER,
    CONSUMER
}
#[derive(Debug,Copy,Clone,Serialize)]
pub enum WeightUnit{
    KG,
    TON,
    PACKET
}
#[derive(Debug,Copy,Clone,Serialize)]
pub enum OrderType{
    INCOMING,
    OUTGOING
}
#[derive(Debug,Copy,Clone,Serialize)]
pub enum OrderState{
    PENDING,
    PREPARING,
    SHIPPING,
    DELIVERED,
    CANCELLED,
    REJECTED,    
}
#[derive(Debug,Clone,PartialEq,Serialize)]

pub enum ItemType {
    RICE,
    WHEAT,
}