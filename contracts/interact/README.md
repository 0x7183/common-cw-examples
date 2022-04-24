# Terra Interact Starter Pack

This is a simple template to interact with another Smart Contract.

You can Interact with this Smart Contract using this **TESTNET** address:
```
terra1xuakqsr4eyvgj2fg4v99z8juxpn27kjcfp2kgz
```
## ExecuteMsg

### Interact
```
{"interact": {"to_address": "receiver"}}
```
## Query

### Get Transaction

You'll need to know the id, try to figure out how to get that id alone :)

```
{"get_interaction":{"address":"contract_id", "id":"transaction_id"}}
```

