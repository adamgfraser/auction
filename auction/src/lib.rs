mod auction_logic;
mod model;

use bindings::*;
use exports::golem::component::api::{
    Api, Auction as WitAuction, BidResult as WitBidResult, BidderId as WitBidderId,
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
    fn initialize(auction: WitAuction) {
        with_state(|state| auction_logic::intitialize(state, auction.into()))
    }

    fn bid(bidder_id: WitBidderId, price: f32) -> WitBidResult {
        with_state(|state| auction_logic::bid(state, bidder_id.into(), price).into())
    }

    fn close_auction() -> Option<WitBidderId> {
        with_state(|state| auction_logic::close_auction(state).map(|bidder_id| bidder_id.into()))
    }
}

bindings::export!(AuctionImpl);
