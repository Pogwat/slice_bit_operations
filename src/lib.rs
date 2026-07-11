#![no_std]
#[doc = include_str!("../README.md")]
pub use bit_operations::{BitOps,MutBitProxy, NumRangeExtract};
pub use biter::{Biter,MutBiter};
/// Methods for Immutable BitSlice
pub trait SliceBitOps<ElementType:BitOps>:AsRef<[ElementType]> {
    ///Number of bits in BitSlice
    fn bit_len(&self) -> usize {self.as_ref().len()*ElementType::TYPE_BITS}
    /// Global Bit Index to bit positon in a element
    fn bits_idx(bit:usize) -> usize {bit/ElementType::TYPE_BITS}
    /// Global Bit Index to a element index
    fn bits_bit(bit:usize) -> usize {bit%ElementType::TYPE_BITS}
    /// Get bit by global index
    fn bit_get(&self,bit:usize) -> bool {self.as_ref()[Self::bits_idx(bit)].get_bit(Self::bits_bit(bit))}
    /// Iterate over a BitSlice, yields bools
    fn bit_iter<'short>(&'short self) -> Biter<'short,ElementType> {Biter::from(self)}
    ///start and end bits for a bit range
    fn range_extract<R:NumRangeExtract<usize> >(&self,range:R) -> (usize,usize) { ( range.start().unwrap_or(0), range.end().unwrap_or(self.bit_len()-1) ) }
    ///iterate over bits using a range
    fn biter<'short, R:NumRangeExtract<usize> >(&'short self, range:R) -> Biter<'short,ElementType> {
        let (start,end) = self.range_extract(range); //start end bits
        let spointer = unsafe { (self.as_ref() as *const [ElementType] as *const ElementType).add(Self::bits_idx(start)) };
        unsafe { Biter::new(spointer, Self::bits_bit(start) as u8, end-start+1) } //startptr, start bit pos, remaning bits
    }
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

impl <ElementType:BitOps,S:AsRef<[ElementType]> > SliceBitOps<ElementType> for S {}
impl <ElementType:BitOps,S:AsMut<[ElementType]>+AsRef<[ElementType]> > MutSliceBitOps<ElementType> for S {}
