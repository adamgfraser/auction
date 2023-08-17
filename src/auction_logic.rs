use std::time::{Duration, SystemTime};

use crate::model::*;

pub fn register_bidder(state: &mut State, name: String, address: String) -> BidderId {
    let bidder_id = BidderId::new();
    let bidder = Bidder::new(bidder_id.clone(), name, address);
    state.bidders.insert(bidder_id.clone(), bidder);
    bidder_id
}

pub fn list_item(
    state: &mut State,
    name: String,
    description: String,
    limit_price: f32,
    expiration: Deadline,
) -> ItemId {
    let item_id = ItemId::new();
    let item = Item::new(item_id.clone(), name, description, limit_price, expiration);
    state.items.insert(item_id.clone(), item);
    item_id
}

pub fn get_available_items(state: &State) -> Vec<Item> {
    state
        .items
        .values()
        .filter(|item| item.expiration.deadline > now())
        .cloned()
        .collect()
}

pub fn bid(state: &mut State, bidder_id: BidderId, item_id: ItemId, price: f32) -> BidResult {
    let winning_bid = state.winning_bids.get(&item_id);

    if winning_bid.is_none() {
        state.winning_bids.insert(item_id, (bidder_id, price));
        return BidResult::Success;
    }

    let (_, winning_price) = winning_bid.unwrap();
    if price > *winning_price {
        state.winning_bids.insert(item_id, (bidder_id, price));
        return BidResult::Success;
    }

    BidResult::Failure("Bid too low".to_string())
}

pub fn close_auction(state: &mut State, item_id: ItemId) -> Option<BidderId> {
    let winning_bid = state.winning_bids.get(&item_id);
    if winning_bid.is_none() {
        return None;
    }

    let (bidder_id, _) = winning_bid.unwrap().clone();
    state.winning_bids.remove(&item_id);
    Some(bidder_id)
}

fn now() -> Duration {
    SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
}
