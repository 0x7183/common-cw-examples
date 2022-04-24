# Terra Deposit Starter Pack

This is a simple template to manage `UST` deposit into a Smart Contract

You can Interact with this Smart Contract using this **TESTNET** address:
```
terra1yqngjhzda6gclquuwhacedf4h2mts8ztd6xc2z
```
Or using the integration testing:

```
git clone https://github.com/0x7183/common-cw-examples
```
```
cd common-cw-examples/contracts/deposit
```
```
python3 ../../test/test.py deposit deposit.wasm all
```
## ExecuteMsg

### Deposit
```
{"deposit":{}}
```
### Withdraw all

```
{"withdrawal":{"id":"deposit_id"}}
```

## Query

### Get Deposits

```
{"get_deposits":{"address":"depositor_address"}}
```
