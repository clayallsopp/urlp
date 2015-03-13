# urlp [![Build Status](https://travis-ci.org/clayallsopp/urlp.svg?branch=master)](https://travis-ci.org/clayallsopp/urlp)

A simple command-line utility for parsing URLs. No more awk/sed/grep incantations. Does [One Thing Well](http://en.wikipedia.org/wiki/Unix_philosophy).

Implemented in Rust, using [Servo's URL parser](https://github.com/servo/rust-url).


```shell
$ urlp --host http://www.google.com
google.com
$ echo http://www.google.com | urlp --host
www.google.com
$ echo http://localhost:4567 | urlp --port
4567
```

## Installation

urlp is available for Linux and OS X, 64-bit only for now:

```
curl -L https://github.com/clayallsopp/urlp/releases/download/1.0.0/urlp-`uname -s`-x86_64 > /usr/local/bin/urlp; chmod +x /usr/local/bin/urlp
```

If you want to install from the source, checkout the repo and run `make install`.

## Usage

```shell
$ urlp --help
Usage: urlp OPTION URL

Options:
    --host                  Hostname
    --port                  Port
    --protocol, --scheme    Protocol
    --password              Password
    --username              Username
    --path                  Path
    --fragment              Fragment
    --query                 Query string
```