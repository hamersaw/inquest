use std::fs::File;

use inquest_pb::ProbeResult;

use chrono::offset::utc::UTC;
use protobuf::{CodedOutputStream, Message, ProtobufError};

pub trait Writer {
    fn write_probe_result(&mut self, probe_result: &ProbeResult) -> Result<(), ProtobufError>;
}

pub struct FileWriter {
    file: File,
    directory: String,
    filesize: u32,
    max_filesize: u32,
}

impl FileWriter {
    pub fn new(directory: &str, max_filesize: u32) -> FileWriter {
        FileWriter {
            file: create_file(directory),
            directory: directory.to_owned(),
            filesize: 0,
            max_filesize: max_filesize,
        }
    }
}

fn create_file(directory: &str) -> File {
    File::create(format!("{}/{}.prd", directory, UTC::now().to_rfc3339())).unwrap()
}

impl Writer for FileWriter {
    fn write_probe_result(&mut self, probe_result: &ProbeResult) -> Result<(), ProtobufError> {
        //write probe result to file
        {
            let mut output_stream = CodedOutputStream::new(&mut self.file);
            try!(output_stream.write_uint32_no_tag(probe_result.compute_size()));
            try!(probe_result.write_to_with_cached_sizes(&mut output_stream));
            try!(output_stream.flush());
        }

        //add filesize
        self.filesize += 4 + probe_result.get_cached_size();
        if self.filesize >= self.max_filesize {
            self.file = create_file(&self.directory);
            self.filesize = 0;
        }
        
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
