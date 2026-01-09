/*



*/




pub const INPUT_BITS : usize = 2048;
pub const INPUT_BYTES : usize = INPUT_BITS / 8;
pub const BITS_PER_BYTE: usize = 8;
pub const ACTIVE_COLUMNS: usize = 40; // sparity of 2% for 2048 columns


trait SdrInterface 
{
    fn get_active_bits(&self) -> Vec<usize>;
    fn get_sparse(&self) -> &[u8];
    
}

pub struct SDR 
{
    bits: [u8; INPUT_BYTES],
}

impl SDR 
{
    pub fn new() -> Self 
    {
        SDR 
        {
            bits: [0u8; INPUT_BYTES],
        }
    }
}

impl SdrInterface for SDR 
{
    fn get_active_bits(&self) -> Vec<usize> 
    {
        let mut active_bits = Vec::new();

        for i in 0..INPUT_BITS 
        {
            if get_bit(&self.bits, i) == 1 
            {
                active_bits.push(i);
            }
        }

        active_bits
    }

    fn get_sparse(&self) -> &[u8] 
    {
        &self.bits
    }
}

// helper function to get bit value from packed byte array
#[inline]
pub fn get_bit(input: &[u8], index: usize) -> u8
{
    debug_assert!(input.len() == INPUT_BYTES, "Input length must be {} bytes", INPUT_BYTES);

    let byte_index = index / BITS_PER_BYTE;
    let bit_index = index % BITS_PER_BYTE;

    return (input[byte_index] >> bit_index) & 1;
}


















// unit tests

#[cfg(test)]
mod tests 
{
    use super::*;

    #[test]
    fn test_get_bit() 
    {
        let input: [u8; INPUT_BYTES] = [0xAA; INPUT_BYTES]; // Example input with alternating bits

        assert_eq!(get_bit(&input, 0), 0); // LSB right to left
        assert_eq!(get_bit(&input, 1), 1);
        assert_eq!(get_bit(&input, 2), 0);
        assert_eq!(get_bit(&input, 3), 1);
        assert_eq!(get_bit(&input, 8), 0);
        assert_eq!(get_bit(&input, 9), 1);
        assert_eq!(get_bit(&input, 2047), 1);

    }

}