## I. Reflection Questions

### i. What are the two programming languages contained within Rust?

- Rust contains both a safe and unsafe programming language.
- Rust is designed with memory safety in mind, and rustc, the Rust compiler, provides strong guarantees through its ownership model and borrow checker. Unlike many other languages, Rust does not rely on garbage collection for memory management. Instead, it gives developers precise control over when and where memory is allocated and used.

- The separation between Safe Rust and Unsafe Rust is controlled with the `unsafe` keyword, which acts as a interface from one to the other.

- The `unsafe` keyword has two uses:
  - to declare the existence of contracts the compiler can't check
  - to declare that a programmer has checked that these contracts have been upheld.

- unsafe rust can do:
  - dereference raw pointer

    ``` rust
    fn main() {
        let x = 42;
        let ptr: *const i32 = &x;
        let null_ptr: *const i32 = std::ptr::null();

        unsafe {
            println!("{}", *ptr);
        }
    }
    ```

  - call `unsafe` function / method. include some FFI functions (Foreign Function Interface) from C/C++:

    ```rust
    unsafe fn dangerous() {

    }

    extern "C" {
        fn abs(x: i32) -> i32;
    }

    fn main() {
        unsafe {
            dangerous();
            println!("{}", abs(-5)); // gọi hàm C
        }
    }
    ```

  - read/write `static mut`

    ```rust
    static mut COUNTER: u32 = 0;

    fn main() {
        unsafe {
            COUNTER += 1;
            println!("{}", COUNTER);
        }
    }
    ```

  - implement `unsafe trait`
  - Some common unsafe traits in Rust include Send, Sync, and GlobalAlloc.
    - Send — indicates that ownership of a type can be transferred safely between threads.
    - Sync — indicates that a type can be safely shared between multiple threads through references.
    - GlobalAlloc — used for implementing custom global memory allocators.
- from rust nomicon book:

> That’s it. The reason these operations are relegated (chuyển xuống, chuyển vào) to Unsafe is that misusing any of these things will cause the ever dreaded(đáng sợ) Undefined Behavior. Invoking Undefined Behavior gives the compiler full rights to do arbitrarily bad things to your program. You definitely should not invoke Undefined Behavior.

### ii. When might a programmer need to use Unsafe Rust instead of Safe Rust?

- low level control over implementation details - without most of the problems that come with trying to integrate it with a completely different safe language.

`When`:

- FFI Gọi code từ C/ C++ hoặc ngôn ngữ khác
- Direct manipulation with memory:

```rust
let f: f32 = 1.0;
let bits: u32 = unsafe {
    std::mem::transmute(f)
}
```

- Extremely high performance optimization

```rust
let val = slice[i];

let val = unsafe { *slice.get_unchecked(i) }; 
```

- Implement data-structure that is impossible to implement in safe Rust

- Write OS, Embedded or bare-metal

- Implement `Send` / `Sync` manually

- Write custom memory allocator

### iii. What safety guarantees does Safe Rust provide?

- no dangling pointer, and doesnt use-after-free
- no data race. rules -> At the same time:
  - multiple &T
  - `Or` only once &mut T
=> equivalent: one writer- multiple reader

- no buffer overflow / out-of-bound access

- no null pointer dereference

- no double free: error compile when use after free pointer, or the owenership of the data has recently moved

- no existence Uninitialized-memory

- doesnt have interger-overflow

### iv. How does Unsafe Rust relate to Safe Rust?

```text
            ┌─────────────────────────────────────┐
            │             Unsafe Rust             │
            │                                     │
            │   ┌─────────────────────────────┐   │
            │   │          Safe Rust          │   │
            │   │                             │   │
            │   │ ownership, borrowing,       │   │
            │   │ lifetimes, type system      │   │
            │   └─────────────────────────────┘   │
            │                                     │
            │ + raw pointers, FFI, static mut,    │
            │   unsafe traits, union fields       │
            └─────────────────────────────────────┘
```

- unsafe-rust is a **strict superset** of safe rust. Every valid Safe Rust program is also valid Unsafe Ryst. The `unsafe` keyword simply unlocks a small set of additional capabilities that the rustc cannot verify on its own.

#### Safe Rust: The Compiler is your safety net

- In Safe Rust, the compiler **statically enforces** a set of rules that eliminate entire classes of bugs - memory errors, data races, null dereferences - before your program ever runs. You trade some flexibility for a strong guarantee: **If it compiles, it is memory safe.

#### Unsafe Rust: The programmer become the Safety Net

- When the dev enter an`unsafe` block, they are making a contract with the compiler:

> I know you cannot verify this. I will take responsibility for upholding(ủng hộ) the invariants myself

- The compiler doesnt stop checking what it can. Ownership, lifetimes, and the type system all remain fully active. The only difference is that five specific operations are now permitted:

### v. What is the value of separating safe and unsafe langs in Rust?

- Rust is a programming language designed to guarantee memory safety at compile time.
- Unsafe Rust allows developers to bypass some of the compiler’s safety restrictions and perform low-level memory operations similar to those in C or C++. However, developers are then responsible for manually ensuring memory safety and correctly managing memory lifetimes

- Unsafe rust unlock it to it has is capable like C/C++:

- `unsafe` is the **foundation that safe Rust is built on** — and the separation ensures that foundation stays contained.

## II. Discussion Prompts

### i. In what situations have you needed to use unsafe features in a systems programming lang? What problems did you encounter?

- FFI Bindings to C Libraries
- Building custom data structures
- memory-mapped I/O Embedded / OS development
- performance-critical hot paths
- implementing a custom allocator

### ii. How do you think Rust's approach to safe/unsafe compares to other systems langs like C++? What are the tradeoffs?

- The fundamental philosophical difference
  - C/C++:  "Trust the programmer. Safety is opt-in."
  - Rust:   "Trust the compiler. Unsafety is opt-in."

-

- The core tradeoff matrix:

| Feature                   | C                | C++              | Rust               |
|----------------------------|------------------|------------------|--------------------|
| Safety model               | None             | Opt-in, optional | Opt-out, enforced  |
| Unsafe visibility          | Invisible        | Invisible        | Explicit           |
| Compiler guarantee         | None             | Partial          | Strong             |
| Learning curve             | Low              | Very high        | High               |
| Expressiveness             | High             | Very high        | High               |
| Legacy code                | Enormous         | Enormous         | Small but growing  |
| Audit difficulty           | Entire codebase  | Entire codebase  | Only unsafe blocks |
| Zero-cost abstractions     | Manual           | Yes              | Yes                |
| Memory model               | Manual           | RAII + manual    | Ownership system   |

### iii. What techniques can minimize the amount of unsafe code required in Rust?

**principle** => *Unsafe should be last resort, not the first instince*

1. Use Std Lib abstraction first

2. Use `Cell` and `RefCell` for Interior Mutability
    - A common reason people reach for unsafe is needing to mutate through a shared reference. Cell and RefCell handle this safely.

3. Use `Pin` for Self-Referential Structures
    - Self-referential structs are a classic reason to reach for raw pointers. Pin was designed specifically to handle this safely

    - Without Pin — raw pointers tempting for async state machines
    - With Pin — compiler enforces that pinned data cannot move

4. Encode Invariants in the Type System
    - Instead of relying on runtime checks or unsafe assumptions, use types to make invalid states unrepresentable. This eliminates the need for unsafe checks entirely,

    ```rust
    // Weak approach — validity checked at runtime, unsafe temptation
    fn send_packet(data: &[u8], is_validated: bool) {
        if is_validated { /* ... */ } // easy to forget, easy to bypass
    }

    // Strong approach — validity encoded in the type
    struct ValidatedPacket(Vec<u8>); // can only be constructed through validation

    impl ValidatedPacket {
        pub fn new(data: Vec<u8>) -> Option<Self> {
            if data.len() > 0 && data.len() < 1500 {
                Some(ValidatedPacket(data)) // ✅ invariant guaranteed by construction
            } else {
                None
            }
        }
    }

    fn send_packet(packet: &ValidatedPacket) {
        // No need to check — the type already guarantees validity
        // No unsafe needed
    }
    ```

5. Use Wrapper Types Instead of Raw Pointers

    ```rust
    // Raw pointer scenarios and their safe alternatives:

    *mut T        →  Box<T>          // unique ownership
    *mut T        →  Rc<T>           // shared ownership, single thread
    *mut T        →  Arc<T>          // shared ownership, multi thread
    *mut T        →  Weak<T>         // non-owning reference (breaks cycles)
    *const T      →  &T              // borrowed reference
    *mut T        →  &mut T          // exclusive borrowed reference
    *mut T        →  RefCell<T>      // runtime-checked mutability
    ```

6. Use `bytemuck` and `zerocopy` for Type Punning(trừng phạt)
    - Crates like bytemuck and zerocopy provide safe abstractions for this:

    ```rust
    use bytemuck::{Pod, Zeroable};

    // Instead of unsafe transmute...
    let bytes: [u8; 4] = [0x01, 0x00, 0x00, 0x00];
    let val: i32 = unsafe { std::mem::transmute(bytes) }; 

    // Derive Pod (Plain Old Data) and cast safely
    #[derive(Pod, Zeroable, Copy, Clone)]
    #[repr(C)]
    struct Packet {
        id: u32,
        length: u16,
        flags: u16,
    }

    let val: &Packet = bytemuck::from_bytes(&bytes); // safe, checked at compile time
    ```

7. Isolate Unsafe Behind a Minimal Safe Abstraction

8. Reach for `Miri` Early
    - Miri is Rust's interpreter that detects undefined behavior in unsafe code at runtime during testing. Using it early catches problems before unsafe blocks proliferate:

    ```bash
    cargo +nightly miri test
    ```

9. Check the Ecosystem Before Writing Unsafe

## References

- [how-safe-and-unsafe-interact](https://doc.rust-lang.org/nomicon/safe-unsafe-meaning.html)
- [what-unsafe-can-do](https://doc.rust-lang.org/nomicon/what-unsafe-does.html)
- [working-with-unsafe](https://doc.rust-lang.org/nomicon/working-with-unsafe.html)
- [box](https://doc.rust-lang.org/book/ch15-01-box.html)
- [smart-pointers](https://doc.rust-lang.org/book/ch15-00-smart-pointers.html)

## Appendix

- notes: from nomicon rust's book

> As an aside, while `Send` and `Sync` are unsafe traits, they are also automatically implemented for types when such derivations are provably safe to do. `Send` is automatically derived for all types composed only of values whose types also implement `Send`. `Sync` is automatically derived for all types composed only of values whose types also implement `Sync`. This minimizes the pervasive(tràn lan, lan tràn) unsafety of making these two traits unsafe. And not many people are going to implement memory allocators (or use them directly, for that matter).
------------------------------------------------------
> This is the balance between Safe and Unsafe Rust. The separation is designed to make using Safe Rust as ergonomic(thuận tiện, công thái học) as possible, but requires extra effort and care when writing Unsafe Rust. The rest of this book is largely a discussion of the sort of care that must be taken, and what contracts Unsafe Rust must uphold.

- `Send` trait: moving ownership to the another thread
- `Sync` trait: share the reference between multiple threads
