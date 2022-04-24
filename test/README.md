# Integration Testing

This python script can be used to Compile, Upload, Initialize and Test your Smart Contract.

## Usage

Create all the files, see below section, then cd into your sc directory and run:

```
python3 path/test.py <files_directory> <wasmfile_name> <option>
```

### Examples

If you want to compile, upload, instantiate and test your contract, from `pluton-protocol-core/contracts/deposits` run:

```
python3 ../../test.py deposits pluton.wasm all
```

Or, if you just want to test your Smart Contract:

```
python3 ../../test.py deposits pluton.wasm terra1wpyx7crglfu2w886y7wf9gkk3ak6l5layqf6zh
```

## Create files:

* Insert at least one seed phrase in `mnemonics.txt`
* Insert your query in `query.txt`
* Insert your executemsg in `execute.txt`
* Insert your initialize message in `initialize.txt`
* Create a `build.sh` script for building your contract and place it in your code directory

### Execute Message File:

This is the syntax for `execute.txt`:
```
[
    {
        "msg": {"<YourMsg>": {"<parameter_1>":"<input_1">, ... , "<parameter_n>":"<input_n"> }},
        "coin": {"<denom>": <amount>},
        "wallet": <index>
    },

     {
        "msg": {"<YourMsg_2>": {"<parameter_1":"input_1">, ... , "<parameter_n":"input_n"> }},
        "coin": {"<denom_2>": <amount_2>},
        "wallet": <index_2>
    }

```
The message in `msg` will be executed with the `index` wallet that is created with the `index-th` seed that you inserted in `mnemonics.txt`.
Here an [example](https://github.com/0x7183/pluton-protocol-core/blob/main/test/deposits/execute.txt)

### Query Message File

This is the syntax for `query.txt`:
```
[
    {"query1":{"<parameter1>": "<value1>"}},
    {"query2":{"<parameter1>": "<value1>"}},

]
```
Here an [example](https://github.com/0x7183/pluton-protocol-core/blob/main/test/deposits/query.txt)

### Initialize Message File
```
[
    
    {"<parameter_1>":"<input_1>", ... , "<parameter_n>":"<input_n>"},
        
    
]
```
Here an [example](https://github.com/0x7183/pluton-protocol-core/blob/main/test/deposits/initialize.txt)







