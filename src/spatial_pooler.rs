
/* ---------------------------------------------------------------------
 * HTM Community Edition of NuPIC
 * Copyright (C) 2013, Numenta, Inc.
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU Affero Public License version 3 as
 * published by the Free Software Foundation.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.
 * See the GNU Affero Public License for more details.
 *
 * You should have received a copy of the GNU Affero Public License
 * along with this program.  If not, see http://www.gnu.org/licenses.
 * ---------------------------------------------------------------------- */

/** @file
 * Definitions for the Spatial Pooler in C++ 
 * 
 * rust implementation: spatial_pooler.rs
 * 
 */


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

pub const INPUT_BITS : usize = 2048;
pub const INPUT_BYTES : usize = INPUT_BITS / 8;
pub const BITS_PER_BYTE: usize = 8;
pub const ACTIVE_COLUMNS: usize = 40; // sparity of 2% for 2048 columns


// helper function to get bit value from packed byte array
#[inline]
pub fn get_bit(input: &[u8], index: usize) -> u8
{
    debug_assert!(input.len() == INPUT_BYTES, "Input length must be {} bytes", INPUT_BYTES);

    let byte_index = index / BITS_PER_BYTE;
    let bit_index = index % BITS_PER_BYTE;

    return (input[byte_index] >> bit_index) & 1;
}


/// Spatial Pooler Interface

trait SpatialPoolerInterface 
{
    fn initialize(&mut self, input_dimensions: Vec<usize>, column_dimensions: Vec<usize>);
    fn compute(&mut self, input_vector: &[u8], learn: bool, active_columns: &mut Vec<usize>);
   

}


// Spatial Pooler Implementation
pub struct SpatialPooler 
{
    input_dimensions: Vec<usize>,
    column_dimensions: Vec<usize>,
  
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