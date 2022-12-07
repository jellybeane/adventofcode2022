use std::vec;

use aoc_runner_derive::{aoc, aoc_generator};

use anyhow::Result;

type Data = Vec<Vec<Content>>;

// Implementing a fake filesystem I guess???

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Content{
    File(String, usize), // name, filesize
    Directory(String, usize) // name, index
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
    // use indices instead
    let mut directories = vec![vec![]];

    let mut dir_stack: Vec<(&str, usize)> = vec![("/", 0)]; // directory name and index
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
                // moving to a new directory
                // the directory is the 3rd thing
                iter.next();
                iter.next();
                let location = iter.next().unwrap();

                // special case: root dir
                if location == "/" {
                    continue;
                }

                let &(cur_dir_name, cur_dir_index) = dir_stack.last().unwrap();
                let dir_contents = &mut directories[cur_dir_index];
                if !contents.is_empty() {
                    // we just ls-ed
                    // update directories to reflect contents
                    std::mem::swap(&mut contents, dir_contents);
                }

                if location == ".." {
                    // going up
                    //dbg!("popping", &dir_stack);
                    dir_stack.pop();
                }
                else {
                    // going in
                    // one of the things inside dir_contents is this directory
                    //dbg!(location, &dir_contents);
                    let &my_index = dir_contents.iter().filter_map(|x| {
                        match x {
                            Content::File(_, _) => None,
                            Content::Directory(name, index) => {
                                if name == location {
                                    Some(index)
                                }
                                else {
                                    None
                                }
                            },
                        }
                    }).next().unwrap();
                    dir_stack.push((location, my_index));
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
                let cur_index = directories.len();
                contents.push(Content::Directory(name, cur_index));
                // make a space for this dir on the directories vec
                directories.push(Vec::new());
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
    if !contents.is_empty() {
        let &(cur_dir_name, cur_dir_index) = dir_stack.last().unwrap();
        // update directories to reflect contents
        let dir_contents = &mut directories[cur_dir_index];
        std::mem::swap(&mut contents, dir_contents);
    }
    
    Ok(directories)
}

// Recursively get the total size of this directory
fn get_dir_size(index: usize, directories: &Vec<Vec<Content>>) -> usize
{
    //dbg!(index);
    let mut total = 0;
    for content in &directories[index] {
        let content_size:usize = match content {
            &Content::File(_, size) => size,
            &Content::Directory(_, subdir_index) => get_dir_size(subdir_index, directories),
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
    for (index, dir) in input.iter().enumerate() {
        let dir_size = get_dir_size(index, input);
        if dir_size <= 100_000 {
            //dbg!(index, dir_size);
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
    let total_space = 70000000;
    let want = 30000000;

    let root_dir_size = get_dir_size(0, input);
    let current_free = total_space - root_dir_size;
    let need_to_free = want - current_free;

    let mut smallest_found = total_space;
    for (index, dir) in input.iter().enumerate() {
        let dir_size = get_dir_size(index, input);
        if dir_size >= need_to_free && dir_size <= smallest_found {
            smallest_found = dir_size;
        }
    } 

    smallest_found
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

        assert_eq!(result, 24933642);
    }
}
