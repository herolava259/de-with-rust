

# Why is trait more powerful than interface(c#/java)

1. implement for any struct, enum, primitive, external type

2. blanket implementation 

```rust
impl<T: Clone> MyTrait for T {
    // any T is kind of Clone also use 
}
```
3. trait bound + generic = zero-cose abstraction 


4. Associated types (clear over interface)

5. default method + flexible override 

6. Trait object + generic (two types of polmorphism)

- `T: Trait`
- `dyn Trait`

# Features More

- has default methods

- trait bound 

- `where` clause 

- trait as return type `fn get_sss() -> impl SSS`

- trait object (dynamic dispatch):  `fn do_somethin(tr: &dyn SSS)...`

- inheritence trait `trait B: A`

- derive trait `#[derive(Debug, Clone, PartialEq, Hash)]`

- trait + lifetime: eg`fn longest<'a, T: AsRef<str>>(x: &'a T, y: &'a T) -> &'a str`

