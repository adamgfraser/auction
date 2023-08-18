use std::time::{Duration, SystemTime};

use crate::model::*;

pub fn intitialize(state: &mut State, auction: Auction) {
    state.auction = Some(auction);
}

pub fn bid(state: &mut State, bidder_id: BidderId, price: f32) -> BidResult {
    let winning_bid = state.winning_bid.clone();

    if winning_bid.is_none() {
        state.winning_bid = Some((bidder_id, price));
        return BidResult::Success;
    }

    let (_, winning_price) = winning_bid.unwrap();
    if price > winning_price {
        state.winning_bid = Some((bidder_id, price));
        return BidResult::Success;
    }

    BidResult::Failure("Bid too low".to_string())
}

pub fn close_auction(state: &mut State) -> Option<BidderId> {
    let winning_bid = state.winning_bid.clone();
    if winning_bid.is_none() {
        return None;
    }

    let (bidder_id, _) = winning_bid.unwrap().clone();
    Some(bidder_id)
}

fn now() -> Duration {
    SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
}
