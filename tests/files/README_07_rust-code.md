# Rust test

<!-- BEGIN mktoc -->
  - [Derive](#derive)
<!-- END mktoc -->

### Derive

Here is an example with the `#[derive(Clone, Debug)]` annotation inside the code example. This would break earlier versions of `mktoc`
.
```rust
#[derive(Clone, Debug)]
pub struct MyStruct<'a> {
    pub name: &'a str,
    pub id: String
}

impl Default for MyStruct<'_> {
    fn default() -> Self {
        Self{
            name: "my_struct",
            id: String::from("1234-4321-9817"),
        }
    }
}
```
It's a good idea to implement the Default trait for all pub struct. This way users can use them with `MyStruct{..Default::default()}` instead of needing to manage all fields everywhere.

Now we can use `..Default::default()` to assign default values and we can also define any value we want to overwrite.

```rust
let a = MyStruct{name: "a_struct", ..Default::default()};

assert_eq("a_struct", a.name);
assert_eq(String::from("1234-4321-9817"), a.id);
```

### No lang tag code block

```
## this is some text
just some text :)
```