# Network speed testing

## TODO

[x] Download testing
[ ] Upload testing
[ ] Better calculation of speed. With fancy units etc

## Protocol

Client -> Download 200
Server <- Ok
Client -> Read until '\0'

Client -> Upload 200
Server <- Ok
Client -> Write 200 end with '\0'

client state - Command Upload Download
server state - Command Upload Download
