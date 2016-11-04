use std::fs::File;

use pb::proddle::{Probe, ProbeResult};

use chrono::offset::utc::UTC;
use protobuf::{CodedOutputStream, Message, ProtobufError};

pub trait Writer {
    fn write_probe(&mut self, probe: &Probe) -> Result<(), ProtobufError>;
    fn write_probe_result(&mut self, probe_result: &ProbeResult) -> Result<(), ProtobufError>;
}

pub struct FileWriter {
    probe_file: File,
    probe_result_file: File,
    directory: String,
    probe_filesize: u32,
    probe_result_filesize: u32,
    max_filesize: u32,
}

impl FileWriter {
    pub fn new(directory: &str, max_filesize: u32) -> FileWriter {
        FileWriter {
            probe_file: create_file(directory, "pdp"),
            probe_result_file: create_file(directory, "pdr"),
            directory: directory.to_owned(),
            probe_filesize: 0,
            probe_result_filesize: 0,
            max_filesize: max_filesize,
        }
    }
}

fn create_file(directory: &str, extension: &str) -> File {
    File::create(format!("{}/{}.{}", directory, UTC::now().to_rfc3339(), extension)).unwrap()
}

impl Writer for FileWriter {
    fn write_probe(&mut self, probe: &Probe) -> Result<(), ProtobufError> {
        //write probe to file
        {
            let mut output_stream = CodedOutputStream::new(&mut self.probe_file);
            try!(output_stream.write_uint32_no_tag(probe.compute_size()));
            try!(probe.write_to_with_cached_sizes(&mut output_stream));
            try!(output_stream.flush());
        }

        //add filesize
        self.probe_filesize += 4 + probe.get_cached_size();
        if self.probe_filesize >= self.max_filesize {
            self.probe_file = create_file(&self.directory, "pdp");
            self.probe_filesize = 0;
        }
        
        Ok(())
    }

    fn write_probe_result(&mut self, probe_result: &ProbeResult) -> Result<(), ProtobufError> {
        //write probe result to file
        {
            let mut output_stream = CodedOutputStream::new(&mut self.probe_result_file);
            try!(output_stream.write_uint32_no_tag(probe_result.compute_size()));
            try!(probe_result.write_to_with_cached_sizes(&mut output_stream));
            try!(output_stream.flush());
        }

        //add filesize
        self.probe_result_filesize += 4 + probe_result.get_cached_size();
        if self.probe_result_filesize >= self.max_filesize {
            self.probe_result_file = create_file(&self.directory, "pdr");
            self.probe_result_filesize = 0;
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
    fn write_probe(&mut self, probe: &Probe) -> Result<(), ProtobufError> {
        println!("probe: {:?}", probe);
        Ok(())
    }

    fn write_probe_result(&mut self, probe_result: &ProbeResult) -> Result<(), ProtobufError> {
        println!("probe_result: {:?}", probe_result);
        Ok(())
    }
}

unsafe impl Send for PrintWriter {}
