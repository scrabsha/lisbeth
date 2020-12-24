macro_rules! declare_map_n {
    (
        #[doc = $ordinal:literal]
        $name:ident::$fn_name:ident
    ) => {
        #[doc = "Allows to map the "]
        #[doc = $ordinal]
        #[doc = " element of a tuple to another type."]
        pub trait $name<T, U> {
            type Output;
            fn $fn_name<Func>(self, f: Func) -> Self::Output
            where
                Func: FnOnce(T) -> U;
        }
    };
}

declare_map_n! {
    /// first
    TupleMap1::map_1
}
declare_map_n! {
    /// second
    TupleMap2::map_2
}
declare_map_n! {
    /// third
    TupleMap3::map_3
}
declare_map_n! {
    /// fourth
    TupleMap4::map_4
}
declare_map_n! {
    /// difth
    TupleMap5::map_5
}
declare_map_n! {
    /// sixth
    TupleMap6::map_6
}
declare_map_n! {
    /// seventh
    TupleMap7::map_7
}
declare_map_n! {
    /// eighth
    TupleMap8::map_8
}

macro_rules! impl_map_n {
    (
        $trait:ident::$fn:ident for ( $( $before:ident, )* _ $( , $after:ident )* $(,)? ) $(,)?
    ) => {
        impl<$( $before, )* $( $after, )* T, U> $trait<T, U> for ( $( $before, )* T, $( $after, )* ) {
            type Output = ( $( $before, )* U, $( $after, )* );

            #[allow(non_snake_case)]
            fn $fn<Func>(self, f: Func) -> Self::Output
            where
                Func: FnOnce(T) -> U,
            {
                let ( $( $before, )* t, $( $after, )* ) = self;
                let u = f(t);
                ( $( $before, )* u, $( $after, )* )
            }
        }
    };
}

impl_map_n! { TupleMap1::map_1 for (_,) }
impl_map_n! { TupleMap1::map_1 for (_, B) }
impl_map_n! { TupleMap1::map_1 for (_, B, C) }
impl_map_n! { TupleMap1::map_1 for (_, B, C, D) }
impl_map_n! { TupleMap1::map_1 for (_, B, C, D, E) }
impl_map_n! { TupleMap1::map_1 for (_, B, C, D, E, F) }
impl_map_n! { TupleMap1::map_1 for (_, B, C, D, E, F, G) }
impl_map_n! { TupleMap1::map_1 for (_, B, C, D, E, F, G, H) }

impl_map_n! { TupleMap2::map_2 for (A, _) }
impl_map_n! { TupleMap2::map_2 for (A, _, C) }
impl_map_n! { TupleMap2::map_2 for (A, _, C, D) }
impl_map_n! { TupleMap2::map_2 for (A, _, C, D, E) }
impl_map_n! { TupleMap2::map_2 for (A, _, C, D, E, F) }
impl_map_n! { TupleMap2::map_2 for (A, _, C, D, E, F, G) }
impl_map_n! { TupleMap2::map_2 for (A, _, C, D, E, F, G, H) }

impl_map_n! { TupleMap3::map_3 for (A, B, _) }
impl_map_n! { TupleMap3::map_3 for (A, B, _, D) }
impl_map_n! { TupleMap3::map_3 for (A, B, _, D, E) }
impl_map_n! { TupleMap3::map_3 for (A, B, _, D, E, F) }
impl_map_n! { TupleMap3::map_3 for (A, B, _, D, E, F, G) }
impl_map_n! { TupleMap3::map_3 for (A, B, _, D, E, F, G, H) }

impl_map_n! { TupleMap4::map_4 for (A, B, C, _) }
impl_map_n! { TupleMap4::map_4 for (A, B, C, _, E) }
impl_map_n! { TupleMap4::map_4 for (A, B, C, _, E, F) }
impl_map_n! { TupleMap4::map_4 for (A, B, C, _, E, F, G) }
impl_map_n! { TupleMap4::map_4 for (A, B, C, _, E, F, G, H) }

impl_map_n! { TupleMap5::map_5 for (A, B, C, D, _) }
impl_map_n! { TupleMap5::map_5 for (A, B, C, D, _, F) }
impl_map_n! { TupleMap5::map_5 for (A, B, C, D, _, F, G) }
impl_map_n! { TupleMap5::map_5 for (A, B, C, D, _, F, G, H) }

impl_map_n! { TupleMap6::map_6 for (A, B, C, D, E, _) }
impl_map_n! { TupleMap6::map_6 for (A, B, C, D, E, _, G) }
impl_map_n! { TupleMap6::map_6 for (A, B, C, D, E, _, G, H) }

impl_map_n! { TupleMap7::map_7 for (A, B, C, D, E, F, _) }
impl_map_n! { TupleMap7::map_7 for (A, B, C, D, E, F, _, H) }

impl_map_n! { TupleMap8::map_8 for (A, B, C, D, E, F, G, _) }
