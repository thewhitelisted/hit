# hit: a shitty git clone written in blazingly fast rust

```rust
let git: Git = Git::new();
let hit: Git = git.clone();
```

## motivations

i always wanted to understand and learn lower level programming at a higher level. so, i decided to learn how git works under the hood. this project should hopefully be a functional version of git, written in rust, that i can use (i will not use this) in other projects.

i named the project "hit" because it is similar to git and it was the next letter after g.

## install

you can install hit by cloning this repository and running 

```bash
cargo build --release
```

after doing this, you can add the hit.exe file to your path in /target/release/

## current features

### make a repository!

```bash
hit init
```

### stage files!

```bash
hit add .
```

### unstage or delete files!

```bash
hit rm --cached <file>
hit rm <file>
```

### commit changes!

```bash
hit commit [-m message]
```

### get a tree hash!

```bash
hit write-tree
```

### use stage hash to commit!

```bash
hit commit-tree [tree-hash] [-m message]
```

### checkout!

```bash
hit checkout [commit-hash]
hit checkout [branch]
```

### create and list branches!

```bash
hit branch [name]
hit branch
```

### track your changed, deleted, and created files!

```bash
hit status
```

### reset index to HEAD

```bash
hit reset <path>
```

## TODOS

- [x] init
- [x] hash-object
- [x] cat-file -p
- [x] write-tree
- [x] commit-tree
- [x] hitignore
- [x] checkout -> not quite finished, see TODOS in /commands/checkout.rs
- [x] branch
- [x] status
- [x] add and .hit/index
- [x] commiting!
- [x] reset
- [x] log
- [ ] config
- [ ] diff
- [ ] merge
- [ ] rebase
- [ ] remote (create tcp server and store hit repositories)