# httpheader

Fetch and display HTTP response headers.

## Install

```console
cargo build --release
sudo cp target/release/httpheader /usr/local/bin/
```

## Usage

```console
httpheader https://example.com
httpheader http://api.example.com/health
```

Output:

```
HTTP/1.1 200 OK
Content-Type: text/html
Content-Length: 1256
```
