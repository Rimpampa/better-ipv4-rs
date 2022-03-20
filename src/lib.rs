use std::{marker::PhantomData, net::Ipv4Addr};

mod english;
mod italian;
mod tuple;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tuple() {
        #[rustfmt::skip]
        assert_eq!(Ipv4Addr::new(127, 0, 0, 1), *tuple::IPV4.127.0.0.1);
        assert_eq!(
            Ipv4Addr::new(127, 0, 0, 1),
            *english::IPV4.one_hundred_twenty_seven.zero.zero.one
        );
        assert_eq!(
            Ipv4Addr::new(127, 0, 0, 1),
            *italian::IPV4.centoventisette.zero.zero.uno
        );
    }
}

/// Types that are [`Octet`]s can represent in some way all the 256 possible values of an 8bit number
pub trait Octet {
    const VALUES: Self;
}

pub struct Value<O: Octet, T, const V: u8> {
    _o: PhantomData<O>,
    _t: PhantomData<T>,
}

pub struct Ipv4xxxx;
pub struct Ipv4axxx<const A: u8>;
pub struct Ipv4abxx<const A: u8, const B: u8>;
pub struct Ipv4abcx<const A: u8, const B: u8, const C: u8>;
pub struct Ipv4abcd<const A: u8, const B: u8, const C: u8, const D: u8>;

impl<const A: u8, const B: u8, const C: u8, const D: u8> Ipv4abcd<A, B, C, D> {
    const ADDR: Ipv4Addr = Ipv4Addr::new(A, B, C, D);
}

#[macro_export]
macro_rules! impls_for_octet {
    ($octet_type:ident) => {
        impl<Any, const A: u8> std::ops::Deref
            for $crate::Value<$octet_type<Any>, $crate::Ipv4xxxx, A>
        {
            type Target = $octet_type<$crate::Ipv4axxx<A>>;

            fn deref(&self) -> &Self::Target {
                &<Self::Target as $crate::Octet>::VALUES
            }
        }

        impl<Any, const A: u8, const B: u8> std::ops::Deref
            for $crate::Value<$octet_type<Any>, $crate::Ipv4axxx<A>, B>
        {
            type Target = $octet_type<$crate::Ipv4abxx<A, B>>;

            fn deref(&self) -> &Self::Target {
                &<Self::Target as $crate::Octet>::VALUES
            }
        }

        impl<Any, const A: u8, const B: u8, const C: u8> std::ops::Deref
            for $crate::Value<$octet_type<Any>, $crate::Ipv4abxx<A, B>, C>
        {
            type Target = $octet_type<$crate::Ipv4abcx<A, B, C>>;

            fn deref(&self) -> &Self::Target {
                &<Self::Target as $crate::Octet>::VALUES
            }
        }

        impl<Any, const A: u8, const B: u8, const C: u8, const D: u8> std::ops::Deref
            for $crate::Value<$octet_type<Any>, $crate::Ipv4abcx<A, B, C>, D>
        {
            type Target = std::net::Ipv4Addr;

            fn deref(&self) -> &Self::Target {
                &$crate::Ipv4abcd::<A, B, C, D>::ADDR
            }
        }
    };
}
