# I. Reflection Questions

## i. What are the owenership rules in Rust?

> - Each value in Rust has an owner.
> - There can only be one owner at a time.
> - When the owner goes out of scope, the value will be dropped.

## ii. How does owenership help manage memory safely in Rust?

- only once owner at a time -> no data race.
- compiler know lifetime of data because owenership rules -> compiler insert efficiently deallocate heap data. when a variable is at end life time, it is deallocated it's owned data in heap.
- ensuring any dangling pointer
- ensuring references never outlive their data

```text
            ┌─────────────────────────────────────────────────────────────┐
            │                   Ownership System                          │
            ├─────────────────────────────────────────────────────────────┤
            │                                                             │
            │  OWNERSHIP                                                  │
            │  → Every value has one owner                                │
            │  → Prevents: double free, memory leaks                      │
            │                                                             │
            │  MOVE SEMANTICS                                             │
            │  → Ownership transfers, not copies                          │
            │  → Prevents: double free on assignment                      │
            │                                                             │
            │  DROP                                                       │
            │  → Automatic deallocation when owner leaves scope           │
            │  → Prevents: memory leaks, resource leaks                   │
            │                                                             │
            │  BORROWING (&T / &mut T)                                    │
            │  → Use without owning                                       │
            │  → Prevents: use-after-free, data races                     │
            │                                                             │
            │  LIFETIMES                                                  │
            │  → References cannot outlive their data                     │
            │  → Prevents: dangling pointers                              │
            │                                                             │
            └─────────────────────────────────────────────────────────────┘
                      │
                      ▼
              All of this checked at COMPILE TIME
              Zero runtime overhead
              No garbage collector
```

## iii. What is the difference between a deep copy and s shallow copy?

- shallow copy - Copying the pointer, not the data
- deep copy -> duplicate everything

- rust has 2 trait to implement both kind of replicate structres:
  - `Copy` trait : Implicit, always shallow, only for stack data
  - `Clone` trait: explicit, always deep, for heap data
- otherwise: Move mechanism in ownership system: neither `copy` or `clone`, just transfer of ownership.

- Side-by-side comparision

    ```text
    ┌─────────────────┬──────────────────────┬──────────────────────┐
    │                 │    Shallow Copy      │      Deep Copy       │
    ├─────────────────┼──────────────────────┼──────────────────────┤
    │ What is copied  │ Stack data only      │ Stack + heap data    │
    │ Heap data       │ Shared (aliased)     │ Fully duplicated     │
    │ Independence    │ None — linked        │ Complete             │
    │ Cost            │ Cheap (always O(1))  │ Expensive (O(n))     │
    │ Risk            │ Double free, aliasing│ None                 │
    │ In Rust         │ Copy trait (implicit)│ Clone trait (explicit)│
    │ Example types   │ i32, bool, f64       │ String, Vec, HashMap │
    └─────────────────┴──────────────────────┴──────────────────────┘
    ```

## iv. When are moves or copies inexpensive in Rust?

- when copy/move data in stack type or copy the pointer of heap data
- Copies are inexpensive when copy stack only types
- struct and tuples of Copy types are also cheap.

Copy is cheap when:

- The type's entire representation fits on the stack
- There is no heap data to duplicate
- Size is fixed and small

Move is cheap always (almost):

- Only stack metadata is transferred
- Heap data never physically moves
- Cost is proportional to stack size, not heap size
- The only exception: unusually large stack-resident types

## v. What is the Copy trait, and when can it be used?

- `Copy` means:
  - In Rust, the default behavior when you assign a value or pass it to a function is to move it — transferring ownership. The Copy trait changes this behavior: instead of moving, Rust implicitly duplicates the value in place.
- can be used $\iff$
  - all fields must also implement `Copy`
  - The type must not implement `Drop`, because Copy only duplicates the stack data. If a type implements `Drop`, copying it could lead to undefined behavior when two values refer to the same heap allocation. In `Rust`, a type cannot implement both `Drop` and `Copy`.

# II. Discussion Prompts

## i. How does Rust's ownership model compare to garbage collection in other language? what are the tradeoffs?

- fundamental problem:
  - Free to early -> use-after-free. Free too late -> memory leak. Free twice -> double free. The answer to this question defines the entire character of a lang's memory model

```text
Strategy 1: Let the programmer decide     → C/C++   (fast, dangerous)
Strategy 2: Let a runtime decide          → GC langs (safe, overhead)
Strategy 3: Let the compiler decide       → Rust     (safe, no overhead)
```

### How Garbage Collection Works

- a garbage collector is a runtime component that preodically traces all live references in the program and frees anything that is no longer reachable.

```text
Program runs...
       │
       ├── allocates objects on heap
       ├── objects reference each other
       │
       ▼
GC runs (triggered by allocation pressure or timer)
       │
       ├── traces all references from "roots" (stack, globals)
       ├── marks everything reachable as "live"
       ├── frees everything not marked
       │
       ▼
Program continues...
```

- different GC strategies
  - mark-and-sweep
  - generational
  - reference counting
  - concurrent
- => a runtime component making decisions about memory at runtime.

### How Ownership Works?

```text
Compiler analyzes code...
       │
       ├── assigns every value exactly one owner
       ├── tracks where ownership moves
       ├── verifies borrow rules (no aliased mutation)
       ├── determines exactly when each value goes out of scope
       │
       ▼
Compiler emits drop() calls at precise locations
       │
       ▼
Program runs — memory managed by generated code, not a runtime
```

### Performance tradeoffs

- GC introduces **pause times** - moments where the program stops to collect garbage:

```text
GC program timeline:
────────────┬────────────────┬──────────┬─────────────────┬──────
  running   │   GC PAUSE     │ running  │   GC PAUSE      │
────────────┴────────────────┴──────────┴─────────────────┴──────
            ▲                           ▲
       unpredictable              unpredictable
       10ms–100ms+                timing and duration
```

- Modern GCs (Go, JVM G1, .NET) have reduced pauses dramatically through concurrent collection. But pauses are never fully elimnated

- Rust ownership model has **zero pause times** by design. Memory is freed exactly when the owner goes out of scope - deterministic, predictable, inline with program execution:

```text
Rust program timeline:
────────────────────────────────────────────────────────────────
  running   drop()  running  drop()  running  drop()  running
────────────────────────────────────────────────────────────────
               ▲                ▲                ▲
          deterministic    nanoseconds      compile-time
          timing           each              determined
```

### throughput - rougly comparable

- For raw throughput (total work done over time), modern GCs are surprisingly competitive. The JVM's GC has been tuned for decades. Go's GC is fast. The throughput gap between Rust and GC languages is often smaller than expected.
- The difference appears at the tail — the worst-case latency during GC pauses — not necessarily the average.

### Memory usage - ownership wins

- GCs languages require **2-5x more heap memory** that minimum needed

### Safety tradeoffs

- GC guarantees:
  - memory safety
  - but not concurrency safety
- Rust ownership model guarantees:
  - provide both **memory safety** and **data race freedom**

### Dev Exp Tradeoffs

```text
┌──────────────────┬──────────────────────────────────────┬──────────────────────────────────────┐
│                  │ Garbage Collection (GC)              │ Ownership Model                      │
├──────────────────┼──────────────────────────────────────┼──────────────────────────────────────┤
│ General idea     │ Low upfront friction, pain deferred  │ High upfront friction, pain front\   |
|                  |                                      |-loaded.                              │
│ Writing code     │ Easy — allocate freely,              │ Hard initially — fighting            │
│                  │ never think about memory             │ the borrow checker                   │
├──────────────────┼──────────────────────────────────────┼──────────────────────────────────────┤
│ Debugging        │ Hard — memory issues appear as       │ Easy — memory bugs are caught        │
│                  │ mysterious slowdowns, OOM crashes,   │ at compile time                      │
│                  │ or GC pressure                       │ Runtime crashes are usually          │
│                  │                                      │ logic bugs, not memory bugs          │
├──────────────────┼──────────────────────────────────────┼──────────────────────────────────────┤
│ Optimization     │ Hard — GC behavior is opaque         │ Easy — allocation is explicit,       │
│                  │ and allocation patterns are          │ visible, and predictable             │
│                  │ difficult to reason about            │                                      │
└──────────────────┴──────────────────────────────────────┴──────────────────────────────────────┘

friction: ma sát
```

## Reference Counting - A middle ground

- It is worth noting that Rust does offer **optional reference counting** via `Rc<T>` and `Arc<T>`
=> a form of GC with different tradeoffs

- limitations:
  - cannot handle **cycles** references

## ii. What ownership rules have you found most confusing learning Rust? How did you gain understanding?

- confusing:
  - no two mutable references
  - into_iter(), iter_mut(), iter()
  - borrow lifetime
  - shadow
  - move semantics
- gain understanding:
  - Lifecycle of learning and growth:
    >Practice → try new things as much as possible → stay curious → face mistakes → search and read → fix issues guided by the compiler’s “annoying” errors → explore solutions from others → learn from mistakes → repeat and improve.
    > =>Pain then gain

## iii. Why do you think Rust favors moves over deep copying by default? What are the advantages?

- Moves are cheaper but deep copying need drastically to cost

- Moves make the cost of data transfer explicit and honest.
- Moves Naturally Enforce Single Ownership. -> requires transfer, not duplication
- Moves prevent an entire class of bugs. prevent:
  - use-after-move
  - double free
  - accidental aliasing
- Moves compose correctly with RAII(Resource Acquisition Is Initialization) - resources are tied to object lifetimes and released when the owner drops.
  - only once owner for a resource -> to be released once
- Moves encourage better API design.
  - The devs aware of when need to borrow or move and mutable or immutable
- zero-cost abstraction - Moves are free

## iv. what are some cases where the ownership rules may impose too much ceremony on the Rust code? How could it be improved?

## v. How does ownership affect how you design and structure programs in Rust? What changes compared to other languages?

- funndamental shift: In rust, ownership adds a second dimension:
  - Other languages:  "What is the right structure for this problem?"

  - Rust:"What is the right structure for this problem,
                   given that data must have clear ownership?"

### 1. Data structure become tree-shaped, not graph-shaped

- Rust's ownership model is inherently **hierarchical**
- Every value has one owner, onwers form as tree

### 2. The arena pattern replaces object graphs

- when one genuinely need graph-shaped data -> Rust solution is the **arena pattern**: store all nodes in a central **Vec** reference them by index

### 3. Ownership makes data flow explicit in APIs

`du-nw`

### 4. Builder Pattern becomes idiomatic

`du-nw`

### 5. State machines map onto ownership naturally

- Ownership enables typestate programming

```rust
// States as distinct types — not enums, not booleans
struct Unvalidated(FormData);
struct Validated(FormData);
struct Submitted(FormData);

impl Unvalidated {
    fn validate(self) -> Result<Validated, ValidationError> {
        // consumes Unvalidated, produces Validated
        // cannot skip validation — the type system enforces it
        Ok(Validated(self.0))
    }
}

impl Validated {
    fn submit(self) -> Submitted {
        // cannot submit unvalidated data — Unvalidated has no submit method
        Submitted(self.0)
    }
}
```

### 6. Error Handling intergrates with ownership

`dunw`

### 7. Concurrency Design becomes structural

- wrap shared data between threads before move them. Some struct for the job canoncially like : `Arc<T>`

### 8. The component boundary problem

```text
Other languages:
  Component A holds reference to Component B
  Component B holds reference to Component A
  → Circular dependency, handled by GC

Rust:
  This requires Rc<RefCell<>> or Arc<Mutex<>>
  → The compiler makes the cycle visible and costly
  → Pushes toward better-separated components
```

- Rust prgrams tend to have `lower coupling` between components

### 9. The lifetime of data shapes module structure

- In other languages, modules are organized around behavior — what does this module do?
- In Rust, ownership pushes devs to also think about data lifetime — who creates this data, and who is responsible for it?

ie:

```rust
// Common Rust pattern: one authoritative owner per resource
// Other modules borrow from the owner — they never own

mod storage {
    pub struct Store { data: HashMap<String, Value> }

    impl Store {
        pub fn get(&self, key: &str) -> Option<&Value> { ... }
        pub fn insert(&mut self, key: String, value: Value) { ... }
    }
}

// Other modules borrow from Store — never own the data
mod display {
    pub fn render(store: &storage::Store) { ... } // borrows
}

mod processor {
    pub fn process(store: &mut storage::Store) { ... } // borrows mutably
}
```
