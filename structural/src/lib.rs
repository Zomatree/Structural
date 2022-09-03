#![feature(adt_const_params)]
#![feature(const_type_name)]
#![allow(incomplete_features)]

pub use structural_macros::{Struct, has_attrs};

pub trait Attrs {
    const ATTRS: &'static [(&'static str, &'static str)];

    fn attrs(&self) -> &[(&'static str, &'static str)] {
        Self::ATTRS
    }
}

pub trait HasAttr<const K: &'static str>: Attrs {
    type Ty;

    fn get(&self) -> &Self::Ty;
    fn set(&mut self, value: Self::Ty);
    fn take(self) -> Self::Ty;
}
