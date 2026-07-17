# BitSliceOps
Bit Manipulation of a slice

## Examples
Note: Bits are 0 indexed

Get And Set bits in a Slice
```rust
    use slice_bit_operations::{MutSliceBitOps,SliceBitOps};
    let mut array:[u8;4]=[1,2,3,4];
    assert_eq!(array.bit_get(0),true);
    array.bit_set(0,false);
    assert_eq!(array.bit_get(0),false);
```

Get A mutable refrence to A bit in a Bitslice (proxy struct). MUST BE DROPPED FOR BIT TO UPDATE. DROP UPDATES!!!!
```rust
    use slice_bit_operations::{MutSliceBitOps,SliceBitOps};
    let mut array:[u8;4]=[1,2,3,4];
    assert_eq!(array.bit_get(0),true);
    {
        let mut mutbit = array.bit_get_mut(0);
        assert_eq!(*mutbit,true);
        *mutbit = false;
        assert_eq!(*mutbit,false);
    }
    assert_eq!(array.bit_get(0),false);
```

Iterate over bits in a Bitslice
```rust
    use slice_bit_operations::{MutSliceBitOps,SliceBitOps};
    let array:[u8;4]=[1,2,3,4];
    let mut set_bits:usize =0;
    array.bit_iter().for_each(|bit| set_bits+=bit as usize);
    assert_eq!(set_bits,1+1+2+1);
    //iterate over a bit range
    set_bits=0;
    array.biter(5..=10).for_each(|bit| set_bits+=bit as usize);
    assert_eq!(set_bits,1);

    
    let mut array = array;
    set_bits=0;
    array.bit_iter_mut().for_each(|mut bit| *bit = true); //Mutable proxy struct (MutBitProxy) as return, implemented for Mutable Bitslices
    array.bit_iter().for_each(|bit| set_bits+=bit as usize);
    assert_eq!(set_bits,4*8);
    assert_eq!(array.bit_len(),4*8); //Number of bits in BitSlice, Impl for Mutable and Immutable BitSlices
    //mutably iterate over a bit range
    array.biter_mut(0..=5).for_each(|mut bit| *bit = false);
    assert_eq!(array[0], 0b11000000)
    
```

number of set or unset bits in a slice
```rust
    use slice_bit_operations::SliceBitOps;
    let array: [u8;3] = [1,2,3];
    assert_eq!(array.popcnt(0..), 1+1+2);
    assert_eq!(array.ctz(0..), 3*8-1-1-2);
    assert_eq!(array.popcnt(3..=10), 1);
    assert_eq!(array.ctz(3..=10), 7);
```

first or last set or unset bit 
```rust 
    use slice_bit_operations::SliceBitOps;
    let array: [u8;3] = [1,2,3];
    assert_eq!(array.first_one(0..), Some(0));
    assert_eq!(array.first_zero(0..), Some(1));
    assert_eq!(array.last_one(0..), Some(2*8+1));
    assert_eq!(array.last_zero(0..), Some(3*8-1));
```

for full docs use docs.rs : [docs](https://docs.rs/slice_bit_operations/latest/)
for docs for Biters or BitOps from my other crates use their docs: [biter](https://docs.rs/biter/latest/), [bit_operations](https://docs.rs/bit_operations/latest/)
