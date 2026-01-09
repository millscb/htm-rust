
/* ---------------------------------------------------------------------

htm segment -- module for handling segments in HTM
HTM Community Edition of NuPIC
Copyright (C) 2013, Numenta, Inc.

https://github.com/egrembocki/psu-capstone/blob/main/src/psu_capstone/agent_layer/htm/segment.py
folling the python implementation of segment from SWENG 480/481 capston PSU-WC

*/

use crate::distal_synapse::DistalSynapse;

pub struct Segment 
{
    id: u32,
    cell_id: u32,
    pub connected: bool,
    pub synapses: Vec<DistalSynapse>,
}





// unit test
#[cfg(test)]
mod tests 
{
    use super::*;
    #[test]
    fn test_create_segment() 
    {
        let segment_111 = Segment 
        {
            id: 111,
            cell_id: 11,
            connected: true,
            synapses: vec![],
        };

        let segment_112 = Segment 
        {
            id: 112,
            cell_id: 11,
            connected: true,
            synapses: vec![],
        };

        assert_eq!(segment_111.id, 111);
        assert_eq!(segment_111.cell_id, 11);
        assert_eq!(segment_111.connected, true);
        assert_eq!(segment_112.id, 112);
        assert_eq!(segment_112.cell_id, 11);
        assert_eq!(segment_112.connected, true);
    }

}