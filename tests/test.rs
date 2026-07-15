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
fn biter() {
    use slice_bit_operations::SliceBitOps;
    let array:[u8;4]=[1,2,3,4];
    let mut count:usize=0;
    array.biter(0..).for_each(|_bit| count+=1);
    assert_eq!(count,4*8);
    count=0;
    array.biter(0..=1).for_each(|_bit| count+=1);
    assert_eq!(count,2);
}

#[test]
fn biter_mut() {
    use slice_bit_operations::MutSliceBitOps;
    let mut array:[u8;4]=[1,2,3,4];
    let mut count:usize=0;
    array.biter_mut(0..).for_each(|_bit| count+=1);
    assert_eq!(count,4*8);
    count=0;
    array.biter_mut(0..=1).for_each(|_bit| count+=1);
    assert_eq!(count,2);
    array.biter_mut(0..).for_each(|mut bit| *bit = true);
    assert_eq!(array.iter().map(|&x| x as usize).sum::<usize>(),4*u8::MAX as usize);
}

#[test]
fn ctz_popcnt() {
    use slice_bit_operations::SliceBitOps;
    let array:[u8;4]=[1,2,3,4];
    let (start,end) = array.range_extract(0..); //start end bits
    let (sidx,sbit) = (<[u8;4]>::bits_idx(start), <[u8;4]>::bits_bit(start));
    let (eidx,ebit) = (<[u8;4]>::bits_idx(end), <[u8;4]>::bits_bit(end));
    assert_eq!(start,0);
    assert_eq!(end,4*8-1);
    assert_eq!(sbit, 0);
    assert_eq!(ebit, 7);
    assert_eq!(sidx,0);
    assert_eq!(eidx,3);
    assert_eq!(array.ctz(0..), 4*8-1-1-2-1);
    assert_eq!(array.ctz(5..), 4*8-5-1-2-1);
    assert_eq!(array.ctz(5..=7), 3);
    assert_eq!(array.ctz(0..)+array.popcnt(0..),4*8);
    assert_eq!(array.popcnt(0..), 1+1+2+1);
    assert_eq!(array.popcnt(0..=7), 1); //Array[0]
    assert_eq!(array.popcnt(7+1..=14+1), 1); //Array[1]
    assert_eq!(array.popcnt(14+1+1..=21+1), 2); //Array[2]
    assert_eq!(array.popcnt(21+1+1..=28+1), 1); //Array[3]
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

#[test]
fn frist_one_zero() {
    use slice_bit_operations::SliceBitOps;
    let array:[u8;2] = [0,1];
    assert_eq!(array.first_one(0..=7),None);
    assert_eq!(array[0..1].first_one(0..),None);
    assert_eq!(array.first_one(0..),Some(7+1));
    assert_eq!(array.first_one(7..=8),Some(8));
}
