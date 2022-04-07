# Terra Deposit Starter Pack

This is a simple template to send `UST` to another address via Smart Contract

You can Interact with this Smart Contract using this **TESTNET** address:
```
terra1en4wnedwtq335gxx74u78net75ypew5mx7uggn
```
## ExecuteMsg

### Send
```
{"send": {"to_address": "receiver"}}
```

## Query

### Get Transaction

You'll need to know the id, try to figure out how to get that id alone :)

```
{"get_transaction":{"address":"sender_address", "id":"id"}}
```

