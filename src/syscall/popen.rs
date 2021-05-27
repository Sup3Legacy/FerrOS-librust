
use crate::syscall;
use crate::io::OpenFlags;

use alloc::string::String;
use alloc::vec::Vec;

pub enum Opt {
    Read,
    Write,
}

pub enum Result {
    Finished(usize),
    Interrupted(usize),
}


pub struct pfile {
    id: usize,
    stdin: Option<usize>,
    stdout: Option<usize>,
}


impl pfile {
    pub unsafe fn popen(flags: Vec<Opt>, ex_name: String, args: Vec<String>) -> Self {
        let mut r = false;
        let mut w = false;
        for f in flags {
            match f {
                Opt::Read => r = true,
                Opt::Write => w = true,
            }
        }
        let fifo_in = syscall::open(&String::from("/def/fifo"), OpenFlags::ORD | OpenFlags::OWR);
        let fifo_out = syscall::open(&String::from("/def/fifo"), OpenFlags::ORD | OpenFlags::OWR);
        let id = syscall::fork();
        if id == 0 {
            syscall::dup2(crate::io::STD_IN, fifo_in);
            if w {
                syscall::dup2(crate::io::STD_OUT, fifo_out);
            }
            syscall::close(fifo_in);
            syscall::close(fifo_out);
            syscall::exec(&ex_name, &args);
            syscall::exit(usize::MAX);
            panic!("should not be reached")
        } else {
            Self {
                id,
                stdin: if w {
                    Some(fifo_in)
                } else {
                    syscall::close(fifo_in);
                    None
                },
                stdout: if r {
                    Some(fifo_out)
                } else {
                    syscall::close(fifo_out);
                    None
                },
            }
        }
    }

    pub fn pread(&self, data: &mut [u8], offset: isize, length: usize) -> usize {
        match self.stdin { 
            None => 0,
            Some(fd) => unsafe {
                let data2 = data.as_mut_ptr().offset(offset);
                syscall::read(fd, data2, length)
            }
        }
    }

    pub fn pwrite(&self, data: &[u8], offset: isize, length: usize) -> usize {
        match self.stdin { 
            None => 0,
            Some(fd) => unsafe {
                let data2 = data.as_ptr().offset(offset);
                syscall::write(fd, data2, length)
            }
        }
    }

    pub unsafe fn pclose(self) -> Result {
        match self.stdin {
            None => (),
            Some(fd) => {syscall::close(fd);},
        }
        match self.stdout {
            None => (),
            Some(fd) => {syscall::close(fd);},
        }
        let (id, return_val) = syscall::listen_proc(self.id);
        if id == 0 {
            syscall::kill(self.id);
            let mut id = 0;
            let mut output = 0;
            while id == 0 {
                let v = syscall::listen_proc(self.id);
                id = v.0;
                output = v.1;
            }
            Result::Interrupted(output)
        } else {
            Result::Finished(return_val)
        }
    }
}