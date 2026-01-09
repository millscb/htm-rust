/**
 * column.rs for htm core module
 * HTM Community Edition of NuPIC
 * Copyright (C) 2013, Numenta, Inc.
 * 
 * https://github.com/egrembocki/psu-capstone/blob/main/src/psu_capstone/agent_layer/htm/column.py
 * 
 * folling the python implementation of column from SWENG 480/481 capston PSU-WC
 */


use crate::cell::Cell;
use crate::synapse::Synapse;



pub struct Column 
{
    id: u32,
    position: (u32, u32),
    pub cells: Vec<Cell>, // cells in this column chaned my temporal memory pooler
    pub proximal_synapses: Vec<Synapse>, // to input space
}



// unit test
#[cfg(test)]
mod tests
{
    use super::*;

    #[test]
    fn test_column_creation() 
    {
        let column = Column 
        {
            id: 1,
            position: (0, 0),
            cells: vec![],
            proximal_synapses: vec![],
        };

        assert_eq!(column.id, 1);
        assert_eq!(column.position, (0, 0));
        assert!(column.cells.is_empty());
        assert!(column.proximal_synapses.is_empty());
    }
}