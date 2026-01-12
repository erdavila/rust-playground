use std::fmt::Display;

#[derive(Clone, Copy)]
pub struct Indentation {
    level: u8,
    single_size: u8,
}

impl Indentation {
    pub fn new(single_size: u8) -> Self {
        Indentation {
            level: 0,
            single_size,
        }
    }

    pub fn next(self) -> Self {
        Indentation {
            level: self.level + 1,
            single_size: self.single_size,
        }
    }
}

impl Display for Indentation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let total_size = self.level * self.single_size;

        let padding = if true {
            " ".repeat(total_size as usize)
        } else {
            let char = (b'0' + self.level) as char;
            if total_size > 0 {
                char.to_string().repeat((total_size - 1) as usize) + "]"
            } else {
                String::new()
            }
        };

        write!(f, "{padding}")
    }
}
