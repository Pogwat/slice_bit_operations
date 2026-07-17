#![no_std]
#[doc = include_str!("../README.md")]
pub use bit_operations::{BitOps,MutBitProxy, NumRangeExtract};
pub use biter::{Biter,MutBiter};

pub struct BitRanger {
    start_bit:usize,
    end_bit:usize,
    starts_idx:usize,
    ends_idx:usize,
    starts_bit:u8,
    ends_bit:u8
}

macro_rules! counters {
    (   $(#[$meta:meta])* $fn_name:ident, $bit_method:ident) => {
        $(#[$meta])*
        #[inline]
        fn $fn_name<R:bit_operations::NumRangeExtract<usize> >(&self,range:R) -> usize {
           let ranger = self.new_ranger(range);
            if ranger.starts_idx==ranger.ends_idx { unsafe {
                self.as_ref().get_unchecked(ranger.ends_idx).$bit_method(&(ranger.starts_bit..=ranger.ends_bit)) as usize
            } } else { unsafe {
                self.as_ref().get_unchecked(ranger.starts_idx).$bit_method(&(ranger.starts_bit..)) as usize +
                self.as_ref().get_unchecked(ranger.ends_idx).$bit_method(&(..=ranger.ends_bit)) as usize +
                self.as_ref().get_unchecked(ranger.starts_idx..ranger.ends_idx).iter().skip(1).map(|num| num.$bit_method(&(0..)) as usize).sum::<usize>()
            } }
        }
    }
}

macro_rules! first {
    ($(#[$meta:meta])* $fn_name:ident, $fn_calculator:ident) => {
        $(#[$meta])*
        #[inline]
        fn $fn_name<R:bit_operations::NumRangeExtract<usize>+core::slice::SliceIndex<[ElementType], Output = [ElementType]> >(&self, range:R) -> Option<usize> {
            let ranger = self.new_ranger(range);
            if ranger.starts_idx == ranger.ends_idx { unsafe {
                self.as_ref().get_unchecked(ranger.starts_idx).$fn_calculator(&(ranger.starts_bit..=ranger.ends_bit)).map(|bit| bit as usize + ranger.start_bit - ranger.starts_bit as usize)
            } } else { unsafe {
                self.as_ref().get_unchecked(ranger.starts_idx).$fn_calculator(&(ranger.starts_bit..)).map(|bitpos| bitpos as usize + ranger.start_bit - ranger.starts_bit as usize)
                    .or_else(||
                self.as_ref().get_unchecked(ranger.starts_idx..ranger.ends_idx).iter().skip(1)
                .find_map(|num| num.$fn_calculator(&(0..))
                    .map(|fz| (num as *const ElementType).offset_from(self.as_ref().as_ptr())as usize*ElementType::BITS as usize  +fz as usize )
                )
                    ).or_else(||
                self.as_ref().get_unchecked(ranger.ends_idx).$fn_calculator(&(..=ranger.ends_bit)).map(|bitpos| bitpos as usize + ranger.end_bit - ranger.ends_bit as usize)
                )
            } }
        }
    }
}

macro_rules! last {
    ($(#[$meta:meta])* $fn_name:ident, $fn_calculator:ident) => {
        $(#[$meta])*
        #[inline]
        fn $fn_name<R:bit_operations::NumRangeExtract<usize>+core::slice::SliceIndex<[ElementType], Output = [ElementType]> >(&self, range:R) -> Option<usize> {
            let ranger = self.new_ranger(range);
            if ranger.starts_idx == ranger.ends_idx { unsafe {
                self.as_ref().get_unchecked(ranger.starts_idx).$fn_calculator(&(ranger.starts_bit..=ranger.ends_bit)).map(|bit| bit as usize + ranger.start_bit - ranger.starts_bit as usize)
            } } else { unsafe {
                self.as_ref().get_unchecked(ranger.ends_idx).$fn_calculator(&(..=ranger.ends_bit)).map(|bitpos| bitpos as usize + ranger.end_bit - ranger.ends_bit as usize)
                    .or_else(||
                        self.as_ref().get_unchecked(ranger.starts_idx..ranger.ends_idx).iter().skip(1).rev()
                        .find_map(|num| num.$fn_calculator(&(0..))
                            .map(|fz| (num as *const ElementType).offset_from(self.as_ref().as_ptr())as usize*ElementType::BITS as usize  +fz as usize  )
                        )
                    ).or_else(||
                self.as_ref().get_unchecked(ranger.starts_idx).$fn_calculator(&(ranger.starts_bit..)).map(|bitpos| bitpos as usize + ranger.start_bit - ranger.starts_bit as usize)
                )
            } }
        }
    }
}

use core::slice::SliceIndex;
/// Methods for Immutable BitSlice
pub trait SliceBitOps<ElementType:BitOps+>:AsRef<[ElementType]> {
    ///Number of bits in BitSlice
    #[inline]
    fn bit_len(&self) -> usize {self.as_ref().len()*ElementType::BITS as usize}
    /// Global Bit Index to bit positon in a element
    #[inline]
    fn bits_idx(bit:usize) -> usize {bit/ElementType::BITS as usize}
    /// Global Bit Index to a element index
    #[inline]
    fn bits_bit(bit:usize) -> u8 {(bit%ElementType::BITS as usize) as u8 }
    /// Get bit by global index
    #[inline]
    fn bit_get(&self,bit:usize) -> bool {self.as_ref()[Self::bits_idx(bit)].get_bit(Self::bits_bit(bit))}
    /// Iterate over a BitSlice, yields bools
    #[inline]
    fn bit_iter<'short>(&'short self) -> Biter<'short,ElementType> {Biter::from(self)}
    ///start and end bits for a bit range
    #[inline]
    fn range_extract<R:NumRangeExtract<usize> >(&self,range:R) -> (usize,usize) { ( range.start().unwrap_or(0).min(self.bit_len()-1), range.end().unwrap_or(self.bit_len()-1).min(self.bit_len()-1) ) }
    ///iterate over bits using a range
    fn new_ranger<R:NumRangeExtract<usize> >(&self,range:R) -> BitRanger {
        let (start_bit,end_bit) = self.range_extract(range);
        let (starts_idx,starts_bit):(usize,u8) = (Self::bits_idx(start_bit), Self::bits_bit(start_bit));
        let (ends_idx,ends_bit):(usize,u8) = (Self::bits_idx(end_bit), Self::bits_bit(end_bit));
        BitRanger { start_bit,end_bit,starts_idx,ends_idx,starts_bit,ends_bit }
    }

    #[inline]
    fn biter<'short, R:NumRangeExtract<usize> >(&'short self, range:R) -> Biter<'short,ElementType> {
        let (start,end) = self.range_extract(range); //start end bits
        let spointer = unsafe { (self.as_ref() as *const [ElementType] as *const ElementType).add(Self::bits_idx(start)) };
        unsafe { Biter::new(spointer, Self::bits_bit(start), end-start+1) } //startptr, start bit pos, remaning bits
    }
    /// first ones index
    #[inline]
    fn first_one_idx<R:SliceIndex<[ElementType], Output = [ElementType]> >(&self,idx_range:R) -> Option<usize>{
        self.as_ref()[idx_range].iter().position(|num| *num!=ElementType::ZERO)
    }
    /// first zeros index
    #[inline]
    fn first_zero_idx<R:SliceIndex<[ElementType], Output = [ElementType]> >(&self,idx_range:R) -> Option<usize>{
        self.as_ref()[idx_range].iter().position(|num| *num!=!ElementType::ZERO)
    }

    counters!(
        /// Count zeros from bit range
        ctz,ctz);
    counters!(
        /// Count ones from bit range
        popcnt,popcnt);

    first!(
        /// find first zero from bit range
        first_zero,first_zero);
    first!(
        /// find first one from bit range
        first_one,first_one);
    last!(
        /// find last one from bit range
        last_one,last_one);
    last!(
        /// find last zero from bit range
        last_zero,last_zero);

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
