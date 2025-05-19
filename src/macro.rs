/// Creates an asynchronous function that takes a closure with parameters and executes it asynchronously.
///
/// This macro generates an asynchronous closure that captures external variables by cloning them, and
/// allows the closure to be invoked with the specified parameters. The body of the closure is executed
/// inside an asynchronous block.
///
/// # Parameters
/// - `$($var:ident),*` : A list of variables from the outer scope to be captured and cloned inside the closure.
/// - `$($closure_param:ident),*` : A list of parameters for the closure. These parameters are used when calling the closure.
/// - `{ $($closure_body:tt)* }` : The body of the closure that is executed asynchronously.
///
/// # Returns
/// A closure that takes the specified parameters and returns a `Pin<Box<dyn Future<Output = ()> + Send>>`
/// which is an asynchronous task that can be awaited.
///
/// This macro allows you to encapsulate external state (captured via cloning) and run an asynchronous
/// task with a specified closure.
#[macro_export]
macro_rules! future_fn {
    ($($var:ident),*, { $($closure_body:tt)* }) => {
        || {
            #[allow(unused_parens)]
            let ($($var),*) = ($($var.clone()),*);
            async move {
                $($closure_body)*
            }
        }
    };

    ($($var:ident),*, |$( $closure_param:ident $(: $closure_param_ty:ty)? ),*| { $($closure_body:tt)* }) => {
        {
            #[allow(unused_parens)]
            let ($($var),*) = ($($var.clone()),*);
            move |$( $closure_param $(: $closure_param_ty)? ),*| {
                let ($($var),*) = ($($var.clone()),*);
                async move {
                    $($closure_body)*
                }
            }
        }
    };
}
