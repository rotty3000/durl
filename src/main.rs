use anyhow::{anyhow, Context, Result};
use clap::Parser;
use std::path::Path;
use url::Url;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
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

    let parsed_url = Url::parse(&args.url)
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
                    'b' => {
                        if let Some(segments) = u.path_segments() {
                            if let Some(last) = segments.last() {
                                if !last.is_empty() {
                                    output.push_str(last);
                                } else {
                                    // If path ends in /, path_segments returns ["path", ""]
                                    // gurl uses path.Base which returns the component before the trailing slash if any
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
                    'D' => {
                        if let Some(host) = u.host_str() {
                            output.push_str(host);
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
                    'F' => {
                        if let Some(fragment) = u.fragment() {
                            output.push('#');
                            output.push_str(fragment);
                        }
                    }
                    'f' => {
                        if let Some(fragment) = u.fragment() {
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
                    'p' => {
                        output.push_str(u.path());
                    }
                    'Q' => {
                        if let Some(query) = u.query() {
                            output.push('?');
                            output.push_str(query);
                        }
                    }
                    'q' => {
                        if let Some(query) = u.query() {
                            output.push_str(query);
                        }
                    }
                    'S' => {
                        let scheme = u.scheme();
                        if !scheme.is_empty() {
                            output.push_str(scheme);
                            output.push_str("://");
                        }
                    }
                    's' => {
                        output.push_str(u.scheme());
                    }
                    'U' => {
                        if let Some(password) = u.password() {
                            output.push_str(password);
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
