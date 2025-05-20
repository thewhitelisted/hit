# hit: a shitty git clone written in blazingly fast rust

## motivations

i always wanted to understand and learn lower level programming at a higher level. so, i decided to learn how git works under the hood. this project should hopefully be a functional version of git, written in rust, that i can use (i will not use this) in other projects.

i named the project "hit" because it is similar to git and it was the next letter after g.

## TODOS

- [x] init
- [x] hash-object
- [x] cat-file -p
- [x] write-tree
- [x] commit-tree
- [x] hitignore
- [x] checkout -> not quite finished, see TODOS in /commands/checkout.rs
- [x] branch
- [ ] status
- [ ] add and .hit/index
- [ ] log