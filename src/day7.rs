use std::{vec, collections::HashMap};

use aoc_runner_derive::{aoc, aoc_generator};

use anyhow::Result;

type Data = HashMap<String, Vec<Content>>;

// Implementing a fake filesystem I guess???

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Content{
    File(String, usize),
    Directory(String)
}

// Commands are preceded by $
// $ cd: change directory
//   $ cd x: move from current directory into sub-directory x
//   $ cd ..: move from current directory into parent directory
// $ ls: list contents of current directory. Output comes in one of two forms:
//   123 abc: File abc with size 123
//   dir xyz: A directory named xyz
#[aoc_generator(day7)]
pub fn input_generator(input: &str) -> Result<Data> {
    input_generator_inner(input)
}
fn input_generator_inner(input: &str) -> Result<Data> {
    // I sure hope directory names are unique...
    // directory names are not unique :(
    let mut directories = HashMap::new();

    let mut dir_stack: Vec<&str> = vec![];
    let mut contents = vec![];
    for line in input.lines() {
        //dbg!(line);
        let mut iter = line.split_whitespace();
        // commands
        if line.is_empty() {
            continue;
        }
        if line.starts_with("$")
        {
            if line.starts_with("$ cd ") {
                // moving to a new directory: check if we just finished listing one
                if !contents.is_empty() {
                    let cur_dir:String = dir_stack.last().unwrap().to_string();
                    directories.insert(cur_dir, contents);
                    contents = vec![];
                }
                // the directory is the 3rd thing
                iter.next();
                iter.next();
                let location = iter.next().unwrap();
                if location == ".." {
                    // going up
                    dir_stack.pop();
                }
                else {
                    // going in
                    dir_stack.push(location);
                }
            }
        }
        // contents
        else
        {
            if line.starts_with("dir") {
                // subdirectory
                iter.next();
                let name = iter.next().unwrap().to_string();
                contents.push(Content::Directory(name))
            }
            else {
                // file
                let size: usize = iter.next().unwrap().parse()?;
                let name = iter.next().unwrap().to_string();
                contents.push(Content::File(name, size));
            }
        }
    }
    // make sure to do the last directory
    let last_dir = dir_stack.pop().unwrap().to_string();
    directories.insert(last_dir, contents);

    Ok(directories)
}

// Recursively get the total size of this directory
fn get_dir_size(name: &String, directories: &HashMap<String, Vec<Content>>) -> usize
{
    //dbg!(name);
    let mut total = 0;
    for content in directories.get(name).unwrap() {
        let content_size:usize = match content {
            Content::File(_, size) => *size,
            Content::Directory(subdir_name) => get_dir_size(subdir_name, directories),
        };
        total += content_size;
    }
    total
}

// Find all the directories with total size at most 100_000
// What is the sum of the total sizes of those directories?
#[aoc(day7, part1)]
pub fn solve_part1(input: &Data) -> usize {
    solve_part1_inner(input)
}
fn solve_part1_inner(input: &Data) -> usize {
    let mut total = 0;
    for (dir_name, _) in input {
        let dir_size = get_dir_size(dir_name, input);
        if dir_size <= 100_000 {
            dbg!(dir_name, dir_size);
            total += dir_size;
        }
    } 
    total
}

#[aoc(day7, part2)]
pub fn solve_part2(input: &Data) -> usize {
    solve_part2_inner(input)
}
fn solve_part2_inner(input: &Data) -> usize {
    unimplemented!()
}

#[cfg(test)]
mod test {
    const TEST_INPUT: &'static str =
r#"
$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k
"#;
    #[test]
    fn test_part1_example() {
        let input = super::input_generator(TEST_INPUT).unwrap();
        let result = super::solve_part1(&input);

        assert_eq!(result, 95437);
    }

    #[test]
    fn test_part2_example() {
        let input = super::input_generator(TEST_INPUT).unwrap();
        let result = super::solve_part2(&input);

        assert_eq!(result, 4);
    }
}
