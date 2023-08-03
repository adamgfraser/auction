use bindings::*;
use exports::golem::component::api::*;
use once_cell::sync::Lazy;
use std::collections::HashMap;
use std::time::SystemTime;
use uuid::Uuid;

struct AuctionImpl;

struct State {
    bidders: Lazy<HashMap<Uuid, (String, String)>>,
    items: Lazy<HashMap<Uuid, Item>>,
    winning_bids: Lazy<HashMap<Uuid, (Uuid, f32)>>,
}

static mut STATE: State = State {
    bidders: Lazy::new(|| HashMap::new()),
    items: Lazy::new(|| HashMap::new()),
    winning_bids: Lazy::new(|| HashMap::new()),
};

fn with_state<T>(f: impl FnOnce(&mut State) -> T) -> T {
    let result = unsafe { f(&mut STATE) };

    return result;
}

fn now() -> u64 {
    SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_secs()
}

// Here, we declare a Rust implementation of the `Auction` trait.
impl Api for AuctionImpl {
    fn register_bidder(name: String, address: String) -> BidderId {
        with_state(|state| {
            let bidder_id = Uuid::new_v4();
            state.bidders.insert(bidder_id, (name, address));
            BidderId {
                bidder_id: bidder_id.to_string(),
            }
        })
    }

    fn list_item(
        name: String,
        description: String,
        limit_price: f32,
        expiration: Deadline,
    ) -> ItemId {
        with_state(|state| {
            let item_id = Uuid::new_v4();
            state.items.insert(
                item_id,
                Item {
                    item_id: item_id.to_string(),
                    name,
                    description,
                    limit_price,
                    expiration,
                },
            );
            ItemId {
                item_id: item_id.to_string(),
            }
        })
    }

    fn get_available_items() -> Vec<Item> {
        with_state(|state| {
            state
                .items
                .values()
                .filter(|item| item.expiration > now())
                .cloned()
                .collect()
        })
    }

    fn bid(bidder_id: BidderId, item_id: ItemId, price: f32) -> BidResult {
        with_state(|state| {
            let bidder_id = Uuid::parse_str(&bidder_id.bidder_id).unwrap();
            let item_id = Uuid::parse_str(&item_id.item_id).unwrap();
            let item = state.items.get(&item_id).unwrap();
            let bidder = state.bidders.get(&bidder_id).unwrap();
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
        })
    }

    fn close_auction(item_id: String) -> Option<BidderId> {
        with_state(|state| {
            let item_id = Uuid::parse_str(&item_id).unwrap();
            let winning_bid = state.winning_bids.get(&item_id);
            if winning_bid.is_none() {
                return None;
            }

            let (bidder_id, _) = winning_bid.unwrap();
            let bidder_id = bidder_id.to_string();
            state.winning_bids.remove(&item_id);
            Some(BidderId { bidder_id })
        })
    }
}

bindings::export!(AuctionImpl);
