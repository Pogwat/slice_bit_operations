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
fn first_one_zero() {
    use slice_bit_operations::{SliceBitOps,BitOps};
    let array:[u8;3] = [0,1,2];
    assert_eq!(array.first_one(0..=7),None);
    assert_eq!(array[0..1].first_one(0..),None);
    assert_eq!(array.first_one(0..),Some(7+1));
    assert_eq!(array.first_one(7..=8),Some(8));
    assert_eq!(array.first_one(7..8),None);
    assert_eq!(array.first_zero(0..), Some(0));
    assert_eq!(array.first_zero(8..), Some(9));
    assert_eq!(array.first_zero(8..9), None);
    let zarray:[u8;2] = [0b00000010,0b10000001];
    assert_eq!(zarray.first_one(8..9),Some(8));
    assert_eq!(zarray.first_one(7..8),None);
    assert_eq!(zarray[1].first_zero(&(2..3)), Some(2));
    assert_eq!(zarray.first_zero(10..11),Some(10));
    let oarray: [u8;1] = [1];
    let (start_bit,end_bit) = (0,0);

    let (starts_idx,starts_bit):(usize,u8) = (<[u8;1]>::bits_idx(start_bit),<[u8;1]>::bits_bit(start_bit) );
    let (ends_idx,ends_bit):(usize,u8) = (<[u8;1]>::bits_idx(end_bit), <[u8;1]>::bits_bit(end_bit));
    assert_eq!(starts_idx,0);
    assert_eq!(ends_idx,0);
    assert_eq!(starts_bit,0);
    assert_eq!(ends_bit,0);
    assert_eq!(u8::bitmask(&(starts_bit..=ends_bit)), 0b00000001);
    let truncated = oarray[0] & u8::bitmask(&(starts_bit..=ends_bit));
    assert_eq!(truncated,1);
    let op = (truncated!=u8::MAX).then_some(truncated.trailing_ones() as usize+start_bit -starts_bit as usize);
    println!("{:?}",op);
    assert_eq!(oarray.first_one(7..8),None);
    assert_eq!(oarray.first_zero(0..=0),None);
}

#[test]
fn last_one_zero() {
    use slice_bit_operations::{SliceBitOps,BitOps};
    let array:[u8;4] = [0,1,2,3];
    assert_eq!(array.last_zero(0..), Some(7+8*3));
    assert_eq!(array.last_zero(4..=5), Some(5));
    assert_eq!(array.last_zero(4..=5), Some(5));
    assert_eq!(array.last_zero(4..=5), Some(5));

    assert_eq!(array.bit_get(7+8*2-1), false);
    assert_eq!(array.bit_get(7+8*2), false);
    assert_eq!(array.bit_get(7+8*2+1), true);
    assert_eq!(array.bit_get(7+8*2+2), true);
    assert_eq!(array.bit_get(7+8*2+3), false);

    assert_eq!(array.last_zero(7+8*2..=7+8*2), Some(7+8*2));
    assert_eq!(array.last_zero(7+8*2..=7+8*2+2), Some(7+8*2));
    assert_eq!(array.last_zero(7+8*2..=7+8*2+3), Some(7+8*2+3));

    assert_eq!(array.last_one(7+8*2-1..=7+8*2), None);
    assert_eq!(array.last_one(7+8*2-1..=7+8*2+1), Some(7+8*2+1));
    assert_eq!(array.last_one(7+8*2-1..=7+8*2+2), Some(7+8*2+2));
    assert_eq!(array.last_one(7+8*2-1..=7+8*2+3), Some(7+8*2+2));

    //0-7   8-15   16-23    24-31    32-39
    let farry: [u8;5] = [!0,0,!0,!0,0];

    assert_eq!(farry.last_zero(0..),Some(39));
    assert_eq!(farry.bit_get(16),true);
    assert_eq!(farry.bit_get(15),false);
    assert_eq!(farry.last_zero(0..=23),Some(15));
    assert_eq!(farry.last_zero(0..=7),None);
    assert_eq!(farry.last_zero(16..=31),None);
    assert_eq!(farry.last_zero(16..=32),Some(32));

    assert_eq!(farry.last_one(16..=31),Some(31));
    assert_eq!(farry.last_one(0..),Some(31));

    //0-7   8-15   16-23    24-31    32-39   40-47   48-55
    let oarray: [u8;7] = [0,0,!0,!0,0,0,!0];
    assert_eq!(oarray.last_one(0..),Some(55));
    assert_eq!(oarray.last_one(0..=15),None);
    assert_eq!(oarray.last_one(21..=41),Some(31));
}
