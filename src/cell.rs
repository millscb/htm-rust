/*
cell base module for htm core module
HTM Community Edition of NuPIC
Copyright (C) 2013, Numenta, Inc.

https://github.com/egrembocki/psu-capstone/blob/main/src/psu_capstone/agent_layer/htm/cell.py

folling the python implementation of cell from SWENG 480/481 capston PSU-WC

*/

// hold a list of segments (distal)

//use crate::segment::Segment;


#[derive(Default)]
pub struct Cell 
{
    id: u32 ,
    col_id: u32 ,
    //segments: Vec<Segment>,
}


impl Cell 
{
    pub fn new(id: u32, col_id: u32) -> Self // segments: Vec<Segment>) -> Self 
    {
        Cell 
        {
            id,
            col_id,
            //segments,
        }
    }

    pub fn get_id(&self) -> u32 
    {
        self.id
    }
    pub fn get_col_id(&self) -> u32 
    {
        self.col_id
    }

    /* 
    fn get_segments(&self) -> &Vec<Segment> 
    {
    &self.segments
    }
    */


}





// unit test

#[cfg(test)]
mod tests 
{
    use super::*;   
    
    #[test]
    fn test_create_cell() 
    {
        let cell_11 = Cell::new(11, 1);
        let cell_12 = Cell::new(12, 1);
        let cell_21 = Cell::new(21, 2);
        let cell_22 = Cell::new(22, 2);  
    


       assert_eq!(cell_11.get_id(), 11);
       assert_eq!(cell_11.get_col_id(), 1);
       assert_eq!(cell_12.get_id(), 12);
       assert_eq!(cell_12.get_col_id(), 1);
       assert_eq!(cell_21.get_id(), 21);
       assert_eq!(cell_21.get_col_id(), 2);
       assert_eq!(cell_22.get_id(), 22);
       assert_eq!(cell_22.get_col_id(), 2);

      /* 
      assert_eq!(cell_11.get_segments().len(), 0);
      assert_eq!(cell_12.get_segments().len(), 0);
      assert_eq!(cell_21.get_segments().len(), 0);
      assert_eq!(cell_22.get_segments().len(), 0);
      */  
    }
        
}