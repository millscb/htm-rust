/**
 * ### Description
 * The Spatial Pooler is responsible for creating a sparse distributed
 * representation of the input. Given an input it computes a set of sparse
 * active columns and simultaneously updates its permanences, duty cycles,
 * etc.
 *
 * The primary public interfaces to this function are the "initialize"
 * and "compute" methods.
 *
 * Example usage:
 *
 *     SpatialPooler sp;
 *     sp.initialize(inputDimensions, columnDimensions, <parameters>);
 *     while (true) {
 *        <get input vector>
 *        sp.compute(inputVector, learn, activeColumns)
 *        <do something with output>
 *     }
 *
 */

// Pack bits input vector into bytes
// 2048 bits -> 256 bytes (256 * 8 = 2048)
// bit is in byte i / 8 at position i % 8
// e.g., bit 13 is in byte 1 (13 / 8 = 1) at position 5 (13 % 8 = 5)



pub const INPUT_BYTES: usize = 256;
pub const BITS_PER_BYTE: usize = 8;
pub const INPUT_ZERO_VECTOR: [u8; INPUT_BYTES] = [0x00; INPUT_BYTES]; 
pub const INPUT_ONES_VECTOR: [u8; INPUT_BYTES] = [0xFF; INPUT_BYTES];

/// Spatial Pooler Interface

trait SpatialPoolerInterface 
{
    fn initialize(&mut self, input_dimensions: Vec<usize>, column_dimensions: Vec<usize>);
    fn compute(&mut self, input_vector: &[u8], learn: bool, active_columns: &mut Vec<usize>);
   

}


// Spatial Pooler struct
pub struct SpatialPooler 
{
    
    input_dimensions: Vec<usize>,
    column_dimensions: Vec<usize>,
    input_vector: [u8; INPUT_BYTES],
    active_columns: Vec<usize>,
    initial_synapse_column: u32
    
    
}



// Implement Spatial Pooler Interface for Spatial Pooler struct
impl SpatialPoolerInterface for SpatialPooler
{

    fn initialize(&mut self, input_dimensions: Vec<usize>, column_dimensions: Vec<usize>) 
    {
        self.input_dimensions = input_dimensions;
        self.column_dimensions = column_dimensions; 
        self.input_size = self.input_dimensions.iter().product();
        self.column_size = self.column_dimensions.iter().product();
        self.input_vector = INPUT_ZERO_VECTOR;
        self.active_columns = vec![];
        self.active_columns.resize(self.column_size, 0);
    



    }



    fn compute(&mut self, input_vector: &[u8], learn: bool, active_columns: &mut Vec<usize>) 
    {
        // Placeholder implementation
       
    }






}

