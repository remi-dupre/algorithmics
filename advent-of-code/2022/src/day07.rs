use std::iter;

use anyhow::{bail, Context, Result};
use fxhash::FxHashMap;

type Size = u64;

#[derive(Debug, Default)]
pub struct Directory<'i> {
    directories: FxHashMap<&'i str, Directory<'i>>,
    files: FxHashMap<&'i str, u64>,
    size: u64,
}

impl<'i> Directory<'i> {
    fn iter_dirs<'s>(&'s self) -> impl Iterator<Item = &Directory<'i>> + 's {
        iter::once(self).chain(
            Box::new(self.directories.values().flat_map(Self::iter_dirs))
                as Box<dyn Iterator<Item = &Self>>,
        )
    }

    fn update_from_instructions(&mut self, instructions: &mut &[Instruction<'i>]) -> Result<()> {
        while let Some((inst, tail)) = instructions.split_first() {
            *instructions = tail;

            match inst {
                Instruction::Ls(entries) => {
                    for entry in entries {
                        match entry {
                            LsEntry::Dir { name } => {
                                self.directories.entry(name).or_default();
                            }
                            LsEntry::File { name, size } => {
                                self.files.insert(name, *size);
                            }
                        }
                    }
                }
                Instruction::Cd(name) => {
                    if *name == ".." {
                        break;
                    }

                    let dir = self
                        .directories
                        .get_mut(name)
                        .context(format!("could not find directory {name:?}"))?;

                    dir.update_from_instructions(instructions)?;
                }
            }
        }

        let dirs_size: u64 = self.directories.values().map(|dir| dir.size).sum();
        let files_size: u64 = self.files.values().sum();
        self.size = dirs_size + files_size;
        Ok(())
    }
}

#[derive(Debug)]
pub enum LsEntry<'i> {
    Dir { name: &'i str },
    File { name: &'i str, size: Size },
}

#[derive(Debug)]
pub enum Instruction<'i> {
    Ls(Vec<LsEntry<'i>>),
    Cd(&'i str),
}

pub fn parse<'i>(input: &'i str) -> Result<Vec<Instruction<'i>>> {
    input
        .split("$ ")
        .skip(1)
        .map(|cmd| {
            let cmd = cmd.trim_end();

            let inst = match cmd.split_at(2) {
                ("cd", path) => Instruction::Cd(&path[1..]),
                ("ls", tail) => {
                    let tail = tail.trim_start();

                    let output = tail
                        .lines()
                        .map(|line| {
                            let (kind, name) = line
                                .split_once(' ')
                                .context("missing space separator in ls response")?;

                            Ok({
                                if kind == "dir" {
                                    LsEntry::Dir { name }
                                } else {
                                    let size = kind.parse().context("invalid size")?;
                                    LsEntry::File { name, size }
                                }
                            })
                        })
                        .collect::<Result<_>>()?;
                    Instruction::Ls(output)
                }
                _ => bail!("invalid command {cmd}"),
            };

            Ok(inst)
        })
        .collect()
}

pub fn part1(instructions: &[Instruction]) -> Result<u64> {
    let mut root = Directory::default();
    root.update_from_instructions(&mut &instructions[1..])?;

    let sum = root
        .iter_dirs()
        .map(|dir| dir.size)
        .filter(|size| *size <= 100_000)
        .sum();

    Ok(sum)
}

pub fn part2(instructions: &[Instruction]) -> Result<u64> {
    let mut root = Directory::default();
    root.update_from_instructions(&mut &instructions[1..])?;

    let target_size = 40_000_000;
    let to_remove = root.size - target_size;

    let res = root
        .iter_dirs()
        .map(|dir| dir.size)
        .filter(|size| *size >= to_remove)
        .min()
        .unwrap(); // at least root can be removed

    Ok(res)
}
