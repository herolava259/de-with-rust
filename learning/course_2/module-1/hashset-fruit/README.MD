# Reflection Questions:

## 1. How does the generate_fruit() function ensure a random fruit is chosen every time it's called?

- random core: `thead_rng` choose a element from a specific collection randomly or follow by uniform distribution? 


## 2. What is the purpose of using a HashSet in this program?

- save all elements inserted without duplication? 

## 3. What would happen if you changed the HashSet to a different collection type, such as a Vec or LinkedList?

- To Vec: save element inserted or push sequentially. For example: insert at last or push elemement 1 into the vec, ensure all element inserted follow by the time they are in the vec. The internal structure of The `Vec` collection is a consecutive memory with a specific capacity. If the vec is fully of elements, it will extent capacity by call os or advisor take more memory or reallocation more memory than before.

- To Linked-List: imagine it like a chain with multiple link can be connected with references of them 
    - It is a datastructure has order 
    - poor performance to query a element with specific position. 

# Challenge Questions:

## 1. Modify the program to generate a user-specified number of random fruits. Hint: you'll need to use command-line arguments to get input from the user.

## 2. How would you modify the program to keep track of how many times each fruit is generated? Hint: you might need to use a different collection type.

## 3. Can you adjust the program to print out the list of unique fruits at the end?