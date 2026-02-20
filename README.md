# durl

`durl` is a simple command-line tool for parsing and formatting URLs, implemented in Rust as a drop-in replacement for `gurl`.

## Installation

```bash
cargo install --path .
```

## Usage

Usage: `durl +<format> <url>`

Example:

```bash
durl +%S%H%p https://google.com/search?q=rust
```

### Format Specifiers

| Specifier | Description | Example Output |
| :--- | :--- | :--- |
| `%s` | scheme | `https` |
| `%S` | scheme with delimiter | `https://` |
| `%a` | auth | `foo:bar` |
| `%A` | auth with delimiter | `foo:bar@` |
| `%u` | username | `foo` |
| `%U` | password | `bar` |
| `%H` | host (with port) | `www.example.com:8443` |
| `%D` | domain (hostname) | `www.example.com` |
| `%d` | subdomain | `www` |
| `%P` | port | `8443` |
| `%p` | path | `/path/to/file.txt` |
| `%b` | base (filename) | `file.txt` |
| `%q` | query | `query=value` |
| `%Q` | query with delimiter | `?query=value` |
| `%f` | fragment | `section1` |
| `%F` | fragment with delimiter | `#section1` |
