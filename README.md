#inquest

##Overview
A distributed service probing application. Currently supports HTTP/HTTPS.

##Compiling
Note that you must have rust-protoc 
(https://github.com/stepancheg/rust-protobuf) and rust-protoc-grpc 
(https://github.com/stepancheg/grpc-rust) binarys installed.

1. make pbinit
2. make pbcompile
3. cargo build

##Components
####inquest_server
Configuration server which acts as a passive cache to schedule/cancel probes.

####inquest_prober
Actively submits probes to target hosts. Many instances will be deployed and
probe results will be gather for futher analysis. Probes are scheduled/canceled
by periodically polling the configuration server.

####inquisitor
Command line application to interact with the configuration server. Has the
ability to scheudle/cancel probes, list probe ids, and describe probes.

##TODO
- parameterize many prober & server variables
- add SSL/TLS support for inquisitor->server and prober->server communication
- implement ping
    https://github.com/libpnet/libpnet
    https://tools.ietf.org/html/rfc792
- implement traceroute
- resolve IPv6 as well - (DNS easy)
- make configuration server a distributed application that's fully redundant
- stop prober when it can't connect to configuration server for awhile
