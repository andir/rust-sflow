// Local Imports
use types::*;
use utils::ReadBytesLocal;
use flow_records::FlowRecord;
use error::Result;

// Std Lib Imports
use std::io::SeekFrom;

#[derive(Debug, Clone)]
pub enum SampleRecord {
    FlowSample(FlowSample),
    Unknown,
}

add_decoder!{
#[derive(Debug, Clone, Default)]
pub struct FlowSample {
    // Incremented with each flow sample generated by this source_id. Note: If the agent resets the
    // sample_pool then it must also reset the sequence_number.
    pub sequence_number: u32,

    // sFlowDataSource
    pub sflow_data_source: SourceID,

    // sFlowPacketSamplingRate
    pub sampling_rate: u32,

    // Total number of packets that could have been sampled (i.e. packets skipped by sampling
    // process + total number of samples)
    pub sample_pool: u32,

    // Number of times that the sFlow agent detected that a packet marked to be sampled was dropped
    // due to lack of resources. The drops counter reports the total number of drops detected since
    // the agent was last reset. A high drop rate indicates that the management agent is unable to
    // process samples as fast as they are being generated by hardware. Increasing sampling_rate
    // will reduce the drop rate. Note: An agent that cannot detect drops will always report zero.
    pub drops: u32,

    // Interface packet was received on.
    pub input_id: Interface,

    // Interface packet was sent on.
    pub output_id: Interface,

    // Information about a sampled packet */
    pub flow_records: Vec<FlowRecord>,
}
}

impl ::utils::Decodeable for Vec<SampleRecord> {
    fn read_and_decode(stream: &mut ReadSeeker) -> Result<Vec<SampleRecord>> {
        // First we need to figure out how many samples there are.
        let count = try!(stream.be_read_u32());
        let mut results: Vec<SampleRecord> = Vec::new();

        for _ in 0..count {
            let format = try!(stream.be_read_u32());
            let length = try!(stream.be_read_u32());

            match format {
                1 => {
                    let fs: FlowSample = try!(::utils::Decodeable::read_and_decode(stream));
                    results.push(SampleRecord::FlowSample(fs));
                }
                // Skip unknown samples.
                _ => {
                    results.push(SampleRecord::Unknown);
                    try!(stream.seek(SeekFrom::Current(length as i64)));
                }
            }
        }

        Ok(results)
    }
}