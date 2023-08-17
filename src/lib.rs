mod auction_logic;
mod model;

use bindings::*;
use exports::golem::component::api::{
    Api, BidResult as WitBidResult, BidderId as WitBidderId, Deadline as WitDeadline,
    Item as WitItem, ItemId as WitItemId,
};
use once_cell::sync::Lazy;

use model::*;

struct AuctionImpl;

struct WitState {
    state: Lazy<State>,
}

static mut STATE: WitState = WitState {
    state: Lazy::new(|| State::new()),
};

fn with_state<T>(f: impl FnOnce(&mut State) -> T) -> T {
    let result = unsafe { f(&mut STATE.state) };

    return result;
}

// Here, we declare a Rust implementation of the `Auction` trait.
impl Api for AuctionImpl {
    fn register_bidder(name: String, address: String) -> WitBidderId {
        with_state(|state| auction_logic::register_bidder(state, name, address).into())
    }

    fn list_item(
        name: String,
        description: String,
        limit_price: f32,
        expiration: WitDeadline,
    ) -> WitItemId {
        with_state(|state| {
            auction_logic::list_item(state, name, description, limit_price, expiration.into())
                .into()
        })
    }

    fn get_available_items() -> Vec<WitItem> {
        with_state(|state| {
            auction_logic::get_available_items(state)
                .into_iter()
                .map(|item| item.into())
                .collect()
        })
    }

    fn bid(bidder_id: WitBidderId, item_id: WitItemId, price: f32) -> WitBidResult {
        with_state(|state| {
            auction_logic::bid(state, bidder_id.into(), item_id.into(), price).into()
        })
    }

    fn close_auction(item_id: WitItemId) -> Option<WitBidderId> {
        with_state(|state| {
            auction_logic::close_auction(state, item_id.into()).map(|bidder_id| bidder_id.into())
        })
    }
}

bindings::export!(AuctionImpl);
