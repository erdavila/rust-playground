use tailcall::{dump, tailcall};

fn main() {
    let addrs = addrs(vec![]);
    for addr in addrs {
        println!("{:p}", addr);
    }
}

#[tailcall]
fn addrs(mut ptrs: Vec<*const ()>) -> Vec<*const ()> {
    let ptr = &ptrs as *const Vec<*const ()>;
    let ptr = ptr as *const ();

    ptrs.push(ptr);
    if ptrs.len() < 3 {
        addrs(ptrs)
    } else {
        ptrs
    }
}

#[tailcall]
fn general_recursive(n: u8, trace: Vec<&str>) -> Vec<&str> {
    let mut trace = trace;
    match n {
        0 => {
            trace.push("0");
            general_recursive(1, trace)
        }
        1 => {
            trace.push("1");
            return general_recursive(2, trace);
        }
        2 => general_recursive(3, {
            trace.push("2");
            trace
        }),
        3 => {
            return general_recursive(4, {
                trace.push("3");
                trace
            })
        }
        4 => {
            trace.push("4");
            trace
        }
        _ => {
            trace.push("_");
            return trace;
        }
    }
}

#[dump]
fn general_loop(n: u8, trace: Vec<&str>) -> Vec<&str> {
    mod __tailcall {
        pub enum Control<A0, A1, R> {
            Continue(A0, A1),
            Return(R),
        }
    }

    let mut control = __tailcall_general_loop(n, trace);
    loop {
        match control {
            __tailcall::Control::Continue(a0, a1) => control = __tailcall_general_loop(a0, a1),
            __tailcall::Control::Return(r) => return r,
        }
    }

    fn __tailcall_general_loop(
        n: u8,
        trace: Vec<&str>,
    ) -> __tailcall::Control<u8, Vec<&str>, Vec<&str>> {
        let __tailcall_result = {
            let mut trace = trace;
            match n {
                0 => {
                    trace.push("0");
                    {
                        return __tailcall::Control::Continue(1, trace);
                    }
                }
                1 => {
                    trace.push("1");
                    return __tailcall::Control::Continue(2, trace);
                }
                2 => {
                    return __tailcall::Control::Continue(3, {
                        trace.push("2");
                        trace
                    });
                }
                3 => {
                    return __tailcall::Control::Continue(4, {
                        trace.push("3");
                        trace
                    })
                }
                4 => {
                    trace.push("4");
                    trace
                }
                _ => {
                    trace.push("_");
                    return __tailcall::Control::Return(trace);
                }
            }
        };
        __tailcall::Control::Return(__tailcall_result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_general_recursive() {
        test_general_fn(general_recursive);
    }

    #[test]
    fn test_general_loop() {
        test_general_fn(general_loop);
    }

    fn test_general_fn(f: fn(u8, Vec<&str>) -> Vec<&str>) {
        assert_eq!(f(0, Vec::new()), vec!["0", "1", "2", "3", "4"]);
        assert_eq!(f(1, Vec::new()), vec!["1", "2", "3", "4"]);
        assert_eq!(f(2, Vec::new()), vec!["2", "3", "4"]);
        assert_eq!(f(3, Vec::new()), vec!["3", "4"]);
        assert_eq!(f(4, Vec::new()), vec!["4"]);
        assert_eq!(f(50, Vec::new()), vec!["_"]);
    }
}
