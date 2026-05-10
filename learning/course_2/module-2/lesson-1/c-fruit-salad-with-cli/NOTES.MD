## Running the program

`cargo run -- --csvfile fruits.csv`

or 

`cargo run -- --fruits "apple, pear"`


# I. Reflection Questions

## 1. What is the role of the `create_fruit_salad` function in the lib.rs file?

- to mess-up `fruit-salad vec` by way randomly.

## 2. How does the program read input from either a CSV file or command-line arguments?

- read file name from cli arg -> read file'content as string with the provided user's input -> read line by line and parse the content with separated characted between columns and turn into a data structure 

## 3. What is the purpose of the `Opts` struct in main.rs?

- provide some argument that the user can give for the cli program can be understandable

# II. Challenge Questions

## i. How would you modify the program to allow for customizable "salad dressing", such as adding a random syrup or spice to the fruit salad?

- Extend the program to allow adding or customizing dishes.

## ii. Can you extend the application to output the resulting fruit salad to a new CSV file?
- `done`
## iii. Can you adapt this code for different kinds of data, such as a list of books or movies?

`done`