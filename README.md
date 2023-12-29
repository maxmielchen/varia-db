```text
 _    __           _       ____  ____ 
| |  / /___ ______(_)___ _/ __ \/ __ )
| | / / __ `/ ___/ / __ `/ / / / __  |
| |/ / /_/ / /  / / /_/ / /_/ / /_/ / 
|___/\__,_/_/  /_/\__,_/_____/_____/  
```
VariaDB | Key-Value Storage


## What is VariaDB?

VariaDB is a key-value storage system that is designed to be fast, lightweight, and easy to use. It is written in Rust.

## How to use VariaDB?

Put data into VariaDB:
```json
{
    "Put": [
        "key1",
        {
            "Text": "text"
        }
    ]
}
```
> Note: you can also use Number, Boolean instead of Text.

Get data from VariaDB:
```json
{
    "Get": "key1"
}
```

Delete data from VariaDB:
```json
{
    "Del": "key1"
}
```

List all keys in VariaDB:
```json
{
    "List": null
}
```
