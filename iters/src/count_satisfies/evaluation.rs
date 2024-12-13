mod private {
    pub struct Output<E>
    where
        E: Evaluator,
    {
        pub(crate) value: bool,
        pub(crate) evaluation: Option<E>,
    }

    pub trait Evaluator: Sized {
        fn evaluate_count(self, count: usize) -> Output<Self>;
    }
}

use private::Output;

pub trait Evaluator: private::Evaluator {
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

    fn not(self) -> impl Evaluator {
        Not::new(self)
    }

    fn or(self, other: impl Evaluator) -> impl Evaluator {
        Or::new(self, other)
    }

    fn and(self, other: impl Evaluator) -> impl Evaluator {
        And::new(self, other)
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

        impl private::Evaluator for $name {
            fn evaluate_count(self, count: usize) -> Output<Self> {
                let value = count $op self.tested_count();
                let value_can_change = count $op2 self.tested_count();

                Output {
                    value,
                    evaluation: value_can_change.then_some(self)
                }
            }
        }

        impl Evaluator for $name {}
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
impl<E> private::Evaluator for Not<E>
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
impl<E> Evaluator for Not<E> where E: Evaluator {}

macro_rules! binary_logical_evaluator {
    ($name:ident, $op:tt; $neutral:literal) => {
        #[derive(Clone, PartialEq, Eq, Debug)]
        pub(crate) enum $name<E1, E2> {
            Both(E1, E2),
            Eval1(E1),
            Eval2(E2),
        }

        impl<E1, E2> $name<E1, E2> {
            pub(crate) fn new(e1: E1, e2: E2) -> Self {
                $name::Both(e1, e2)
            }
        }

        impl<E1, E2> private::Evaluator for $name<E1, E2>
        where
            E1: Evaluator,
            E2: Evaluator,
        {
            fn evaluate_count(self, count: usize) -> Output<Self> {
                match self {
                    $name::Both(e1, e2) => {
                        let output1 = e1.evaluate_count(count);
                        let output2 = e2.evaluate_count(count);

                        let value = output1.value $op output2.value;
                        let evaluation = match (output1.evaluation, output2.evaluation) {
                            (None, None) => None,
                            (None, Some(e2)) => (output1.value == $neutral).then_some($name::Eval2(e2)),
                            (Some(e1), None) => (output2.value == $neutral).then_some($name::Eval1(e1)),
                            (Some(e1), Some(e2)) => Some($name::Both(e1, e2)),
                        };

                        Output { value, evaluation }
                    }
                    $name::Eval1(e1) => {
                        let output1 = e1.evaluate_count(count);
                        Output {
                            value: output1.value,
                            evaluation: output1.evaluation.map($name::Eval1),
                        }
                    }
                    $name::Eval2(e2) => {
                        let output2 = e2.evaluate_count(count);
                        Output {
                            value: output2.value,
                            evaluation: output2.evaluation.map($name::Eval2),
                        }
                    }
                }
            }
        }

        impl<E1, E2> Evaluator for $name<E1, E2>
        where
            E1: Evaluator,
            E2: Evaluator,
        {}
    };
}

binary_logical_evaluator!(Or, || ; false);
binary_logical_evaluator!(And, && ; true);
