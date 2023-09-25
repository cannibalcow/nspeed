# Network Speed Testing CLI (nspeed)

Simple software to meassure network speed. Start server on a remote host and run client with 
the data argument to send given amount of megabytes. 


## Usage Client

```bash
./speed client --help

Options:
  -H, --host <HOST>  Adress for server [default: 0.0.0.0]
  -p, --port <PORT>  Server port [default: 6666]
  -d, --data <DATA>  Amount of data to be sent/received under test [default: 800]
  -h, --help         Print help
```

![image](https://github.com/cannibalcow/nspeed/assets/6787042/1869e3c3-32cf-4e6c-8c18-489e13f48b97)

## Usage server

```bash
./nspeed server --help

Options:
  -b, --bind <BIND>  Binding adress for server [default: 0.0.0.0]
  -p, --port <PORT>  Server port [default: 6666]
  -h, --help         Print help
```
![image](https://github.com/cannibalcow/nspeed/assets/6787042/f1b900ea-b196-415d-a8de-8943c890f7ba)
