# Golem CLI Instructions

Get your token:

```bash
golem-cli token list
```

Add the auction template:

```bash
golem-cli template add --template-name auction-1 auction.wasm
```

Add the auction registry template:

```bash
golem-cli template add --template-name auction-registry-1 auction-registry.wasm
```

Create a bidder:

```bash
golem-cli worker invoke-and-await --template-name=auction-registry-1 --worker-name=auction-registry-1 --function=golem:component/api/create-bidder --parameters='["Adam", "123 green street"]'
```

b768a93e-878f-40f8-986f-bb5cf94ca879

Create an auction:

```bash
golem-cli worker invoke-and-await --template-name=auction-registry-1 --worker-name=auction-registry-1 --function=golem:component/api/create-auction --parameters='["My first auction", "A simple auction", 100, 0]'
```

Bid:

```bash
golem-cli worker invoke-and-await --template-name=auction-new --worker-name=67071013-6622-422e-a634-32a11d7b72c3 --function=golem:component/api/bid --parameters='[{ "bidder-id": "ceccb680-8d71-4e1f-97c5-52008f7a4e30" }, 200]'
```

Here are the exports from each of our template for reference:

exports:
  - 'golem:component/api/initialize(auction: {auction-id: {auction-id: str}, name:
  str, description: str, limit-price: f32, expiration: u64}) => '
  - 'golem:component/api/bid(bidder-id: {bidder-id: str}, price: f32) => variant(success:
  (), failure: str)'
  - 'golem:component/api/close-auction() => {bidder-id: str}?'

exports:
  - 'golem:component/api/create-bidder(name: str, address: str) => {bidder-id: str}'
  - 'golem:component/api/create-auction(name: str, description: str, limit-price:
  f32, expiration: u64) => {auction-id: str}'
  - 'golem:component/api/get-auctions() => [{auction-id: {auction-id: str}, name:
  str, description: str, limit-price: f32, expiration: u64}]'