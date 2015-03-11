# urlp [![Build Status](https://travis-ci.org/clayallsopp/urlp.svg?branch=master)](https://travis-ci.org/clayallsopp/urlp)

A simple command-line utility for parsing URLs; does One Thing Well; uses Servo's URL parser; implemented in Rust


```shell
$ urlp --host http://www.google.com
google.com
$ echo http://www.google.com | urlp --host
www.google.com
$ echo http://localhost:4567 | urlp --port
4567
```

## Installation

## Usage

```shell
$ urlp --help
Usage: urlp OPTION URL

Options:
    --host                  Hostname
    --port                  Port
    --protocol, --scheme    Protocol
    --password              Password
    --username              Password
    --path                  Path
    --fragment              Fragment
    --query                 Query string
```