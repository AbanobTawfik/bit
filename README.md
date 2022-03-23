# bit
Bob and Rileys legit but not shit 

## Requirements

- [ ] Init a repository
- [ ] Set files to be added to next commit
- [ ] Set files to be removed from next commit
- [ ] Create a commit with the added files
- [ ] Show a log of previous commits (in the branch)
- [ ] Show current status (branch, commit and staged files)
- [ ] Create new branch
- [ ] Change current branch
- [ ] Merge branches without losing history

## Basic Commands

`bit init [<PATH>]`

Initialise the current directory or the path provided.

`bit commit [(-m|--message) <MESSAGE>] [<PATH>...]`

Commit files provided.

`bit branch <BRANCH>`

Create a branch with the specified name

## Upgrades to normal git

### Improved Compression

Using a single dictionary and dynamically upgrading it depending on added files means compression ratios are much higher. ZSTD also offers better compression ratios out of the box

### Multithreading Support

Using rayon as a support library allows for much faster processing of data than the traditional git

### History is Immutable

You can rebase but you can never erase history. Every change you make is tracked so you never lose 





