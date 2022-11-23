# My todolist app

## Install

```
cargo install --git https://github.com/floufen/todo
```

## Usage

### Adding

```
todo add important task 1
todo add important task 2
```

### Listing

List unchecked tasks

```
todo list
```

List all tasks checked and unchecked

```
todo list -a
```

### Check tasks

```
$ todo list
1. - [ ] important task 1
2. - [ ] important task 2

$ todo check 2
$ todo list
1. - [ ] important task 1

$ todo list -a
1. - [ ] important task 1
2. - [x] important task 2

$ todo uncheck 2
$ todo list
1. - [ ] important task 1
2. - [ ] important task 2
```

### Removing

```
$ todo remove <INDEX>
```

### Updating

```
$ todo update <INDEX> new text
```
