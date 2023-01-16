# winrk

```
> winrk --help
winrk

    A command line program for load test of an HTTP server.

Usage:

    winrk [--help | -H] 
          [--duration <duration> | -d <duration>] 
          [--connections <connections> | -c <connections>] 
          [--threads <threads> | -t <threads>]
          [--method <method> | -m <method>]
          [--header '<key>:<value>' | -H '<key>:<value>']
          [--data <data> | -D <data>]
          [--timeout <timeout> | -T <timeout>]
          <url>

Options:

    --help (-H)        - print this help.
    --duration (-d)    - duration of the test in seconds (default 10).
    --connections (-c) - number of parallel connections (default 100).
    --threads (-t)     - number of CPU workers (default 1).
    --method (-m)      - HTTP method (default GET).
    --header (-H)      - HTTP header, can be multiple (default empty).
    --data (-D)        - HTTP payload (default '').
    --timeout (-T)     - connection timeout (default empty).
    url                - URL to request.
```

## Download

Download the installer from SourceForge: https://sourceforge.net/projects/winrk/files/latest/download

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
    total: 462 requests
    errors: 0 errors
    error percentage: 0.0%
    latency min: 168.2623ms
    latency median: 188.50075ms
    latency average: 211.496462ms
    latency max: 742.1273ms
    transfers: 580272 bytes
    rps: 945.6 requests per sec
```

## Packing from source

Step 1: `cargo build --release`
Step 2: `iscc winrk-setup.iss`
