mod auction;
mod auction_registry_logic;
mod model;

cargo_component_bindings::generate!();

use bindings::*;
use exports::golem::component::api::{
    Guest, Auction as WitAuction, AuctionId as WitAuctionId, BidderId as WitBidderId,
    Deadline as WitDeadline,
};
use once_cell::sync::Lazy;

use model::*;

struct Component;

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
impl Guest for Component {
    fn create_bidder(name: String, address: String) -> WitBidderId {
        with_state(|state| auction_registry_logic::create_bidder(state, name, address).into())
    }

    fn create_auction(
        name: String,
        description: String,
        limit_price: f32,
        expiration: WitDeadline,
    ) -> WitAuctionId {
        with_state(|state| {
            auction_registry_logic::create_auction(
                state,
                name,
                description,
                limit_price,
                expiration.into(),
            )
            .into()
        })
    }

    fn get_auctions() -> Vec<WitAuction> {
        with_state(|state| {
            auction_registry_logic::get_auctions(state)
                .into_iter()
                .map(|item| item.into())
                .collect()
        })
    }
}
