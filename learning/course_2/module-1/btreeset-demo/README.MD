# Reflection Questions

## 1. How does the use of BTreeSet affect the output of the program compared to if a HashSet was used?

- BTreeSet maintain order of elements and keep all elements are uniqueness in the collection 
- HashSet only maintain the uniqueness of the collecion 

## 2. What are the benefits of using a BtreeSet over other collection types?

- keep all element following a specific order
- range queries 
- predictable performance:
- cache efficiency
- no hashing required

## 3. What would happen if you changed the BTreeSet to a different collection type, such as a Vec or LinkedList?

- Linked-List : for scalability, maintain time of appearance in the collection, no additional memory allocation when insert, delete elements allowing duplication elements
- Vec: appending elements is extremely fast (O(1))
- BtreeSet: maintain sortability, uniqueess between elements in the collection 

# Challenge Questions:

## 1. Modify the program to include the capability to remove a fruit from the BTreeSet.

## 2. Can you adjust the program to print out the list of unique fruits at the end in reverse order? 

## 3. How would you modify the program to keep track of how many times each fruit is generated? 