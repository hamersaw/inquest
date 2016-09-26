use std::io::Write;

use inquest_pb::ProbeResult;
use protobuf::{CodedOutputStream, Message, ProtobufError};

pub trait Writer {
    fn write_probe_result(&mut self, probe_result: &ProbeResult) -> Result<(), ProtobufError>;
}

pub struct StreamWriter {
    write: Box<Write>,
}

impl StreamWriter {
    pub fn new(write: Box<Write>) -> StreamWriter {
        StreamWriter {
            write: write,
        }
    }
}

impl Writer for StreamWriter {
    fn write_probe_result(&mut self, probe_result: &ProbeResult) -> Result<(), ProtobufError> {
        //try!(self.output_stream.write_message_no_tag(probe_result));
        let mut output_stream = CodedOutputStream::new(&mut self.write);
        try!(output_stream.write_uint32_no_tag(probe_result.compute_size()));
        try!(probe_result.write_to_with_cached_sizes(&mut output_stream));
        try!(output_stream.flush());
        
        Ok(())
    }
}

unsafe impl Send for StreamWriter {}

pub struct PrintWriter {
}

impl PrintWriter {
    pub fn new() -> PrintWriter {
        PrintWriter {
        }
    }
}

impl Writer for PrintWriter {
    fn write_probe_result(&mut self, probe_result: &ProbeResult) -> Result<(), ProtobufError> {
        println!("probe_result: {:?}", probe_result);
        Ok(())
    }
}

unsafe impl Send for PrintWriter {}
