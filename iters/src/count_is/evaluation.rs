pub(crate) struct Output<E>
where
    E: Evaluator,
{
    pub(crate) value: bool,
    pub(crate) evaluation: Option<E>,
}

pub(crate) trait Evaluator: Sized {
    fn evaluate_count(self, count: usize) -> Output<Self>;

    fn evaluate(mut self, it: &mut impl Iterator) -> bool {
        let mut count = 0;

        loop {
            let output = self.evaluate_count(count);
            if let Some(eval) = output.evaluation {
                self = eval;

                if it.next().is_some() {
                    count += 1;
                    continue;
                }
            }
            return output.value;
        }
    }
}

macro_rules! comparison_evaluator {
    ($name:ident, $op:tt; $op2:tt) => {
        #[derive(Clone, PartialEq, Eq, Debug)]
        pub(crate) struct $name(usize);

        impl $name {
            pub(crate) fn new(tested_count: usize) -> Self {
                $name(tested_count)
            }

            fn tested_count(&self) -> usize {
                self.0
            }
        }

        impl Evaluator for $name {
            fn evaluate_count(self, count: usize) -> Output<Self> {
                let value = count $op self.tested_count();
                let value_can_change = count $op2 self.tested_count();

                Output {
                    value,
                    evaluation: value_can_change.then_some(self)
                }
            }
        }
    };
}

comparison_evaluator!(Eq, == ; <=);
comparison_evaluator!(Lt, < ; <);
comparison_evaluator!(Gt, > ; <=);

#[derive(Clone, PartialEq, Eq, Debug)]
pub(crate) struct Not<E>(E);
impl<E> Not<E> {
    pub(crate) fn new(e: E) -> Self {
        Not(e)
    }
}
impl<E> Evaluator for Not<E>
where
    E: Evaluator,
{
    fn evaluate_count(self, count: usize) -> Output<Self> {
        let output = self.0.evaluate_count(count);

        let value = !output.value;
        let evaluation = output.evaluation.map(Not);

        Output { value, evaluation }
    }
}
