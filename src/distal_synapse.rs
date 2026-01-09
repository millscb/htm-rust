/*
distal_synapse.rs for htm core module
HTM Community Edition of NuPIC
Copyright (C) 2013, Numenta, Inc.


https://github.com/egrembocki/psu-capstone/blob/main/src/psu_capstone/agent_layer/htm/distal_synapse.py


folling the python implementation of distal synapse from SWENG 480/481 capston PSU-WC

*/


use crate::cell::Cell;


pub struct DistalSynapse 
{
    pub id: u32,
    pub permanence: f32,
    pub connected: bool,
    pub source_cell: Cell,
    pub target_cell: Cell,
}



// unit test
#[cfg(test)]
mod tests
{
    #[test]
    fn test_create_distal_synapse() 
    {
        use super::*;
        
        let source_cell = Cell 
        {
            id: 11,
            col_id: 1,
            segments: vec![],
        };

        let target_cell = Cell 
        {
            id: 21,
            col_id: 2,
            segments: vec![],
        };

        let ds = DistalSynapse 
        {
            id: 1121,
            permanence: 0.5,
            connected: true,
            source_cell: source_cell,
            target_cell: target_cell,
        };

        assert_eq!(ds.id, 1121);
        assert_eq!(ds.permanence, 0.5);
        assert_eq!(ds.connected, true);
        assert_eq!(ds.source_cell.id, 11);
        assert_eq!(ds.target_cell.id, 21);
    }

}