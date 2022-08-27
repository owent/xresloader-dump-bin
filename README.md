# xresloader-dump-bin

[![Build](https://github.com/xresloader/xresloader-dump-bin/actions/workflows/main.yml/badge.svg)](https://github.com/xresloader/xresloader-dump-bin/actions/workflows/main.yml)
[![GitHub release (latest by date)](https://img.shields.io/github/v/release/xresloader/xresloader-dump-bin)](https://github.com/xresloader/xresloader-dump-bin/releases)

![GitHub code size in bytes](https://img.shields.io/github/languages/code-size/xresloader/xresloader-dump-bin)
![GitHub repo size](https://img.shields.io/github/repo-size/xresloader/xresloader-dump-bin)
![GitHub All Releases](https://img.shields.io/github/downloads/xresloader/xresloader-dump-bin/total)
![GitHub forks](https://img.shields.io/github/forks/xresloader/xresloader-dump-bin?style=social)
![GitHub stars](https://img.shields.io/github/stars/xresloader/xresloader-dump-bin?style=social)

A tool to dump human readable text from binary output of [xresloader][1] .

## Usage

```bash
./xresloader-dump-bin --help

./xresloader-dump-bin -p kind.pb -b arr_in_arr_cfg.bin

xresloader-dump-bin.exe --help
```

You can use environment `RUST_LOG=<level>` to control log level and `RUST_LOG_STYLE=style` to set log style.

## cargo configure example

File path ```~/.cargo/config.toml``` or ```~/.cargo/config```

```toml

[cargo-new]
name = "Your Name"        # name to use in `authors` field
email = "you@example.com" # email address to use in `authors` field
vcs = "none"              # VCS to use ('git', 'hg', 'pijul', 'fossil', 'none')

[http]
debug = false               # HTTP debugging
proxy = "host:port"         # HTTP proxy in libcurl format
ssl-version = "tlsv1.3"     # TLS version to use
ssl-version.max = "tlsv1.3" # maximum TLS version
ssl-version.min = "tlsv1.1" # minimum TLS version
timeout = 30                # timeout for each HTTP request, in seconds
low-speed-limit = 10        # network timeout threshold (bytes/sec)
cainfo = "cert.pem"         # path to Certificate Authority (CA) bundle
check-revoke = true         # check for SSL certificate revocation
multiplexing = true         # HTTP/2 multiplexing
user-agent = "…"            # the user-agent header

[net]
retry = 2                   # network retries
git-fetch-with-cli = true   # use the `git` executable for git operations
offline = false             # do not access the network

[registries.<name>]  # registries other than crates.io
index = "…"          # URL of the registry index
token = "…"          # authentication token for the registry

[registry]
default = "…"        # name of the default registry
token = "…"          # authentication token for crates.io

[source.<name>]      # source definition and replacement
replace-with = "…"   # replace this source with the given named source
directory = "…"      # path to a directory source
registry = "…"       # URL to a registry source
local-registry = "…" # path to a local registry source
git = "…"            # URL of a git repository source
branch = "…"         # branch name for the git repository
tag = "…"            # tag name for the git repository
rev = "…"            # revision for the git repository


```

https://doc.rust-lang.org/cargo/reference/config.html

[1]: https://github.com/xresloader/xresloader
