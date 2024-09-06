## rustash

###### a simple CLI tool to manage your notes

---

#### install

```
cargo install rustash
```

#### usage

##### add note

```
> rustash add hello
> rustash add world
```

##### list note

```
> rustash list
0) hello
1) world
```

##### show one note by index

```
> rustash show
0) hello

> rustash show 1
1) world
```

##### delete all notes

```
> rustash clear
> rustash list
no notes yet...
```
