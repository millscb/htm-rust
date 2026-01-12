
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
 * 
 * 
 * rust-htm implementation based on NuPIC C++ implementation
 * @author Chris Mills 
 *
 */


mod sdr;



// entry point
fn main() 
{

    // setup engine
  


    // basic testing

    println!("HTM Community Edition of NuPIC - Rust Implementation");

    let sdr = SparseDistributedRepresentation::new(2048, [0x01; sdr::INPUT_BYTES]);

    let active_bits = sdr.get_active_bits();

    println!("Active bits: {:?}", active_bits);




 
   





    // launch server







}
