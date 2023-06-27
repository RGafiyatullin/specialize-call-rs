/// A macro producing the invocations of the specified generic function with specific types, depending on the value available in runtime.
/// 
/// Examples:
/// ```rust
/// 
/// struct A<const I: usize>();
/// struct B<const I: usize>();
/// struct C<const I: usize>();
/// 
/// #[derive(Debug)]
/// enum Select<const I: usize> {
///     A,
///     B,
///     C,
/// }
/// 
/// use core::any::type_name as tn;
/// 
/// #[test]
/// fn specialize_call_1() {
///     // there is a function `do_it` that takes one runtime argument, and one type-parameter.
///     fn do_it<T>(_arg: usize) -> &'static str {
///         tn::<T>()
///     }
/// 
///     assert_eq!(
///         specialize_call!(
///             // invoke `do_it::<?>`
///             do_it, 
///             // use `(1)` as the arguments list
///             (1), 
///             // choose the type-parameter depending on this value
///             Select::<1>::A, 
///             // the choice map
///             [
///                 // if the provided value is `Select::A`, then invoke `do_it::<A>`
///                 (Select::<1>::A => A::<1>),
///                 // if the provided value is `Select::B`, then invoke `do_it::<B>`
///                 (Select::<1>::B => B::<1>),
///             ]),
///         Some(tn::<A<1>>())
///     );
/// 
///     assert_eq!(
///         specialize_call!(do_it, (2), Select::<1>::B, [
///                 (Select::<1>::A => A::<1>),
///                 (Select::<1>::B => B::<1>),
///             ]),
///         Some(tn::<B<1>>()),
///     );
/// 
///     assert_eq!(
///         specialize_call!(do_it, (3), Select::<1>::C, [
///                 (Select::<1>::A => A::<1>),
///                 (Select::<1>::B => B::<1>),
///             ]),
///         None::<&str>,
///     );
/// }
/// ```
#[macro_export]
macro_rules! specialize_call {
    (@single_mapping,
        ( $($acc_p:pat),* ),
        ( $($acc_t:ty),* ),
        $func:tt, $args:tt, $select:tt,
        [ ($p:pat => $($t:ty),+ ) $(, $p_t:tt)* $(,)* ],
        $mappings:tt
    ) => {
        specialize_call!(@specialize_call,
            ( $($acc_p,)* $p ),
            ( $($acc_t,)* $($t),+ ),

            $func, $args, $select,
            $mappings
        );

        specialize_call!(@single_mapping,
            ( $($acc_p),* ),
            ( $($acc_t),* ),
            $func, $args, $select,
            [ $($p_t),* ],
            $mappings
        );
    };
    (@single_mapping,
        $acc_p:tt, $acc_t: tt,
        $func:tt, $args:tt, $select:tt,
        [],
        $mappings:tt
    ) => {};

    (@specialize_call,
        $acc_p:tt,
        $acc_t:tt,
        $func:tt,
        $args:tt,
        $select:tt,

        ($head:tt $(, $tail:tt)*)
    ) => {
        specialize_call!(@single_mapping,
            $acc_p, $acc_t,
            $func, $args, $select,
            $head,
            ( $($tail),* )
        );
    };
    (@specialize_call,
        $acc_p:tt,
        $acc_t:tt,
        $func:tt,
        $args:tt,
        $select:tt,

        ()
    ) => {
        specialize_call!(@maybe_invoke,
            $acc_p,
            $acc_t,
            $select,
            $func,
            $args
        );
    };

    (@maybe_invoke,
        ( $($acc_p:pat),* ),
        ( $($acc_t:ty),* ),
        $select:expr,
        $func:ident,
        ($($arg:expr),*)
    ) => {
        #[allow(unused_parens)]
        if matches!($select, ( $($acc_p),* )) { break Some($func::<$($acc_t),*>( $($arg),* )); }
    };

    ($func:ident, ($($arg:expr),* $(,)*), $select:expr, $($mapping:tt),+ $(,)*) => {
        loop {
            specialize_call!(@specialize_call,
                (), (),
                $func,
                ($($arg),*),
                $select,
                ( $($mapping),* )
            );
            break None
        }
    };
}
