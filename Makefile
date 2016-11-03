pbdirinit:
	mkdir src/pb; \
	echo "pub mod proddle;" >> src/pb/mod.rs; \
	echo "pub mod proddle_grpc;" >> src/pb/mod.rs; 

pbinit: pbdirinit
	git init protobuf; \
	cd protobuf; \
	git remote add -f origin https://github.com/CSUNetSec/netsec-protobufs.git; \
	git config core.sparseCheckout true; \
	echo "proddle" >> .git/info/sparse-checkout;

pbpull:
	cd protobuf; \
	git pull origin master;

pbcompile: pbpull
	protoc --rust_out=src/pb protobuf/proddle/proddle.proto; \
	protoc --rust-grpc_out=src/pb protobuf/proddle/proddle.proto;
