extern crate docopt;
extern crate inquest;
extern crate protobuf;
extern crate rustc_serialize;

use std::fs::File;

use docopt::Docopt;
use inquest::pb::proddle::ProbeResult;
use protobuf::{CodedInputStream, Message};

const USAGE: &'static str = "
Application to parse probe result protobuf files

Usage:
    glimpse <filename>
    glimpse (-h | --help)

Options:
    -h --help               Show this screen.
";

#[derive(Debug, RustcDecodable)]
struct Args {
    arg_filename: String,
}

fn main() {
    let args: Args = Docopt::new(USAGE)
                        .and_then(|d| d.decode())
                        .unwrap_or_else(|e| e.exit());

    let mut file = File::open(args.arg_filename).unwrap();

    let mut input_stream = CodedInputStream::new(&mut file);
    loop {
        //read length of protobuf message
        let length = input_stream.read_uint32().unwrap();

        //read bytes for messages
        let mut bytes = Vec::new();
        for _ in 0..length {
            let byte = input_stream.read_raw_byte().unwrap();
            bytes.push(byte);
        }

        //parse message
        let mut message_input_stream = CodedInputStream::from_bytes(&bytes);
        let mut probe_result = ProbeResult::new();
        let _ = probe_result.merge_from(&mut message_input_stream);
        println!("probe_result: {:?}", probe_result);

        //checck if end of file
        if input_stream.eof().unwrap() {
            break;
        }
    }
}
