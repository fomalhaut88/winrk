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

## Example

Command:

```
winrk http://localhost:8000/test -t 4 -d 5 -c 200
```

Output:

```
Input:
    url: http://localhost:8000/test
    method: GET
    threads: 4
    duration: 5s
    connections: 200

Result:
    total: 6949 requests
    errors: 0 errors
    error percentage: 0.0%
    latency min: 2.6628ms
    latency median: 69.6987ms
    latency average: 119.560015ms
    latency max: 4.7167885s
    transfers: 41694 bytes
    rps: 1672.8 requests per sec
```

## Packing from source

Step 1: `cargo build --release`
Step 2: `iscc winrk-setup.iss`
