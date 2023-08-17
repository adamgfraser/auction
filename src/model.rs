use std::collections::HashMap;
use std::time::Duration;
use uuid::Uuid;

use crate::exports::golem::component::api::{
    BidResult as WitBidResult, BidderId as WitBidderId, Deadline as WitDeadline, Item as WitItem,
    ItemId as WitItemId,
};

#[derive(Clone, Eq, Hash, PartialEq, PartialOrd, Ord)]
pub struct BidderId {
    pub bidder_id: Uuid,
}

impl BidderId {
    pub fn new() -> Self {
        BidderId {
            bidder_id: Uuid::new_v4(),
        }
    }
}

impl From<WitBidderId> for BidderId {
    fn from(wit_bidder_id: WitBidderId) -> Self {
        BidderId {
            bidder_id: Uuid::parse_str(&wit_bidder_id.bidder_id).unwrap(),
        }
    }
}

impl Into<WitBidderId> for BidderId {
    fn into(self) -> WitBidderId {
        WitBidderId {
            bidder_id: self.bidder_id.to_string(),
        }
    }
}

#[derive(Clone, Eq, Hash, PartialEq, PartialOrd, Ord)]
pub struct ItemId {
    pub item_id: Uuid,
}

impl ItemId {
    pub fn new() -> Self {
        ItemId {
            item_id: Uuid::new_v4(),
        }
    }
}

impl From<WitItemId> for ItemId {
    fn from(wit_item_id: WitItemId) -> Self {
        ItemId {
            item_id: Uuid::parse_str(&wit_item_id.item_id).unwrap(),
        }
    }
}

impl Into<WitItemId> for ItemId {
    fn into(self) -> WitItemId {
        WitItemId {
            item_id: self.item_id.to_string(),
        }
    }
}

pub enum BidResult {
    Success,
    Failure(String),
}

impl From<WitBidResult> for BidResult {
    fn from(wit_bid_result: WitBidResult) -> Self {
        match wit_bid_result {
            WitBidResult::Success => BidResult::Success,
            WitBidResult::Failure(reason) => BidResult::Failure(reason),
        }
    }
}

impl Into<WitBidResult> for BidResult {
    fn into(self) -> WitBidResult {
        match self {
            BidResult::Success => WitBidResult::Success,
            BidResult::Failure(reason) => WitBidResult::Failure(reason),
        }
    }
}

#[derive(Clone)]
pub struct Deadline {
    pub deadline: Duration,
}

impl From<WitDeadline> for Deadline {
    fn from(wit_deadline: WitDeadline) -> Self {
        Deadline {
            deadline: Duration::from_secs(wit_deadline),
        }
    }
}

impl Into<WitDeadline> for Deadline {
    fn into(self) -> WitDeadline {
        self.deadline.as_secs()
    }
}

#[derive(Clone)]
pub struct Item {
    pub item_id: ItemId,
    pub name: String,
    pub description: String,
    pub limit_price: f32,
    pub expiration: Deadline,
}

impl Item {
    pub fn new(
        item_id: ItemId,
        name: String,
        description: String,
        limit_price: f32,
        expiration: Deadline,
    ) -> Self {
        Item {
            item_id,
            name,
            description,
            limit_price,
            expiration,
        }
    }
}

impl From<WitItem> for Item {
    fn from(wit_item: WitItem) -> Self {
        Item {
            item_id: ItemId::from(wit_item.item_id),
            name: wit_item.name,
            description: wit_item.description,
            limit_price: wit_item.limit_price,
            expiration: Deadline::from(wit_item.expiration),
        }
    }
}

impl Into<WitItem> for Item {
    fn into(self) -> WitItem {
        WitItem {
            item_id: self.item_id.into(),
            name: self.name,
            description: self.description,
            limit_price: self.limit_price,
            expiration: self.expiration.into(),
        }
    }
}

pub struct Bidder {
    bidder_id: BidderId,
    name: String,
    address: String,
}

impl Bidder {
    pub fn new(bidder_id: BidderId, name: String, address: String) -> Self {
        Bidder {
            bidder_id,
            name,
            address,
        }
    }
}

pub struct State {
    pub bidders: HashMap<BidderId, Bidder>,
    pub items: HashMap<ItemId, Item>,
    pub winning_bids: HashMap<ItemId, (BidderId, f32)>,
}

impl State {
    pub fn new() -> Self {
        State {
            bidders: HashMap::new(),
            items: HashMap::new(),
            winning_bids: HashMap::new(),
        }
    }
}
