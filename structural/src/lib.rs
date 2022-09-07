#![feature(adt_const_params)]
#![feature(const_type_name)]
#![allow(incomplete_features)]

pub use structural_macros::{Struct, has_attrs};

pub trait Attrs: Sized {
    const ATTRS: &'static [(&'static str, &'static str)];

    fn attrs(&self) -> &'static [(&'static str, &'static str)] {
        Self::ATTRS
    }

    fn get<const K: &'static str>(&self) -> &<Self as HasAttr<K>>::Ty
    where
        Self: HasAttr<K>,
    {
        HasAttr::<K>::get_attr(self)
    }

    fn set<const K: &'static str>(&mut self, value: <Self as HasAttr<K>>::Ty)
    where
        Self: HasAttr<K>,
    {
        HasAttr::<K>::set_attr(self, value)
    }

    fn take<const K: &'static str>(self) -> <Self as HasAttr<K>>::Ty
    where
        Self: HasAttr<K>,
    {
        HasAttr::<K>::take_attr(self)
    }
}

pub trait HasAttr<const K: &'static str>: Attrs {
    type Ty;

    fn get_attr(&self) -> &Self::Ty;
    fn set_attr(&mut self, value: Self::Ty);
    fn take_attr(self) -> Self::Ty;
}
