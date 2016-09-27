#inquest

##Overview
A distributed service probing application.

##Compiling
protoc --rust_out=src/ protobuf/*.proto; protoc --rust-grpc_out=src/ protobuf/*.proto

#TODO
- set probe result max size
- stop prober when it can't connect to configuration server for awhile
- make configuration server a distributed application that's fully redundant
- some security for users adding probes etc.
