#inquest

##Overview
A distributed service probing application.

##Compiling
protoc --rust_out=src/ protobuf/*.proto; protoc --rust-grpc_out=src/ protobuf/*.proto

#TODO
prober - schedule probe/cancel probe
