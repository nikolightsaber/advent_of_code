use std::error::Error;

#[derive(Debug)]
struct File {
    name: String,
    size: usize,
}

impl File {
    fn new(name: String, size: usize) -> Self {
        File { name, size }
    }
}

#[derive(Debug)]
struct Directory {
    name: String,
    files: Vec<File>,
    dirs: Vec<Directory>,
}

impl Directory {
    fn new(name: String) -> Self {
        Directory {
            name,
            files: Vec::new(),
            dirs: Vec::new(),
        }
    }

    fn cd(&mut self, name: &str) -> Option<&mut Self> {
        if self.name == name {
            return Some(self);
        }

        self.dirs.iter_mut().find(|dir| dir.name == name)
    }

    fn mkdir(&mut self, dir: Directory) {
        self.dirs.push(dir);
    }

    fn touch(&mut self, file: File) {
        self.files.push(file);
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut root = Directory::new(String::from("/"));
    let mut cwd = &mut root;
    let mut last_dir: Option<&mut Directory> = None;
    let input = include_str!("../inp_test.txt");

    for line in input.lines() {
        dbg!(line);
        let (left, right) = line.split_once(" ").expect("uncorrect input");
        match left {
            "$" => {
                let cmd = right.split_once(" ");
                match cmd {
                    Some(("cd", dir)) => {
                        if dir.len() == 0 {
                            panic!("Invalid input");
                        }

                        if dir == ".." {
                            break;
                            cwd = last_dir.take().expect("Invalid input");
                        }

                        cwd = cwd.cd(dir).unwrap();
                        // (cwd, last_dir) = match cwd.cd(dir) {
                        //     (Some(c_dir), l_dir) => (c_dir, l_dir),
                        //     _ => panic!("Invalid input"),
                        // };
                    }
                    None if right == "ls" => (), // Do nothing wait for next lines
                    _ => panic!("Invalid input"),
                }
            }
            "dir" => {
                let new_dir = Directory::new(String::from(right));
                cwd.mkdir(new_dir);
            }
            _ => {
                let file = File::new(String::from(right), left.parse().expect("Invalid input"));
                cwd.touch(file);
            }
        }
    }
    dbg!(root);

    Ok(())
}
