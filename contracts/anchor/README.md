# Anchor Deposit Starter Pack

This is a simple template to manage `UST` deposit into Anchor Protocol.

This template is a little bit harder then the other:
- The code in `querier` interact with Anchor Protocol
- The code in `handler` handle all the interaction with our Smart Contract

This code is a simplified version of [Pluton Protocol](https://github.com/0x7183/pluton-protocol-core).

You can Interact with this Smart Contract using this **TESTNET** address:

```
terra1uj0vx4nv2gjwh8rqhse8z86l06v986e9qr64cl
```

Or using the integration testing:

```
git clone https://github.com/0x7183/common-cw-examples
```
```
cd common-cw-examples/contracts/anchor
```
```
python3 ../../test/test.py anchor anchor.wasm all
```

## Init Message

```
{
    "contract":"terra1ez46kxtulsdv07538fh5ra5xj8l68mu8eg24vr",
    "moneymarket":"terra15dwd5mj8v59wpj0wvt233mf5efdff808c5tkal", 
    "aterra_address":"terra1ajt556dpzvjwl0kl5tzku3fc3p3knkg9mkv8jl", 
    "stable_denom":"uusd"
}
```
## ExecuteMsg

### Deposit Message
```
{"deposit":{}}
```
### Withdraw Message
```
{"withdrawal":{"id": "input_id"}}
```

## Query
```
{"depositor_balance":{"address": "input_address"}}
```
