pub mod composition;

pub trait EmulatedFnOnce<Args> {
    type Output;

    fn call_once(self, args: Args) -> Self::Output;
}

pub trait EmulatedFnMut<Args>: EmulatedFnOnce<Args> {
    fn call_mut(&mut self, args: Args) -> Self::Output;
}

pub trait EmulatedFn<Args>: EmulatedFnMut<Args> {
    fn call(&self, args: Args) -> Self::Output;
}

#[macro_export]
macro_rules! emulated_fn_once {
    (
        captures: {
            $( [ $($capture:tt)* ] ),*
            $(,)?
        },
        signature: ( $($arg:ty),* $(,)? ) => $out:ty,
        $body:expr
    ) => {
        {
            #[allow(non_camel_case_types)]
            struct impl_EmulatedFnOnce<'lt> {
                captures: ( $( $crate::__emulated_closures__fn_once_capture_type!( $($capture)* ) , )* ),
                _phantom: std::marker::PhantomData<&'lt ()>,
            }

            impl<'lt> $crate::EmulatedFnOnce<( $( $arg , )* )> for impl_EmulatedFnOnce<'lt> {
                type Output = $out;

                fn call_once(self, args: ( $( $arg , )* )) -> Self::Output {
                    let body: fn(( $( $crate::__emulated_closures__fn_once_capture_type!( $($capture)* ) , )* ), ( $( $arg , )* )) -> $out = $body;
                    body(self.captures, args)
                }
            }

            impl_EmulatedFnOnce {
                captures: ( $( $crate::__emulated_closures__capture_expr!( $($capture)* ) , )* ),
                _phantom: std::marker::PhantomData,
            }
        }
    };
}

#[macro_export]
macro_rules! emulated_fn_mut {
    (
        captures: {
            $( [ $($capture:tt)* ] ),*
            $(,)?
        },
        signature: ( $($arg:ty),* $(,)? ) => $out:ty,
        $body:expr
    ) => {
        {
            #[allow(non_camel_case_types)]
            struct impl_EmulatedFnMut<'lt> {
                captures: ( $( $crate::__emulated_closures__fn_mut_capture_type!( $($capture)* ) , )* ),
                _phantom: std::marker::PhantomData<&'lt ()>,
            }

            impl<'lt> $crate::EmulatedFnMut<( $( $arg , )* )> for impl_EmulatedFnMut<'lt> {
                fn call_mut(&mut self, args: ( $( $arg , )* )) -> Self::Output {
                    let body: fn(&mut ( $( $crate::__emulated_closures__fn_mut_capture_type!( $($capture)* ) , )* ), ( $( $arg , )* )) -> $out = $body;
                    body(&mut self.captures, args)
                }
            }

            impl<'lt> $crate::EmulatedFnOnce<( $( $arg , )* )> for impl_EmulatedFnMut<'lt> {
                type Output = $out;

                fn call_once(mut self, args: ( $( $arg , )* )) -> Self::Output {
                    $crate::EmulatedFnMut::call_mut(&mut self, args)
                }
            }

            impl_EmulatedFnMut {
                captures: ( $( $crate::__emulated_closures__capture_expr!( $($capture)* ) , )* ),
                _phantom: std::marker::PhantomData,
            }
        }
    };
}

#[macro_export]
macro_rules! emulated_fn {
    (
        captures: {
            $( [ $($capture:tt)* ] ),*
            $(,)?
        },
        signature: ( $($arg:ty),* $(,)? ) => $out:ty,
        $body:expr
    ) => {
        {
            #[allow(non_camel_case_types)]
            struct impl_EmulatedFn<'lt> {
                captures: ( $( $crate::__emulated_closures__fn_capture_type!( $($capture)* ) , )* ),
                _phantom: std::marker::PhantomData<&'lt ()>,
            }

            impl<'lt> $crate::EmulatedFn<( $( $arg , )* )> for impl_EmulatedFn<'lt> {
                fn call(&self, args: ( $( $arg , )* )) -> Self::Output {
                    let body: fn(&( $( $crate::__emulated_closures__fn_capture_type!( $($capture)* ) , )* ), ( $( $arg , )* )) -> $out = $body;
                    body(&self.captures, args)
                }
            }

            impl<'lt> $crate::EmulatedFnMut<( $( $arg , )* )> for impl_EmulatedFn<'lt> {
                fn call_mut(&mut self, args: ( $( $arg , )* )) -> Self::Output {
                    $crate::EmulatedFn::call(self, args)
                }
            }

            impl<'lt> $crate::EmulatedFnOnce<( $( $arg , )* )> for impl_EmulatedFn<'lt> {
                type Output = $out;

                fn call_once(mut self, args: ( $( $arg , )* )) -> Self::Output {
                    $crate::EmulatedFnMut::call_mut(&mut self, args)
                }
            }

            impl_EmulatedFn {
                captures: ( $( $crate::__emulated_closures__capture_expr!( $($capture)* ) , )* ),
                _phantom: std::marker::PhantomData,
            }
        }
    };
}

#[macro_export]
macro_rules! __emulated_closures__fn_once_capture_type {
    ($expr:expr => &mut $ty:ty) => { &'lt mut $ty };
    ($expr:expr => & $ty:ty) => { &'lt $ty };
    ($expr:expr => move $ty:ty) => { $ty };
    ($expr:expr => $ty:ty) => { $ty };
}

#[macro_export]
macro_rules! __emulated_closures__fn_mut_capture_type {
    ($expr:expr => &mut $ty:ty) => { &'lt mut $ty };
    ($expr:expr => & $ty:ty) => { &'lt $ty };
    ($expr:expr => move $ty:ty) => { $ty };
}

#[macro_export]
macro_rules! __emulated_closures__fn_capture_type {
    ($expr:expr => & $ty:ty) => { &'lt $ty };
    ($expr:expr => move $ty:ty) => { $ty };
}

#[macro_export]
macro_rules! __emulated_closures__capture_expr {
    ($expr:expr => $($tt:tt)*) => {
        $expr
    };
}
