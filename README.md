This quickstart guide assumes you have rust installed on your system.

To run the web server,

Please create a `./config.json` (example below):

```JSON
{
    "node_id": "0x987...",
    "mesh_id": "0x123...",
    "port": 8080,
    "external_address": "127.0.0.1:8081",
    "db_connection":"<IMMU_DB>"
}
```

To create a new data mesh, in the `config.json`, just leave the `mesh_id` with empty quotes. The same principal can be applied to new nodes in the network with the `node_id` field.

```JSON
{
    "node_id": "",
    "mesh_id": "",
    "port": 8080,
    "external_address": "127.0.0.1:8081",
    "db_connection":"<IMMU_DB>"
}
```

Next, to run the node, execute the following commands:

`cd main`

`cargo run`