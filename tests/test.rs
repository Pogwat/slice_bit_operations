#[test]
fn bitsliceops() {
    use slice_bit_operations::SliceBitOps;
    let array:[u8;4]=[1,2,3,4];
    assert_eq!(array.bit_len(),4*8);
    assert_eq!(array.bit_get(7+2),true);
    assert_eq!(array.bit_get(0),true);
    assert_eq!(array.bit_get(1),false);
    assert_eq!(<[u8;4]>::bits_idx(8),1);
    assert_eq!(<[u8;4]>::bits_bit(8),0);
    let mut num_bits:usize =0;
    array.bit_iter().for_each(|bit| num_bits+=bit as usize);
    assert_eq!(num_bits,1+1+2+1);
}
#[test]
fn mutbitsliceops() {
    use slice_bit_operations::{MutSliceBitOps,SliceBitOps};
    let mut array:[u8;4]=[1,2,3,4];
    assert_eq!(array.bit_get(0),true);
    array.bit_set(0,false);
    assert_eq!(array.bit_get(0),false);
    {
        let mut mutbit = array.bit_get_mut(0);
        assert_eq!(*mutbit,false);
        *mutbit = true;
        assert_eq!(*mutbit,true);
    }
    assert_eq!(array.bit_get(0),true);
    array.bit_iter_mut().for_each(|mut bit| *bit=false);
    assert_eq!(array.iter().sum::<u8>(),0);
    array.bit_iter_mut().for_each(|mut bit| *bit=true);
    assert_eq!(array.iter().map(|&x| x as usize).sum::<usize>(),4*u8::MAX as usize);
}
