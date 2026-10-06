pub trait Input {}
impl Input for f64 {}
impl<T, const N: usize> Input for [T; N] {}
impl<T> Input for (T,) {}
impl<T, U> Input for (T, U) {}
impl<T, U, V> Input for (T, U, V) {}

pub trait UniformInput: Input {
    type VariableType;
}
impl UniformInput for f64 {
    type VariableType = f64;
}
impl<T, const N: usize> UniformInput for [T; N] {
    type VariableType = T;
}
impl<T> UniformInput for (T,) {
    type VariableType = T;
}
impl<T> UniformInput for (T, T) {
    type VariableType = T;
}
impl<T> UniformInput for (T, T, T) {
    type VariableType = T;
}

pub trait SelectInput<const V: usize>: Input {
    type VariableOutput;
    fn select_input(&self) -> Self::VariableOutput;
}
impl<T> SelectInput<0> for T
where
    T: Clone + UniformInput<VariableType = T>,
{
    type VariableOutput = T;

    fn select_input(&self) -> Self::VariableOutput {
        self.clone()
    }
}
impl<T, const N: usize, const V: usize> SelectInput<V> for [T; N]
where
    T: Clone,
{
    type VariableOutput = T;

    fn select_input(&self) -> Self::VariableOutput {
        self[V].clone()
    }
}
impl<T> SelectInput<0> for (T,)
where
    T: Clone,
{
    type VariableOutput = T;

    fn select_input(&self) -> Self::VariableOutput {
        self.0.clone()
    }
}
impl<T, U> SelectInput<0> for (T, U)
where
    T: Clone,
{
    type VariableOutput = T;

    fn select_input(&self) -> Self::VariableOutput {
        self.0.clone()
    }
}
impl<T, U> SelectInput<1> for (T, U)
where
    U: Clone,
{
    type VariableOutput = U;

    fn select_input(&self) -> Self::VariableOutput {
        self.1.clone()
    }
}
impl<T, U, V> SelectInput<0> for (T, U, V)
where
    T: Clone,
{
    type VariableOutput = T;

    fn select_input(&self) -> Self::VariableOutput {
        self.0.clone()
    }
}
impl<T, U, V> SelectInput<1> for (T, U, V)
where
    U: Clone,
{
    type VariableOutput = U;

    fn select_input(&self) -> Self::VariableOutput {
        self.1.clone()
    }
}
impl<T, U, V> SelectInput<2> for (T, U, V)
where
    V: Clone,
{
    type VariableOutput = V;

    fn select_input(&self) -> Self::VariableOutput {
        self.2.clone()
    }
}
