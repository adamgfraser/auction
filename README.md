# Golem CLI Instructions

Get your token:

```bash
bin/golem token list
```

Add the auction template:

```bash
bin/golem component add --component-name auction-1 auction.wasm
```

Add the auction registry template:

```bash
bin/golem component add --component-name auction-registry-1 auction-registry.wasm
```

Create a bidder:

```bash
bin/golem instance invoke-and-await --component-name=auction-registry-1 --instance-name=auction-registry-1 --function=golem:component/api/create-bidder --parameters='["Adam", "123 green street"]'
```

b768a93e-878f-40f8-986f-bb5cf94ca879

Create an auction:

```bash
bin/golem instance invoke-and-await --component-name=auction-registry-1 --instance-name=auction-registry-1 --function=golem:component/api/create-auction --parameters='["My first auction", "A simple auction", 100, 0]'
```

Bid:

```bash
bin/golem instance invoke-and-await --component-name=auction-1 --instance-name=7d99d8e8-800f-4c40-aa40-946630813dfb --function=golem:component/api/bid --parameters='[{ "bidder-id": "ceccb680-8d71-4e1f-97c5-52008f7a4e30" }, 200]'
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