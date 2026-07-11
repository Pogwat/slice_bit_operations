#![no_std]
#[doc = include_str!("../README.md")]
pub use bit_operations::{BitOps,MutBitProxy};
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
}
/// Methods for Mutable BitSlice
pub trait MutSliceBitOps<ElementType:BitOps>:SliceBitOps<ElementType>+AsMut<[ElementType]> {
    /// Set a bit by global index in a BitSlice
    fn bit_set(&mut self,bit:usize, val:bool) {self.as_mut()[Self::bits_idx(bit)].set_bit(Self::bits_bit(bit),val)}
    /// Get Mutable refrence to Bit (porxy struct: MutBitProxy), REF MUST BE DROPPED FOR BIT TO UPDATE. DROP UPDATES!!!
    fn bit_get_mut<'short>(&'short mut self, bit:usize) -> MutBitProxy<'short,ElementType> {self.as_mut()[Self::bits_idx(bit)].mut_bit(Self::bits_bit(bit))}
    /// Mutably Iterate over a BitSlice, yields MutBitProxy that can be Derefed to a bool
    fn bit_iter_mut<'short>(&'short mut self) -> MutBiter<'short,ElementType> {MutBiter::from(self)}
}

impl <ElementType:BitOps,S:AsRef<[ElementType]> > SliceBitOps<ElementType> for S {}
impl <ElementType:BitOps,S:AsMut<[ElementType]>+AsRef<[ElementType]> > MutSliceBitOps<ElementType> for S {}
