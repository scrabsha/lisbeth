/// Allows to append an element at the end of a tuple.
///
/// The generic type parameter `T` represents the type to be appended. It may
/// be removed in the future, when GAT reach stable rust.
pub trait TupleAppend<T> {
    /// The type that is returned.
    type Appended;

    /// The appending function.
    fn append(self, other: T) -> Self::Appended;
}

macro_rules! impl_tuple_append {
    (
        ( $( $left:ident ),* $(,)? ) + $right:ident $(,)?
    ) => {
        impl<$( $left, )* $right> TupleAppend<$right> for ( $( $left, )* ) {
            type Appended = ( $( $left, )* $right, );

            #[allow(non_snake_case)]
            fn append(self, other: $right) -> Self::Appended {
                let ( $( $left, )* ) = self;

                ( $( $left, )* other, )
            }
        }
    };
}

impl_tuple_append! { () + A }
impl_tuple_append! { (A,) + B }
impl_tuple_append! { (A, B) + C }
impl_tuple_append! { (A, B, C) + D }
impl_tuple_append! { (A, B, C, D) + E }
impl_tuple_append! { (A, B, C, D, E) + F }
impl_tuple_append! { (A, B, C, D, E, F) + G }
impl_tuple_append! { (A, B, C, D, E, F, G) + H }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn append_empty_tuple() {
        let t: ((), (), (), (), ()) = ().append(()).append(()).append(()).append(()).append(());

        // Note: there is acutally nothing to be checked, but let's assume so.
        assert_eq!(t, ((), (), (), (), ()));
    }

    #[test]
    fn append_up_to_eight() {
        let t: (u8, u8, u8, u8, u8, u8, u8, u8) =
            ().append(1)
                .append(2)
                .append(3)
                .append(4)
                .append(5)
                .append(6)
                .append(7)
                .append(8);

        assert_eq!(t, (1, 2, 3, 4, 5, 6, 7, 8));
    }
}
