#[derive(Clone, Copy)]
pub struct Flags {
    pub fast: bool,
    pub silent: bool,
}

static mut flags: Flags = Flags { fast: false, silent: false };

pub fn init_flags(args: &Vec<String>) {
    for i in args {
        if i == "-f" {
            unsafe {
                flags.fast = true;
            }
        } else if i == "-s" {
            unsafe {
                flags.silent = true;
            }
        }
    }
}

pub fn get_flags() -> Flags {
    unsafe {
        return flags;
    }
}
