use std::marker::PhantomData;

struct Source;
impl Source {
    fn get(&self) -> Ref<'_> {
        todo!()
    }

    fn get_mut(&mut self) -> RefMut<'_> {
        todo!()
    }
}

struct Ref<'a> {
    phantom: PhantomData<&'a()>
}
impl<'a> Ref<'a> {
    fn get(&self) -> Ref<'a> {
        todo!()
    }

    fn get2<'s: 'a>(&self) -> Ref<'s> {
        todo!()
    }

    fn get3<'s>(&self) -> Ref<'s> where 'a: 's {
        todo!()
    }
}


struct RefMut<'a> {
    phantom: PhantomData<&'a mut()>
}
impl<'a> RefMut<'a> {
    fn get(&self) -> Ref<'a> {
        todo!()
    }

    fn get2<'s: 'a>(&self) -> Ref<'s> {
        todo!()
    }

    fn get3<'s>(&self) -> Ref<'s> where 'a: 's {
        todo!()
    }

    fn get_mut(&mut self) -> RefMut<'a> {
        todo!()
    }

    fn get2_mut<'s: 'a>(&mut self) -> RefMut<'s> {
        todo!()
    }

    fn get3_mut<'s>(&mut self) -> RefMut<'s> where 'a: 's {
        todo!()
    }
}

fn main() {
    let mut source = Source;
    {
        {
            let x = source.get();
            source.get();
            source.get_mut();
        }
        {
            let x = source.get();
            source.get();
            x.get();
        }
        {
            let x = source.get();
            source.get();
            x.get2();
        }
        {
            let x = source.get();
            source.get();
            x.get3();
        }
        {
            let x = source.get();
            x.get();
            x.get();
        }
        {
            let x = source.get();
            x.get();
            x.get2();
        }
        {
            let x = source.get();
            x.get();
            x.get3();
        }
        {
            let source = Source;
            let x = source.get();
            let y = x.get();
            drop(x);
            y.get3();
        }
    }

    //
}
