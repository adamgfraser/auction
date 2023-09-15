use crate::model::*;

pub fn create(auction: Auction) {
    create_instance(auction.auction_id.clone());
    let invocation_key = get_invocation_key(auction.auction_id.clone());
    initialize_instance(auction, invocation_key);
}

fn create_instance(auction_id: AuctionId) {
    let client = reqwest::Client::new();
    let template_id = "auction";
    let worker_id = format!("auction-{}", auction_id.auction_id);
    let url = format!(
        "https://release.api.golem.cloud/v1/templates/{}/workers?worker-name={}",
        template_id, worker_id
    );
    let token = "token";
    client
        .post(url)
        .header("Authorization", token)
        .send()
        .unwrap();
}

fn get_invocation_key(auction_id: AuctionId) -> String {
    let client = reqwest::Client::new();
    let template_id = "auction";
    let worker_id = format!("auction-{}", auction_id.auction_id);
    let url = format!(
        "https://release.api.golem.cloud/v1/templates/{}/workers/{}/key",
        template_id, worker_id
    );
    let token = "token";
    client
        .post(url)
        .header("Authorization", token)
        .send()
        .unwrap()
        .text()
        .unwrap()
}

fn initialize_instance(auction: Auction, invocation_key: String) {
    let client = reqwest::Client::new();
    let template_id = "auction";
    let worker_id = format!("auction-{}", auction.auction_id.auction_id);
    let url = format!("https://release.api.golem.cloud/v1/templates/{}/workers/{}/invoke-and-await?invocation-key={}", template_id, worker_id, invocation_key);
    let token = "token";
    client
        .post(url)
        .json(&auction)
        .header("Authorization", token)
        .send()
        .unwrap();
}

// Create a new instance

// All endpoints (except for /auth ) requires authorization header Authorization: bearer <token-secret>

// POST /templates
// /{template-id}
// /workers?worker-name={worker-name}&args={args}&env={env}

// Get an invocation key

// POST
// /templates
// /{template-id}
// /workers
// /{worker-name}
// / key

// Invoke a function and wait for its result

// POST
// /templates
// /{template-id}
// /workers
// /{worker-name}
// / invoke-and-await?invocation-key={invocation-key}&function={function-name}&calling-convention={calling-convention-type}

// Request body consist of invoke-parameters which is a JSON, and mostly it is a JSON array.

// Example:
// [{”product-id” : “G1000”, "name": "t-shirt"}]
