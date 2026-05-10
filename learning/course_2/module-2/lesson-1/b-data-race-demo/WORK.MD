# I. Reflection Questions

## 1. What is the purpose of using Mutex in here?

- to synchronization

## 2. How does Rust prevent data races when using Mutex?

- only a thread has a privellige manipulating in it at the moment. 

## 3. What happens when you try to compile the second block of code, and why?

# II. Challenge Questions

## 1. Can you modify the code to use read-write locks (RwLock) instead of a Mutex? What are the advantages and disadvantages?

- Advantage:

## 2. How would you handle potential deadlocks in a more complex application that uses multiple Mutexes?
- locks in a consistent order
- keep lock scope smallest as possible
- avoid nested locks
- use try_lock with timeout to avoid waiting forever
- use message passing to exchange data between threads
- use RwLock when read dominating
## 3. Can you extend the code to use conditional variables to synchronize thread execution?