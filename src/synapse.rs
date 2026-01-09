/*
 * synapes.rs for htm core module
 * HTM Community Edition of NuPIC   
 * Copyright (C) 2013, Numenta, Inc.
 * 
 *  https://github.com/egrembocki/psu-capstone/blob/main/src/psu_capstone/agent_layer/htm/synapse.py
 * 
 * folling the python implementation of synapse from SWENG 480/481 capston PSU-WC
 * 
 * 
 */


 pub struct Synapse 
 {
     pub id: u32,
     pub source_idx: u32, // mapping to input space bit 
     pub permanence: f32,
     pub connected: bool,
 }



 // unit test

#[cfg(test)]
mod tests 
{
    use super::*;

    #[test]
    fn test_synapse_creation() 
    {
        let synapse = Synapse 
        {
            id: 0,
            source_idx: 42,
            permanence: 0.5,
            connected: true,
        };

        assert_eq!(synapse.id, 0);
        assert_eq!(synapse.source_idx, 42);
        assert_eq!(synapse.permanence, 0.5);
        assert!(synapse.connected);
    }
}