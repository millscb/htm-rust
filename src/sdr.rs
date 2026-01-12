/*



*/




pub const BITS_PER_BYTE: usize = 8;
pub const ACTIVE_COLUMNS: usize = 40; // sparity of 2% for 2048 columns
pub const INPUT_BITS : usize = 2048;
pub const INPUT_BYTES : usize = INPUT_BITS / BITS_PER_BYTE; // 256 bytes for 2048 bits


trait SdrInterface 
{
    fn get_active_bits(&self) -> Vec<usize>;
   
    
}

pub struct SparseDistributedRepresentation
{
    size: usize,
    active_bits: [u8; INPUT_BYTES]
    
}

impl SparseDistributedRepresentation 
{
    pub fn new(size: usize, active_bits: [u8; INPUT_BYTES]) -> Self 
    {
        Self 
        {
           
            size: size,
            active_bits: active_bits,


        }
    }
}

impl SdrInterface for SparseDistributedRepresentation
{
    fn get_active_bits(&self) -> Vec<usize> 
    {
        let mut active_bits = Vec::new();

        for i in 0..INPUT_BITS 
        {
            if get_bit(&self.active_bits, i) == 1 
            {
                active_bits.push(i);
            }
        }

        return active_bits;
    }
}
   









// helper function to get bit value from packed byte array
#[inline]
fn get_bit(input: &[u8], index: usize) -> u8
{
    debug_assert!(input.len() == INPUT_BYTES, "Input length must be {} bytes", INPUT_BYTES);

    let byte_index = index / BITS_PER_BYTE;
    let bit_index = index % BITS_PER_BYTE;

    return (input[byte_index] >> bit_index) & 1; // return bit value (0 or 1)
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
        assert_eq!(get_bit(&input, 247), 1); // last byte
        assert_eq!(get_bit(&input, 248), 0);
        assert_eq!(get_bit(&input, 255), 0); // MSB

    }

}