
//use winapi::um::namedpipeapi::CreatePipe;

struct Shell {
    pipe: Pipe
}

impl Shell {
    pub fn new() -> Self {
        Shell {
            pipe: Pipe::new("c:/windows/system32/cmd.exe"),
        }
    }

    pub fn Write(&self, cmd: &str) -> &str {
        self.pipe.Write(cmd)
    }

}


struct Pipe {

}

impl Pipe {

    fn new(program: &str) -> Self {
        Pipe {

        }
    }

    fn Write(&self, info: &str) -> &str {
        "empty write information!"
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn shell_test1() {
        let shell = Shell::new();
        println!("{}", shell.Write("net user"));
    }

}
