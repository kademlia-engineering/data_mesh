To run the web server,

Please create a `./config.json` (example below):

```JSON
{
    "blockchain_id": "0x...",
    "port": 8080
}
```

To create a new blockchain, in the `config.json`, just leave the `blockchain_id` with empty quotes,

```JSON
{
    "blockchain_id": "",
    "port": 8080
}
```

Next, execute the following commands:

`cd main`

`cargo run`