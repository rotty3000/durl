# durl

`durl` is a simple command-line tool for parsing and formatting URLs, implemented in Rust, inspired by GNU Coreutils `date`.

## Usage

```bash
durl is a simple command-line tool for parsing and formatting URLs

Usage: durl +<format> <url>

Arguments:
  <FORMAT>  Format string starting with + (e.g., +%S%H%p)
  <URL>     The URL to parse

Options:
  -h, --help     Print help (this message)
  -V, --version  Print version

Format masks:

  e.g. given: https://foo:bar@www.example.com:8443/path/to/file.txt?query=value#section1

  %s  scheme                  (gives: https)
  %S  scheme with delimiter   (gives: https://)
  %a  auth                    (gives: foo:bar)
  %A  auth with delimiter     (gives: foo:bar@)
  %u  username                (gives: foo)
  %U  password                (gives: bar)
  %H  host                    (gives: www.example.com:8443)
  %D  domain                  (gives: www.example.com)
  %d  subdomain               (gives: www)
  %P  port                    (gives: 8443)
  %p  path                    (gives: /path/to/file.txt)
  %b  base                    (gives: file.txt)
  %q  query                   (gives: query=value)
  %Q  query with delimiter    (gives: ?query=value)
  %f  fragment                (gives: section1)
  %F  fragment with delimiter (gives: #section1)
```

## Installation

```bash
# via Homebrew
brew install rotty3000/durl/durl

# from source
cargo install --path .
```

## Building

Install `cross` to assist with cross compilation:

```bash
cargo install cross --git https://github.com/cross-rs/cross
```

To build static binaries for all supported platforms (Linux on musl amd64/arm64 and Windows on amd64):

```bash
make build
```

To compress the produced binaries using UPX:

```bash
make compress
```

**Note:**
- Cross-compilation uses `cross`, which requires Docker/Podman.
- **Clean builds:** It is highly recommended to run `make clean` before switching between native `cargo` builds and `cross` builds to avoid artifact conflicts.

To run the test suite:

```bash
make test
```

The binaries will be located in the `dist/` directory.

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
