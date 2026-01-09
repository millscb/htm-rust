/*
* engine to drive the htm components
* htm community edition of nupic



 */



mod sdr;
mod spatial_pooler;
mod temporal_memory;

pub struct HTMEngine 
{
    pub input_sdr: sdr::SDR,
    pub spatial_pooler: spatial_pooler::SpatialPooler
    pub temporal_memory: temporal_memory::TemporalMemory,
}



impl HTMEngine 
{
    pub fn new() -> Self 
    {
        HTMEngine 
        {
            spatial_pooler: spatial_pooler::SpatialPooler::new(), 
            temporal_memory: temporal_memory::TemporalMemory::new(),
            input_sdr: sdr::SDR::new(),
        }
    }
}




