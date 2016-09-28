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
- DNS resolution (https://github.com/mikedilger/resolv-rs)
- add prober hostname/ip_address to probe_results
- stop prober when it can't connect to configuration server for awhile
- make configuration server a distributed application that's fully redundant
- some security for users adding probes etc.
