# winrk

[![GitHub stars](https://img.shields.io/github/stars/fomalhaut88/winrk?style=social)](https://github.com/fomalhaut88/winrk/stargazers)
![Top Language](https://img.shields.io/github/languages/top/fomalhaut88/winrk)
![Version](https://img.shields.io/badge/version-v0.1.4-green)
![License](https://img.shields.io/badge/license-MIL-orange)

```
> winrk --help
Winrk is an HTTP benchmarking tool for Windows users, inspired by wrk.

Usage: winrk [OPTIONS] <URL>

Arguments:
  <URL>  URL to load

Options:
  -d, --duration <DURATION>        Load duration [default: 10]
  -c, --connections <CONNECTIONS>  Number of simultaneous connections [default: 100]
  -t, --threads <THREADS>          Number of CPU threads [default: 1]
  -m, --method <METHOD>            HTTP method [default: GET]
  -D, --data <DATA>                Request body [default: ]
  -H, --header <HEADER>            HTTP header
  -T, --timeout <TIMEOUT>          Request timeout
  -h, --help                       Print help
  -V, --version                    Print version
```

Read more on [Medium - Winrk is a wrk alternative for Windows users](https://medium.com/@afomalhaut/winrk-a-wrk-alternative-for-windows-users-8c6aa6c6e23b).

## Download

Download the installer from SourceForge: https://sourceforge.net/projects/winrk/files/latest/download

After installation restart your computer.

## Example

Command:

```
winrk https://example.com -t 4 -d 5 -c 200
```

Output:

```
Input:
    url: https://example.com
    method: GET
    threads: 4
    duration: 5s
    connections: 200

Result:
    total: 5733 requests
    errors: 0 errors
    error percentage: 0.0%
    latency min: 148.93645ms
    latency median: 200.466559ms
    latency average: 212.157226ms
    latency max: 927.661398ms
    transfers: 1.129 MB per sec
    rps: 942.7 requests per sec
```

## Packing from source

Step 1: `cargo build --release`

Step 2: `iscc winrk-setup.iss`
