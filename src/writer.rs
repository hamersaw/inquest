use std::fs::File;

use inquest_pb::ProbeResult;
use protobuf::{CodedOutputStream, Message, ProtobufError};

pub trait Writer {
    fn write_probe_result(&mut self, probe_result: &ProbeResult) -> Result<(), ProtobufError>;
}

pub struct FileWriter {
    file: File,
}

impl FileWriter {
    pub fn new(directory: &str) -> FileWriter {
        let file = File::create(format!("{}/inquest.log", directory)).unwrap();

        FileWriter {
            file: file,
        }
    }
}

impl Writer for FileWriter {
    fn write_probe_result(&mut self, probe_result: &ProbeResult) -> Result<(), ProtobufError> {
        let mut output_stream = CodedOutputStream::new(&mut self.file);
        try!(output_stream.write_uint32_no_tag(probe_result.compute_size()));
        try!(probe_result.write_to_with_cached_sizes(&mut output_stream));
        try!(output_stream.flush());
        
        Ok(())

    }
}

unsafe impl Send for FileWriter {}

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
