mod general {
    #[tailcall::tailcall]
    fn general(n: u8, trace: Vec<&str>) -> Vec<&str> {
        let mut trace = trace;
        match n {
            0 => {
                trace.push("0");
                general(1, trace)
            }
            1 => {
                trace.push("1");
                return general(2, trace);
            }
            2 => general(3, {
                trace.push("2");
                trace
            }),
            3 => {
                return general(4, {
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

    #[test]
    fn test() {
        assert_eq!(general(0, vec![]), vec!["0", "1", "2", "3", "4"]);
        assert_eq!(general(1, vec![]), vec!["1", "2", "3", "4"]);
        assert_eq!(general(2, vec![]), vec!["2", "3", "4"]);
        assert_eq!(general(3, vec![]), vec!["3", "4"]);
        assert_eq!(general(4, vec![]), vec!["4"]);
        assert_eq!(general(50, vec![]), vec!["_"]);
    }
}

mod stack_addresses {
    // #[syn_dump::dump(suffix = "-BEFORE")]
    #[tailcall::tailcall]
    // #[syn_dump::dump(suffix = "-AFTER")]
    fn stack_addresses(option: Option<Vec<*const ()>>) -> Vec<*const ()> {
        if let Some(mut ptrs) = option {
            let ptr = &ptrs as *const Vec<*const ()>;
            let ptr = ptr as *const ();

            ptrs.push(ptr);
            if ptrs.len() < 3 {
                stack_addresses(Some(ptrs))
            } else {
                ptrs
            }
        } else {
            stack_addresses(Some(Vec::new()))
        }
    }

    #[test]
    fn test() {
        let addresses = stack_addresses(None);
        let first_addr = addresses[0];
        for addr in &addresses[1..] {
            assert_eq!(*addr, first_addr);
        }
    }
}

mod no_args {
    // #[syn_dump::dump(suffix = "-BEFORE")]
    #[tailcall::tailcall]
    // #[syn_dump::dump(suffix = "-AFTER")]
    fn no_args() -> u8 {
        static mut N: u8 = 0;

        unsafe {
            N += 1;
            if N < 5 {
                no_args()
            } else {
                let value = N;
                N = 0;
                value
            }
        }
    }

    #[test]
    fn test() {
        assert_eq!(no_args(), 5);
    }
}

mod no_explicit_return_type {
    static mut DONE: bool = false;

    // #[syn_dump::dump(suffix = "-BEFORE")]
    #[tailcall::tailcall]
    // #[syn_dump::dump(suffix = "-AFTER")]
    fn no_explicit_return_type(n: u8) {
        if n > 0 {
            no_explicit_return_type(n - 1);
        } else {
            unsafe { DONE = true };
        }
    }

    #[test]
    fn test() {
        unsafe { DONE = false };

        no_explicit_return_type(5);

        assert!(unsafe { DONE });
    }
}
