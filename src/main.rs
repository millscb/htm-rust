

mod sdr;

use crate::sdr::SparseDistributedRepresentation;


fn main()
{

    println!("Sparse Distributed Representation (SDR) Example");

    let mut sdr_one = SparseDistributedRepresentation::new();
    
    sdr_one.randomize_sdr();

    println!("SDR Dense Bits: {:02X?}", sdr_one.dense_bits);


}



