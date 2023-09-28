# Network Speed Testing CLI (nspeed)

Simple software to meassure network speed. Start server on a remote host and run client with
the data argument to send given amount of megabytes.

## Build

```
cargo build --release
```

## Usage Client

```
./speed client --help

Options:
  -H, --host <HOST>  Adress for server [default: 0.0.0.0]
  -p, --port <PORT>  Server port [default: 6666]
  -d, --data <DATA>  Amount of data to be sent/received under test [default: 800]
  -h, --help         Print help
```

![image](https://github.com/cannibalcow/nspeed/assets/6787042/683b7428-fc72-4074-9d32-3e380ce5131a)

## Usage server

```
./nspeed server --help

Options:
  -b, --bind <BIND>  Binding adress for server [default: 0.0.0.0]
  -p, --port <PORT>  Server port [default: 6666]
  -h, --help         Print help
```

![image](https://github.com/cannibalcow/nspeed/assets/6787042/72533aef-5db1-41d3-83ec-a2ea9aa845d2)

## Todos
```
  [ ] Better calculations for speed with fancy units and stuff
  [ ] Appendable csv/json outputfile
  [ ] Add retry functionallity
```
