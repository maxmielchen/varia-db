```text
 _    __           _       ____  ____ 
| |  / /___ ______(_)___ _/ __ \/ __ )
| | / / __ `/ ___/ / __ `/ / / / __  |
| |/ / /_/ / /  / / /_/ / /_/ / /_/ / 
|___/\__,_/_/  /_/\__,_/_____/_____/  
```
VariaDB | Key-Value Storage


## What is VariaDB?

VariaDB is a fast, lightweight key-value storage system, programmed in Rust, known for its ease of use. In typical scenarios, a client – like a browser or an application – interacts directly with VariaDB. The client sends requests to VariaDB, and VariaDB processes these requests and provides the relevant responses.

A key feature of VariaDB is that it is not designed to be used behind a server application in the backend. Instead, VariaDB is intended to be used directly in the frontend. This allows applications to communicate directly with the database, eliminating the need for a separate backend server as an intermediary.


## How to use VariaDB?

VariaDB is shipped as a Docker image. You can pull the image from the GitHub Container Registry. The image is executable on linux/amd64 and linux/arm64.

#### Quick Start

```bash
docker pull ghcr.io/maxmielchen/varia-db:latest
docker run -p 8654:8654 ghcr.io/maxmielchen/varia-db:latest
```

#### Environment Variables

| Variable | Default | Description |
| --- | --- | --- |
| `LOG_LEVEL` | `info` | The log level to use |
| `DATA_DIR` | `/data/varia.bin` | The file to store the data in |
| `PORT` | `8654` | The port to listen on |
| `CACHE_SIZE` | `4096` | The size in mb of the cache |
| `CACHE_TTL` | `3600` | The time in seconds to keep items in the cache |
| `CACHE_TTI` | `600` | The time in seconds to keep items in the cache if they are not accessed |
| `CORS_ALLOW_ORIGIN` | `*` | The origin to allow CORS requests from |

## Protocol

VariaDB can store key-value pairs. The key is a string, and the value is a typed value. 
A value can be a string, a number, a boolean, or a list of values, or a map of values. In the some cases, the value is packet in a Respond object.
[OpenAPI Documentation](openapi.yaml)

#### Values Examples

```json
{
    "Text": "Hello, world!"
}
```

```json
{
    "Array": [
        {"Text": "Hello, world!"},
        {"Number": 42},
        {"Boolean": true}
    ]
}
```

```json
{
    "Map": [
        ["key2", {"Text": "Hello, world!"}],
        ["key1", {"Number": 42}],
        ["key3", {"Boolean": true}]
    ]
}
```

#### Respond Examples

```json
{
  "Value": {"Text": "Hello, world!"}
}
```

```json
{
  "Array": [ "key1", "key2", "key3" ]
}
```

#### Operations

| Operation | HTTP | Type | Respond | Description |
| --- | --- | --- | --- | --- |
| `PUT` | `PUT /put/{key}` | `Respond::Value` | `{ "Value": null }` | Stores a value under a key and returns the old value. |
| `GET` | `GET /get/{key}` | `Respond::Value` | `{ "Value": { "Text": "Hello, world!" } }` | Returns the value stored under a key. |
| `DEL` | `DELETE /del/{key}` | `Respond::Value` | `{ "Value": { "Text": "Hello, world!" } }` | Deletes the value stored under a key and returns the old value. |
| `LIST` | `GET /list` | `Respond::Array` | `{ "Array": [ "key1", "key2", "key3" ] }` | Returns a list of all keys.


#### cURL Examples

Put:
```curl
curl -X 'PUT' \
  'http://localhost:8654/put/hello' \
  -H 'accept: application/json' \
  -H 'Content-Type: application/json' \
  -d '{"Text": "world"}'
```

Get:
```curl
curl -X 'GET' \
  'http://localhost:8654/get/hello' \
  -H 'accept: application/json'
```

Delete:
```curl
curl -X 'DELETE' \
  'http://localhost:8654/del/hello' \
  -H 'accept: application/json'
```

List:
```curl
curl -X 'GET' \
  'http://localhost:8654/list' \
  -H 'accept: application/json'
```