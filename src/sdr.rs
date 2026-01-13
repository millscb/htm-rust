/*



*/


use std::char::MAX;

use rand::Rng;


pub const BITS_PER_BYTE: usize = 8;
pub const INPUT_BITS : usize = 2048;
pub const INPUT_BYTES : usize = INPUT_BITS / BITS_PER_BYTE; // 256 bytes for 2048 bits
pub const MAX_ACTIVE_BITS: usize = 40; // maximum number of active bits in SDR
pub const SPARSITY: f32 = (MAX_ACTIVE_BITS as f32) / (INPUT_BITS as f32); // 0.02 or 2% sparsity
pub const BOOST_FACTOR: usize = 100; // boost factor to increase sparsity



pub struct SparseDistributedRepresentation
{
   
    pub dense_bits: [u8; INPUT_BYTES],
    pub sparse_bits:[u8; MAX_ACTIVE_BITS],
    sparsity_count: usize,
    sparsity_ratio: f32,
    boost_factor: usize,
    
}

impl SparseDistributedRepresentation 
{
    pub fn new()-> Self 
    {
        Self 
        {                      
            dense_bits: [0; INPUT_BYTES],
            sparse_bits: [0; MAX_ACTIVE_BITS],
            sparsity_count: 0,
            sparsity_ratio: SPARSITY,
            boost_factor: BOOST_FACTOR,

        }
    }


    pub fn set_by_decimal_dense(&mut self, byte_index: usize, value: u8)
    {
        debug_assert!(byte_index < INPUT_BYTES, "Index out of bounds");
       // debug_assert!(value <= 255, "Value must be between 0 and 255");
       
       
        self.dense_bits[byte_index] = value;

       
    }

    pub fn randomize_sdr(&mut self) 
    {
        let mut random_byte = rand::rng();
        let mut skip = rand::rng();
       

        loop 
        {           
            for byte in self.dense_bits.iter_mut()
            {
                // assume that the first loop will not exceed MAX_ACTIVE_BITS

                let skip = skip.random_range(0..=INPUT_BITS * MAX_ACTIVE_BITS * BOOST_FACTOR ); // randomly skip some bytes to increase sparsity

                if skip > MAX_ACTIVE_BITS
                {
                    continue;
                }    
                                              
                
                if *byte != 0
                {
                    continue;
                }
                 
                
                else if self.sparsity_count <= MAX_ACTIVE_BITS 
                {
                    *byte = random_byte.random_range(0..=255);
                    self.sparsity_count += byte.count_ones() as usize; 
                    
                }
                
                
                
                
            }
            
            
            if self.sparsity_count >= MAX_ACTIVE_BITS 
            {
                println!("Reached maximum active bits: {}", MAX_ACTIVE_BITS);
                break;  
            }
            
        }

     
        
    }
 

}




// helper functions  to get and set a bit value from packed byte array

#[inline]
pub fn get_bit(input: &[u8], index: usize) -> u8
{
    debug_assert!(input.len() == INPUT_BYTES, "Input length must be {} bytes", INPUT_BYTES);

    let byte_index = index / BITS_PER_BYTE;
    let bit_index = index % BITS_PER_BYTE;

    return (input[byte_index] >> bit_index) & 1; // return bit value (0 or 1)
}

#[inline]
fn set_bit(input: &mut [u8], index: usize, value: u8)
{
    debug_assert!(input.len() == INPUT_BYTES, "Input length must be {} bytes", INPUT_BYTES);
    debug_assert!(value == 0 || value == 1, "Value must be 0 or 1");

    let byte_index = index / BITS_PER_BYTE;
    let bit_index = index % BITS_PER_BYTE;

    if value == 1 
    {
        input[byte_index] |= 1 << bit_index; // set bit
    } 
    else 
    {
        input[byte_index] &= !(1 << bit_index); // clear bit
    }
}







// unit tests

#[cfg(test)]
mod tests 
{
    use super::*;

    #[test]
    fn test_get_bit() 
    {
        let input: [u8; INPUT_BYTES] = [0x03; INPUT_BYTES]; // Example input with alternating bits

        assert_eq!(get_bit(&input, 0), 1); // LSB right to left
        assert_eq!(get_bit(&input, 1), 1);
        assert_eq!(get_bit(&input, 2), 0);
        assert_eq!(get_bit(&input, 3), 0); // nimble
        assert_eq!(get_bit(&input, 4), 0);
        assert_eq!(get_bit(&input, 5), 0);
        assert_eq!(get_bit(&input, 6), 0);
        assert_eq!(get_bit(&input, 7), 0);
        assert_eq!(get_bit(&input, 8), 1); // Next byte
        assert_eq!(get_bit(&input, 9), 1);
        assert_eq!(get_bit(&input, 247), 1); 
        assert_eq!(get_bit(&input, 248), 0);
        assert_eq!(get_bit(&input, 255), 0); 

    }

}