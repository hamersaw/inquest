#inquest

##Overview
A distributed service probing application. Currently supports HTTP/HTTPS.

##Compiling
1. protoc --rust_out=src/ protobuf/*.proto
2. protoc --rust-grpc_out=src/ protobuf/*.proto
3. cargo build

##Components
####server
Configuration server which acts as a passive cache to schedule/cancel probes.

####prober
Actively submits probes to target hosts. Many instances will be deployed and
probe results will be gather for futher analysis. Probes are scheduled/canceled
by periodically polling the configuration server.

####inquisitor
Command line application to interact with the configuration server. Has the
ability to scheudle/cancel probes, list probe ids, and describe probes.

####glimpse
Simple utility tool to parse probe result protobuf files.

##TODO
- schedule requests in batches - only makes sense
- add SSL/TLS support for inquisitor->server and prober->server communication
- implement ping
        https://github.com/libpnet/libpnet
        https://tools.ietf.org/html/rfc792
- resolve IPv6 as well
- make configuration server a distributed application that's fully redundant
- stop prober when it can't connect to configuration server for awhile
