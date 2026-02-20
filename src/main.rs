use anyhow::{anyhow, Context, Result};
use clap::Parser;
use std::path::Path;
use url::Url;

#[derive(Parser)]
#[command(
    author,
    version,
    about = "durl is a simple command-line tool for parsing and formatting URLs",
    override_usage = "durl +<format> <url>",
    arg_required_else_help = true,
    after_help = "Format masks:\n\n  e.g. given: https://foo:bar@www.example.com:8443/path/to/file.txt?query=value#section1\n\n  %s  scheme                  (gives: https)\n  %S  scheme with delimiter   (gives: https://)\n  %a  auth                    (gives: foo:bar)\n  %A  auth with delimiter     (gives: foo:bar@)\n  %u  username                (gives: foo)\n  %U  password                (gives: bar)\n  %H  host                    (gives: www.example.com:8443)\n  %D  domain                  (gives: www.example.com)\n  %d  subdomain               (gives: www)\n  %P  port                    (gives: 8443)\n  %p  path                    (gives: /path/to/file.txt)\n  %b  base                    (gives: file.txt)\n  %q  query                   (gives: query=value)\n  %Q  query with delimiter    (gives: ?query=value)\n  %f  fragment                (gives: section1)\n  %F  fragment with delimiter (gives: #section1)\n"
)]
struct Args {
    #[arg(help = "Format string starting with + (e.g., +%S%H%p)")]
    format: String,

    #[arg(help = "The URL to parse")]
    url: String,
}

fn main() -> Result<()> {
    let args = Args::parse();

    if !args.format.starts_with('+') {
        return Err(anyhow!("error: format must start with +"));
    }
    let format = &args.format[1..];

    let url_to_parse = if args.url.starts_with("//") {
        format!("none:{}", args.url)
    } else {
        args.url.clone()
    };

    let parsed_url = Url::parse(&url_to_parse)
        .with_context(|| format!("Failed to parse URL: {}", args.url))?;

    println!("{}", parse_format(format, &parsed_url));

    Ok(())
}

fn parse_format(format: &str, u: &Url) -> String {
    let mut output = String::new();
    let mut chars = format.chars().peekable();

    while let Some(c) = chars.next() {
        if c == '%' {
            if let Some(spec) = chars.next() {
                match spec {
                    'A' => {
                        let username = u.username();
                        let password = u.password();
                        if !username.is_empty() || password.is_some() {
                            output.push_str(username);
                            if let Some(p) = password {
                                output.push(':');
                                output.push_str(p);
                            }
                            output.push('@');
                        }
                    }
                    'D' => {
                        if let Some(host) = u.host_str() {
                            output.push_str(host);
                        }
                    }
                    'F' => {
                        if let Some(fragment) = u.fragment() {
                            output.push('#');
                            output.push_str(fragment);
                        }
                    }
                    'H' => {
                        if let Some(host) = u.host_str() {
                            output.push_str(host);
                            if let Some(port) = u.port() {
                                output.push(':');
                                output.push_str(&port.to_string());
                            }
                        }
                    }
                    'P' => {
                        if let Some(port) = u.port() {
                            output.push_str(&port.to_string());
                        }
                    }
                    'Q' => {
                        if let Some(query) = u.query() {
                            output.push('?');
                            output.push_str(query);
                        }
                    }
                    'S' => {
                        let scheme = u.scheme();
                        if !scheme.is_empty() && scheme != "none" {
                            output.push_str(scheme);
                            output.push_str("://");
                        }
                    }
                    'U' => {
                        if let Some(password) = u.password() {
                            output.push_str(password);
                        }
                    }
                    'a' => {
                        let username = u.username();
                        let password = u.password();
                        if !username.is_empty() || password.is_some() {
                            output.push_str(username);
                            if let Some(p) = password {
                                output.push(':');
                                output.push_str(p);
                            }
                        }
                    }
                    'b' => {
                        if let Some(segments) = u.path_segments() {
                            if let Some(last) = segments.last() {
                                if !last.is_empty() {
                                    output.push_str(last);
                                } else {
                                    let path = u.path();
                                    let base = Path::new(path)
                                        .file_name()
                                        .and_then(|s| s.to_str())
                                        .unwrap_or("");
                                    output.push_str(base);
                                }
                            }
                        }
                    }
                    'd' => {
                        if let Some(host) = u.host_str() {
                            let parts: Vec<&str> = host.split('.').collect();
                            if parts.len() > 2 {
                                output.push_str(&parts[..parts.len() - 2].join("."));
                            }
                        }
                    }
                    'f' => {
                        if let Some(fragment) = u.fragment() {
                            output.push_str(fragment);
                        }
                    }
                    'p' => {
                        output.push_str(u.path());
                    }
                    'q' => {
                        if let Some(query) = u.query() {
                            output.push_str(query);
                        }
                    }
                    's' => {
                        let scheme = u.scheme();
                        if scheme != "none" {
                            output.push_str(scheme);
                        }
                    }
                    'u' => {
                        output.push_str(u.username());
                    }
                    _ => {
                        output.push('%');
                        output.push(spec);
                    }
                }
            } else {
                output.push('%');
            }
        } else {
            output.push(c);
        }
    }
    output
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_format() {
        struct TestCase {
            name: &'static str,
            format: &'static str,
            url: &'static str,
            want: &'static str,
        }

        let cases = vec![
            TestCase {
                name: "add username and password",
                format: "%Susername:password@%H%p%Q%F",
                url: "https://example.com:8080/index.html?a=1&b=2#section1",
                want: "https://username:password@example.com:8080/index.html?a=1&b=2#section1",
            },
            TestCase {
                name: "all",
                format: "%S%A%H%p%Q%F",
                url: "https://user:pass@example.com:8080/index.html?a=1&b=2#section1",
                want: "https://user:pass@example.com:8080/index.html?a=1&b=2#section1",
            },
            TestCase {
                name: "all parts",
                format: "%s|%S|%a|%A|%u|%U|%H|%D|%d|%P|%p|%q|%Q|%f|%F",
                url: "https://user:pass@www.example.com:8080/index.html?a=1&b=2#section1",
                want: "https|https://|user:pass|user:pass@|user|pass|www.example.com:8080|www.example.com|www|8080|/index.html|a=1&b=2|?a=1&b=2|section1|#section1",
            },
            TestCase {
                name: "auth",
                format: "%a",
                url: "https://user:pass@example.com:8080/index.html?a=1&b=2#section1",
                want: "user:pass",
            },
            TestCase {
                name: "bare url with just domain",
                format: "%D",
                url: "//example.com",
                want: "example.com",
            },
            TestCase {
                name: "base path",
                format: "%b",
                url: "https://example.com/foo/bar/index.html",
                want: "index.html",
            },
            TestCase {
                name: "base path with trailing slash",
                format: "%b",
                url: "https://example.com/foo/bar/",
                want: "bar",
            },
            TestCase {
                name: "domain",
                format: "%D",
                url: "https://user:pass@www.example.com:8080/index.html?a=1&b=2#section1",
                want: "www.example.com",
            },
            TestCase {
                name: "fragment",
                format: "%f",
                url: "https://user:pass@example.com:8080/index.html?a=1&b=2#section1",
                want: "section1",
            },
            TestCase {
                name: "fragment with hash mark",
                format: "%F",
                url: "https://user:pass@example.com:8080/index.html?a=1&b=2#section1",
                want: "#section1",
            },
            TestCase {
                name: "host",
                format: "%H",
                url: "https://user:pass@example.com:8080/index.html?a=1&b=2#section1",
                want: "example.com:8080",
            },
            TestCase {
                name: "password",
                format: "%U",
                url: "https://user:pass@example.com:8080/index.html?a=1&b=2#section1",
                want: "pass",
            },
            TestCase {
                name: "path",
                format: "%p",
                url: "https://user:pass@example.com:8080/index.html?a=1&b=2#section1",
                want: "/index.html",
            },
            TestCase {
                name: "port",
                format: "%P",
                url: "https://user:pass@example.com:8080/index.html?a=1&b=2#section1",
                want: "8080",
            },
            TestCase {
                name: "query",
                format: "%q",
                url: "https://user:pass@example.com:8080/index.html?a=1&b=2#section1",
                want: "a=1&b=2",
            },
            TestCase {
                name: "query with question mark",
                format: "%Q",
                url: "https://user:pass@example.com:8080/index.html?a=1&b=2#section1",
                want: "?a=1&b=2",
            },
            TestCase {
                name: "scheme",
                format: "%s",
                url: "https://user:pass@example.com:8080/index.html?a=1&b=2#section1",
                want: "https",
            },
            TestCase {
                name: "separator",
                format: "%S",
                url: "https://user:pass@example.com:8080/index.html?a=1&b=2#section1",
                want: "https://",
            },
            TestCase {
                name: "subdomain",
                format: "%d",
                url: "https://user:pass@www.example.com:8080/index.html?a=1&b=2#section1",
                want: "www",
            },
            TestCase {
                name: "user",
                format: "%u",
                url: "https://user:pass@example.com:8080/index.html?a=1&b=2#section1",
                want: "user",
            },
            TestCase {
                name: "with different scheme",
                format: "http://%H%p%F",
                url: "ftp://user:pass@example.com/index.html#section1",
                want: "http://example.com/index.html#section1",
            },
            TestCase {
                name: "with literals",
                format: "%S%H/foo/bar%p%F",
                url: "https://user:pass@example.com/index.html#section1",
                want: "https://example.com/foo/bar/index.html#section1",
            },
            TestCase {
                name: "without auth",
                format: "%S%H%p%Q%F",
                url: "https://example.com:8080/index.html?a=1&b=2#section1",
                want: "https://example.com:8080/index.html?a=1&b=2#section1",
            },
            TestCase {
                name: "without port",
                format: "%S%H%p%Q%F",
                url: "https://example.com/index.html?a=1&b=2#section1",
                want: "https://example.com/index.html?a=1&b=2#section1",
            },
            TestCase {
                name: "without query",
                format: "%S%H%p%F",
                url: "https://example.com/index.html#section1",
                want: "https://example.com/index.html#section1",
            },
        ];

        for case in cases {
            let url_to_parse = if case.url.starts_with("//") {
                format!("none:{}", case.url)
            } else {
                case.url.to_string()
            };
            let u = Url::parse(&url_to_parse).expect(&format!("failed to parse url in test: {}", case.name));
            let got = parse_format(case.format, &u);
            assert_eq!(got, case.want, "failed test: {}", case.name);
        }
    }
}
