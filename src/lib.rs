#![no_std]
#[doc = include_str!("../README.md")]
pub use bit_operations::{BitOps,MutBitProxy, NumRangeExtract};
pub use biter::{Biter,MutBiter};

macro_rules! counters {
    (   $(#[$meta:meta])* $fn_name:ident, $bit_method:ident, $word_method:ident) => {
        $(#[$meta])*
        fn $fn_name<R:NumRangeExtract<usize> >(&self,range:R) -> usize {
            let (start_bit,end_bit) = self.range_extract(range); //start end bits
            let (start_idx,start_bit):(usize,u8) = (Self::bits_idx(start_bit), Self::bits_bit(start_bit));
            let (end_idx,end_bit):(usize,u8) = (Self::bits_idx(end_bit), Self::bits_bit(end_bit));
            let mut acc:usize=0;
            if start_idx==end_idx {
                acc+= unsafe {self.as_ref().get_unchecked(end_idx).$bit_method(&(start_bit..=end_bit)) as usize};
            } else {
                acc+= unsafe {self.as_ref().get_unchecked(start_idx).$bit_method(&(start_bit..)) as usize};
                acc+= unsafe {self.as_ref().get_unchecked(end_idx).$bit_method(&(..=end_bit)) as usize};
                self.as_ref()[start_idx..end_idx].iter().skip(1).for_each(|num| acc+=num.$word_method() as usize );
            }
            acc
        }
    };
}

macro_rules! first_bit {
    ($(#[$meta:meta])* $fn_name:ident, $method:ident, $cantbe:expr) => {
        $(#[$meta])*
        fn $fn_name<R:NumRangeExtract<usize> >(&self,range:R) -> Option<usize> {
            let (start_bit,end_bit) = self.range_extract(range);
            let (starts_idx,starts_bit):(usize,u8) = (Self::bits_idx(start_bit), Self::bits_bit(start_bit));
            let (ends_idx,ends_bit):(usize,u8) = (Self::bits_idx(end_bit), Self::bits_bit(end_bit));
            if ends_idx==starts_idx {
                let truncated = unsafe {*self.as_ref().get_unchecked(starts_idx) } & ElementType::bitmask(&(starts_bit..=ends_bit));
                (truncated!=$cantbe).then_some(truncated.$method() as usize+start_bit -starts_bit as usize)
            } else {
                    let truncated_start =  unsafe {*self.as_ref().get_unchecked(starts_idx) } & ElementType::bitmask(&(starts_bit..) );
                    (truncated_start!=$cantbe).then_some(truncated_start.$method() as usize-starts_bit as usize+start_bit)
                .or_else(||
                    self.as_ref()[starts_idx..ends_idx].iter().skip(1).position(|num| *num!=$cantbe).map(|idx| (starts_idx+ idx + 1)*(ElementType::BITS as usize)+(self.as_ref()[starts_idx+idx+1].$method() as usize) ) //Skip first and last
                ).or_else(|| {
                    let truncated_end =  unsafe {*self.as_ref().get_unchecked(ends_idx) } & ElementType::bitmask(&(..=ends_bit) );
                    (truncated_end!=$cantbe).then_some(truncated_end.$method() as usize + end_bit -ends_bit as usize )
                })
            }
        }
    };
}
use core::iter;
use core::ops::RangeBounds;
use core::slice::SliceIndex;
/// Methods for Immutable BitSlice
pub trait SliceBitOps<ElementType:BitOps+>:AsRef<[ElementType]> {
    ///Number of bits in BitSlice
    fn bit_len(&self) -> usize {self.as_ref().len()*ElementType::BITS as usize}
    /// Global Bit Index to bit positon in a element
    fn bits_idx(bit:usize) -> usize {bit/ElementType::BITS as usize}
    /// Global Bit Index to a element index
    fn bits_bit(bit:usize) -> u8 {(bit%ElementType::BITS as usize) as u8 }
    /// Get bit by global index
    fn bit_get(&self,bit:usize) -> bool {self.as_ref()[Self::bits_idx(bit)].get_bit(Self::bits_bit(bit))}
    /// Iterate over a BitSlice, yields bools
    fn bit_iter<'short>(&'short self) -> Biter<'short,ElementType> {Biter::from(self)}
    ///start and end bits for a bit range
    fn range_extract<R:NumRangeExtract<usize> >(&self,range:R) -> (usize,usize) { ( range.start().unwrap_or(0).min(self.bit_len()-1), range.end().unwrap_or(self.bit_len()-1).min(self.bit_len()-1) ) }
    ///iterate over bits using a range
    fn biter<'short, R:NumRangeExtract<usize> >(&'short self, range:R) -> Biter<'short,ElementType> {
        let (start,end) = self.range_extract(range); //start end bits
        let spointer = unsafe { (self.as_ref() as *const [ElementType] as *const ElementType).add(Self::bits_idx(start)) };
        unsafe { Biter::new(spointer, Self::bits_bit(start), end-start+1) } //startptr, start bit pos, remaning bits
    }
    fn first_one_idx<R:SliceIndex<[ElementType], Output = [ElementType]> >(&self,idx_range:R) -> Option<usize>{
        self.as_ref()[idx_range].iter().position(|num| *num!=ElementType::ZERO)
    }
    fn first_zero_idx<R:SliceIndex<[ElementType], Output = [ElementType]> >(&self,idx_range:R) -> Option<usize>{
        self.as_ref()[idx_range].iter().position(|num| *num!=!ElementType::ZERO)
    }
    fn first_one<R:NumRangeExtract<usize>+SliceIndex<[ElementType], Output = [ElementType]> >(&self, range:R) -> Option<usize> {
        let (start_bit,end_bit) = self.range_extract(range);
        let (starts_idx,starts_bit):(usize,u8) = (Self::bits_idx(start_bit), Self::bits_bit(start_bit));
        let (ends_idx,ends_bit):(usize,u8) = (Self::bits_idx(end_bit), Self::bits_bit(end_bit));
        if starts_idx == ends_idx {
            self.as_ref()[starts_idx].first_one(&(starts_bit..=ends_bit)).map(|bit| bit as usize + start_bit - starts_bit as usize)
        } else {
            self.as_ref()[starts_idx].first_one(&(starts_bit..)).map(|bitpos| bitpos as usize +start_bit)
                .or_else(||
            self.as_ref()[starts_idx..ends_idx].iter().skip(1).position(|num| *num!=ElementType::ZERO).map(|position| (position+1)*ElementType::BITS as usize + start_bit + self.as_ref()[position+1].trailing_zeros() as usize )
                ).or_else(||
            self.as_ref()[ends_idx].first_one(&(..=ends_bit)).map(|bitpos| bitpos as usize + end_bit - ends_bit as usize)
            )
        }
    }

    fn first_zero<R:NumRangeExtract<usize>+SliceIndex<[ElementType], Output = [ElementType]> >(&self, range:R) -> Option<usize> {
        let (start_bit,end_bit) = self.range_extract(range);
        let (starts_idx,starts_bit):(usize,u8) = (Self::bits_idx(start_bit), Self::bits_bit(start_bit));
        let (ends_idx,ends_bit):(usize,u8) = (Self::bits_idx(end_bit), Self::bits_bit(end_bit));
        if starts_idx == ends_idx {
            self.as_ref()[starts_idx].first_zero(&(starts_bit..=ends_bit)).map(|bit| bit as usize + start_bit - starts_bit as usize)
        } else {
            self.as_ref()[starts_idx].first_zero(&(starts_bit..)).map(|bitpos| bitpos as usize +start_bit)
                .or_else(||
            self.as_ref()[starts_idx..ends_idx].iter().enumerate().skip(1).find_map(|(idx,num)| *&num.first_zero(&(0..)).map(|fz| fz as usize+start_bit+(idx+1)*ElementType::BITS as usize))
                ).or_else(||
            self.as_ref()[ends_idx].first_zero(&(..=ends_bit)).map(|bitpos| bitpos as usize + end_bit - ends_bit as usize)
            )
        }
    }
    counters!(
        ///Count zeros form bit range
        ctz,ctz,count_zeros);
    counters!(
        ///Count ones form bit range
        popcnt,popcnt,count_ones);

    // first_bit!(
    //     ///Find first ones bit index from bit range
    //     first_one, trailing_zeros, ElementType::ZERO);
    // first_bit!(
    //     ///Find first zeros bit index from bit range
    //     first_zero, trailing_ones, ElementType::MAX);


}




/// Methods for Mutable BitSlice
pub trait MutSliceBitOps<ElementType:BitOps>:SliceBitOps<ElementType>+AsMut<[ElementType]> {
    /// Set a bit by global index in a BitSlice
    fn bit_set(&mut self,bit:usize, val:bool) {self.as_mut()[Self::bits_idx(bit)].set_bit(Self::bits_bit(bit),val)}
    /// Get Mutable refrence to Bit (porxy struct: MutBitProxy), REF MUST BE DROPPED FOR BIT TO UPDATE. DROP UPDATES!!!
    fn bit_get_mut<'short>(&'short mut self, bit:usize) -> MutBitProxy<'short,ElementType> {self.as_mut()[Self::bits_idx(bit)].mut_bit(Self::bits_bit(bit))}
    /// Mutably Iterate over a BitSlice, yields MutBitProxy that can be Derefed to a bool
    fn bit_iter_mut<'short>(&'short mut self) -> MutBiter<'short,ElementType> {MutBiter::from(self)}
    ///mutably iterate over bits using a range
    fn biter_mut<'short, R:NumRangeExtract<usize> >(&'short mut self, range:R) ->  MutBiter<'short,ElementType> {
        let (start,end) = self.range_extract(range); //start end bits
        let spointer = unsafe { (self.as_mut() as *mut [ElementType] as *mut ElementType).add(Self::bits_idx(start)) };
        unsafe { MutBiter::new(spointer, Self::bits_bit(start) as u8, end-start+1) } //startptr, start bit pos, remaning bits
    }
}

impl <ElementType:BitOps,S:AsRef<[ElementType]>+?Sized > SliceBitOps<ElementType> for S {}
impl <ElementType:BitOps,S:AsMut<[ElementType]>+AsRef<[ElementType]>+?Sized > MutSliceBitOps<ElementType> for S {}
