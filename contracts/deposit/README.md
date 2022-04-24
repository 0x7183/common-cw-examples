# Terra Deposit Starter Pack

This is a simple template to manage `UST` deposit into a Smart Contract

You can Interact with this Smart Contract using this **TESTNET** address:
```
terra1yqngjhzda6gclquuwhacedf4h2mts8ztd6xc2z
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
