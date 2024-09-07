## rustash

###### a simple CLI tool to manage your notes

---

#### install

```sh
cargo install rustash
```

#### usage

##### add note

```sh
> rustash add hello
> rustash add world
```

##### list note

```sh
> rustash list
0) hello
1) world
```

##### show one note by index

```sh
# show first note by default if no index provided
> rustash show
0) hello

> rustash show 1
1) world
```

##### delete all notes

```sh
> rustash clear
> rustash list
no notes yet...
```
