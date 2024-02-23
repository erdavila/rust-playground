use crate::{EmulatedFn, EmulatedFnMut, EmulatedFnOnce};

/*
        let composed = f.compose(g);

    If `f` and `g` are `EmulatedFn`, then `composed` is `EmulatedFn`;
    otherwise, if `f` and `g` are `EmulatedFnMut`, then `composed` is `EmulatedFnMut`;
    otherwise, `composed` is `EmulatedFnOnce`.

                       |                         g                        |
                       +----------------+----------------+----------------+
                       | EmulatedFn     | EmulatedFnMut  | EmulatedFnOnce |
    --+----------------+----------------+----------------+----------------+
      | EmulatedFn     | EmulatedFn     | EmulatedFnMut  | EmulatedFnOnce |
    f | EmulatedFnMut  | EmulatedFnMut  | EmulatedFnMut  | EmulatedFnOnce |
      | EmulatedFnOnce | EmulatedFnOnce | EmulatedFnOnce | EmulatedFnOnce |
*/

trait Compose<O>: Sized {
    fn compose<Args, First>(self, first: First) -> Composed<First, Self>
    where
        First: EmulatedFnOnce<Args, Output = O>;
}

impl<Second, O> Compose<O> for Second
where
    Second: EmulatedFnOnce<(O,)>,
{
    fn compose<Args, First>(self, first: First) -> Composed<First, Self>
    where
        First: EmulatedFnOnce<Args, Output = O>,
    {
        Composed {
            first,
            second: self,
        }
    }
}

pub struct Composed<First, Second> {
    first: First,
    second: Second,
}

impl<Args, First, Second> EmulatedFnOnce<Args> for Composed<First, Second>
where
    First: EmulatedFnOnce<Args>,
    Second: EmulatedFnOnce<(First::Output,)>,
{
    type Output = Second::Output;

    fn call_once(self, args: Args) -> Self::Output {
        let result = self.first.call_once(args);
        self.second.call_once((result,))
    }
}

impl<Args, First, Second> EmulatedFnMut<Args> for Composed<First, Second>
where
    First: EmulatedFnMut<Args>,
    Second: EmulatedFnMut<(First::Output,)>,
{
    fn call_mut(&mut self, args: Args) -> Self::Output {
        let result = self.first.call_mut(args);
        self.second.call_mut((result,))
    }
}

impl<Args, First, Second> EmulatedFn<Args> for Composed<First, Second>
where
    First: EmulatedFn<Args>,
    Second: EmulatedFn<(First::Output,)>,
{
    fn call(&self, args: Args) -> Self::Output {
        let result = self.first.call(args);
        self.second.call((result,))
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        composition::Compose, emulated_fn, emulated_fn_mut, emulated_fn_once, EmulatedFn,
        EmulatedFnMut, EmulatedFnOnce,
    };

    #[test]
    fn test_emulated_fn_once() {
        let f = emulated_fn_once! {
            captures: {},
            signature: (String,) => usize,
            |(), (s,)| {
                s.len()
            }
        };

        let g = emulated_fn_once! {
            captures: {},
            signature: (char,) => String,
            |(), (c,)| {
                String::from(c)
            }
        };

        let composed = f.compose(g);

        let result = composed.call_once(('@',));

        assert_eq!(result, 1);
    }

    #[test]
    fn test_emulated_fn_mut() {
        let f = emulated_fn_mut! {
            captures: {},
            signature: (String,) => usize,
            |(), (s,)| {
                s.len()
            }
        };

        let g = emulated_fn_mut! {
            captures: {},
            signature: (char,) => String,
            |(), (c,)| {
                String::from(c)
            }
        };

        let mut composed = f.compose(g);

        let result = composed.call_mut(('@',));

        assert_eq!(result, 1);
    }

    #[test]
    fn test_emulated_fn() {
        let f = emulated_fn! {
            captures: {},
            signature: (String,) => usize,
            |(), (s,)| {
                s.len()
            }
        };

        let g = emulated_fn! {
            captures: {},
            signature: (char,) => String,
            |(), (c,)| {
                String::from(c)
            }
        };

        let composed = f.compose(g);

        let result = composed.call(('@',));

        assert_eq!(result, 1);
    }
}
