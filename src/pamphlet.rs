/* 1.lesson1:
2.lesson2:
Programs use binary to store any type of data.

Data types:
Memory only stores binary data. However, anything can be represented in binary. Usually, the programmer works at a higher level
than binary, so you generally don't need to worry about the underlying binary data.

Program determines what the binary data represents and the code that you write is automatically converted into binary representations.

Basic types that are universally useful are provided by the language itself. However, you can create your own types.

Anything can be represented with binary data.

3.lesson3: variables
Variables are a way to assign data to a temporary memory location which allows programmer to easily work with memory.

By default variables are immutable.
The language is able to increase the speed of your program when you have immuatble data, because it doesn't need to check if anything was changed
because nothing can changed, since it's immutable.


4.lesson4: functions

5.lesson5: println macro
In macros, in comparison to functions, macros simply generate additional rust code whereas functions, perform actions and evaluates
different things. So whenever you see a macro call, all it's really doing is pasting in a bunch of code for you that you don't have to
write.

Macros use an exclamation point to call but function call does not use an exclamation point.
Macros generate additional rust code for us, so you don't need to write it.

6.lesson6: Control flow using "if":
Try to always include "else" unless there truly is no alternative case.

7.lesson7: Repetition using loops:
loop: infinite loop
while: conditional loop

Both types of loops can exit using "break"

8.lesson8: Tool instalation:
Windows:
- rustup: Manages rust instalation
- vscode
- MSVC C++ build tools: Needed to build on windows

Mac:
brew install rustup-init
brew install --cask visual-studi-code

linux:
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
or use package manager
after package manager install:
rustup toolchain install stable

9.lesson9: Comments:

10.lesson10: Activity - functions:
bin stands for binary.
To run program: cargo run --bin a1(the name of the file containing main function?)

Compiling is the step that takes your code and turns it into machine instructions for the CPU to execute.
If you don't want to see the messages everytime you run the program, pass a special flag into the cargo tool called -q .
This can be useful if you just making small changes and you don't need to how long it took to compile the program nor you need to know whether it
was in debug or release mode

11-lesson11: Basic arithmetic:

12-lesson12: activity - Basic math:

13-lesson13: demo - control flow with if & else

14-lesson14: activity 3a - control flow with if & else

15-lesson15: activity - control flow with if & else .. if

16-lesson16:
Match expressions are similar to if else expressions, however there's one key difference which is match must be exhaustive which means
every possible option must be accounted for in your code.

At the end of each arm in match, we use a comma instead of a semicolon, since match works on expressions not statements.
Important: The key difference is a statement ends a line whereas a comma ends an expression.

match vs else .. if
- With match every possibility will be checked by the compiler and if you forget one, then the compiler will tell you. Also if a new possibilty
is added, then you will be notified in order to handle this situation, but when using else .. if , this will not occur.

Prefer match over else if , when working with a single variable.
Match considers all posibilites which leads to more robust code. But when working with data that has a lot of different possibilities and you only
need to handle a few of them, use underscore to match anything else, which would be the rest of the possibilities.*/
/* 17.lesson17: Demo - making decisions with match:

18.lesson18: activity - basic match expressions:

19.lesson19: activity - basic match expressions:

20-lesson20: Demo - repetition using loop:


21-lesson21: activity - repetition using loop:

22-lesson22: demo - repetition using while:

23-lesson23: activity - repetition using while:

24-lesson24: working with data | enum:
- Enums can only be one variant at a time
- more robust programs when paired with match
- make program code easier to read, that way that we can pass around enumerations instead of just numbers and strings

25-lesson25: Demo | enum:

26-lesson 26: Activity | enum:

27-lesson 27: Working with data | struct:
- All or nothing - each piece of data within the structure must be populated, you cannot have some part of the structure and not the others.
- Each piece of data within the structure is called a field
- structures making work with data easier because similar data can be grouped together.

So:
- structs deal with multiple pieces of data
- all fields must be present in order to create a struct
- fields can be accessed using a dot

28-lesson28: Demo | struct

29-lesson29: Activity | struct

30-lesson30: Working with data | tuples
- a tuple is a type of "record" and a record can be thought of as a line of information on a piece of paper. Tuples are essentially a way to
 access each piece of data within that line or within that record.
- store data anonymously and there's no need to name fields in your tuples, unlike struct or enum where field or variant has to have it's own unique name.
- they are useful to return pairs of data from functions
- can be destructured easily into variables

So:
- Allow for anoynymous data access
- useful for destructuring
- can contain any number of fields. However you will want to use struct when you're working with more than 2 or 3 fields

31-lesson31: Demo | tuples

32-lesson32: activity | tuples:

33-lesson33: expressions:
- rust is an expression-based language. This means that most things are evaulated and return some value
- expression values coalesce to a single point. As a result, expressions can be used for nesting logic.

- Expressions allow nested logic
- if and match expressions can be nested, but it's best not to use more than two or three levels of nesting, because code can become convoluted.

34-lesson34: Demo | expressions

35-lesson35: activity | expressions:

36-lesson36: intermediate memory:
basic memory refreshes:
- memory is stored using binary. It's stored using bits. A bit is also the smallest unit of memory that can be stored.
- computer hardware is optimized for work with bytes.
- 1 byte is equal to 8 contiguous bits
- the entire memory in the computer can be thought of as one contiguous stream of bits

addresses:
- all data in memory has a memory address. These addresses are used to locate data in memory.
  Addresses are always the same, only the data at the address will change.
- Normally when you writing programs you won't utilize addresses directly, instead you let variables do all the heavy lifting for you.

offsets:
- memory offsets can be used to locate items at a specific address
- offsets always begin at 0 and they represent the number of bytes away from the original address. Similar to addresses, usually don't deal with
offsets directly, instead you'll deal with indexes and the compiler will automatically calculate how many bytes away you are from the
original address based on the index.

The variable name will automatically take care of the address and the index will automatiaclly get mapped to an offset by the compiler.
When you're working with programs, maybe each square is 10 bytes or 20 bytes, that's not sth that you need to be concerned with because the
compiler will automatically calculate that for you, to ensure that you're with the correct memory location and have the proper data.

- memory uses addresses and offsets
- addresses are permanant, but the data at the address differs
- offsets can be used to 'index' into some data, so when you have a list of items, the index can be used to access individual items within that list.*/
/* 37-lesson37: ownership:
Ownership is what allows rust to execute code in a performant manner and helps ensure that compiled code executes correctly on various circumstances.
- all proograms must track the memory usage, if they fail to do so, a memory leak occurs. A memory leak is when a program fails to track which
memory is being used and so then has to reserve new pieces of memory. All programming languages utilize their own method of
managing memory and rust utilizes an ownership model.
- in the ownership model, the owner of the memory is responsible for cleaning up the memory and an owner in rust, is simply a function.
- in rust, the memory can either be moved or can be borrowed from the owner.

Wherever you create a variable, that becomes the initial owner. For example if you create a variable in the main function, the variable is owned
by the main function and once you call another function with that data, the owner is then changed to that function and when that function finsihes, the variable
is deleted.

If we want to for example run that function twice, the compiler will throw error on the second function call, to fix this, we have to do a borrow instead of moving.

Referencing data == borrowing data

Recap: When you create a variable at a function, that function immediately becomes the owner of that variable, so that function is allowed to delete that
variable.
Fix: Now when calling the other functions, they need to borrow that variable(the owner is still the inital function that the variable was created inside of it),
now since the other functions are borrowing that variable, they're not allowed to delete it, since they're not the owner. That initial function is the owner.
Once the work of the first function call finished, the control is back to the main function and we can call the function again or call another function, by passing
a reference to that variable, to that function.
Why?
Because that variable still exists because the owner which is the function that the variable was first created in, does not deleted it yet, because we have not
reached the end of the owner function yet.

With ownership, when you create a variable within a function, that function will own it.

When we use the & symbol as one of the parameters of a function, that means that the function which receives an arg with &, will BORROW
the data and if it's borrowed, it's not allowed to deleted it, as only the owner is allowed to delete the data.

recap:
- memory must be managed in some way to prevent memory leaks
- rust uses an ownership model to accomplish memory management. The owner of the data must clean up the memory and this will automatically
occur at the end of the scope which is end of the curly braces.
- the default behavior when you calling functions is to "move" the memory to a new owner and if you just want to borrow data,
you'll use an & symbol to allow code to borrow memory, specifically to allow functions to borrow memory.*/
/* 38-lesson38: Demo | ownership
All data in a program is owned by some other part of the program.
It's the responsibility of the owner to clean up the memory and when it cleans up the memory, this is called a drop, so they're dropping the memory.
Memory gets automatically dropped, once the end of the owner's block(scope) is reached.
Once we call another function to pass the data to that function, the ownership of that variable is transferred or moved to the function we're gonna call and when
we call it, that function will be come the new owner of that variable and that function has it's own scope or block and once that block
finishes executing, the variable is going to get dropped, because the owner is responsible for cleaning up the memory.
Now once that function is executed, that variable will no longer be available later on in the program.
In order to fix this, we need to do borrowing. So instead of moving the ownership of variable into those functions that we're gonna call,
we're going to have those functions just borrow the data that they need and when a piece of data is borrowed, that function which borrowed the data, is not
allowed to drop it, because the owner is the original owner.

The reason we need to do all this, is for efficiency and memory management. If we had a data structure that was large, like several megabytes, if you were to
transfer ownership of that structure to different functions, it would require copying all that data each time you use a function.
But if you just let a function borrow it instead, then it performs much quicker, because the data stays in one place and the function
can simply borrow the data and then give it back. So there's huge performance implications to having borrowing vs transferring of the ownership.


39-lesson39: activity | ownership

40-lesson40: Demo | impl
impl allows you to implement functionality on specific enumerations and structs. This enhances the organization of your code.

When you're accessing a function within an impl block, you have to use: <name of struct or enum>::<name of function>

We can take a reference to self:

Learn: & is used for borrowing and it also is referred to as a reference. So &self means referencing self and self means whatever the struct or enum that
 you're using with impl.

The difference between Self and self is self indicatets that we ALREADY have that struct or enum, created somewhere in the program.
The Self indicates that we don't have that struct that we're using impl for it, somewhere and we're creating a new one or we're just referring to
that struct or enum as their name.
You can think of Self just as the struct or enum's name.
You can use the name of struct or enum that you're using impl for it, instead of the Self. The only issue is that if you decide to later change the name of
that struct or enum to sth else, then you gonna have to change the impl functions that are using the name of that struct or enum as Self. But if you use the Self,
you don't need to do that, after changing the name of that struct or enum.

In cold.show_temp() we didn't pass anything to show_temp() because the self is implied when you use the dot notation.

If you call show_temp() multiple times, it will run correctly, because we're borrowing self(so we take a reference to it in that show_temp() function, hence
&self), so we can use it multiple times.

So implementing functionality on structures and enumarations, provides easy ways to manage your code, because all the Temparature related stuff are in that
impl block, in one place.

41-lesson41: Activity | impl

42-lesson42: data structures | vector:
- A vector is a data structure that allows you to store multiple pieces of data. The data must be of the same type
- They are used for list of information, for example a grocery list.
- vectors allow data to be added and removed and you can also travers the entries of a vector to work with the data

Two ways of creating a vector:
let my_numbers = vec![1, 2, 3];
OR
let mut my_numbers = Vec::new();
my_numbers.push(1);
my_numbers.push(2);
my_numbers.push(3);

let two = my_numbers[1]; this is called the slice notation, so when we have those [] around an index number, we're slicing into the vector.

Macros in rust, expand to actual rust code. So the vec! macro expands into sth like approach 2 for creating a vector.

- vectors contain multiple pieces of similar data
- data can be added or removed from vectors easily
- the vec! macro can be used to easily make vectors in your code
- use for ... in to iterate through items of a vector*/
/* 43-lesson43: Demo | vectors

44-lesson44: Activity | vectors:

45-lesson45: Data types | strings:
Two commonly used types of strings:
String: owned
&str: borrowed String slice

If you want to store a string data in a struct, you must use an owned String. You can't store a slice in the struct(at least not yet).
When you want to give string data to a function, you wanna use a &str string slice(), because it's more efficient.

By default, when you create a string by saying:
Learn: let a = "a string"; , it's automatically borrowed, so it's of type &str . So we can pass it to a function easily, by passing that variable without
 using & on that variable.

If we want to create an owned string, we can do that in multiple different ways:
let owned_string = "owned string".to_owned();
let another_owned = String::from("another");
Now if we want to pass these variables to a function that accepts a string slice, we need to pass them with & . For example:
print_it(&owned_string);
print_it(&another_owned);
But we need to do:
print_it("a string slice"); // so a literal string is a reference string

If we try to store borrowed data in a struct, it won't compile, because when that struct is to be dropped at the end of the scope,
the struct is responsible for cleaning up it's own memory, however since we have borrowed memory in that struct, the struct is not allowed
to clean it up, because it doesn't own that data which is in reference(borrowed) format in there. So it results in compile error.
So for example you can not store string slice or &str in struct in this manner:
struct Employee {
    name: &str,
}

but this compiles:
struct Employee {
    name: String,
}

- Strings are automatically borrowed
- Use .to_owned() or String::from() to create an owned copy of a string slice
- use an owned String when storing in a struct*/
/* 46-lesson-46: Demo | Strings:

47-lesson-47: Activity | Strings:

48-lesson-48: Demo | Derive
We're gonna see how functionality can be automatically implemented on your enums and structs by using a derive macro.

49-lesson49: Fundamentals | type annotations:
Type annotations are required in function signatures.

50-lesson50: Working with data | enum revisited:
- enum is a type that can represent ONE item at a time.
- if a variant has some data, it is REQUIRED to has that data when you create the variant.

51-lesson51: Demo | advanced match

52-lesson52: Activity | advanced match

53-lesson53: working with data | Option(how optional data is managed using Option type)
- The Option type is a type that may be one of two things: it may be some data of a specified type or it maybe nothing.
- it's used in scenarios where data may not be required or is unavailable at the time like:
  unable to find sth
  ran out of items in a list
  form field not filled out

definition of an Option:
enum Option<T> {
    Some(T),
    None
}

Important: Normally with enums, we have to use this syntax: <name of enum>::<name of variant>, in order to actually use the variant, however, Options are
 so commonly used in rust that the variants are exposed for usage directly without typing the word Option each time. That's why we're able to
 just type in: Some(22) to represent the variants within an Option.

To access data within an option, we can do so with a match expression.

Some() and None are variants available on an Option type.

Learn: By using the `return` keyword, it allows you to EARLY return from a function.*/
/* 54-lesson54: Demo | Option

55-lesson55: Activity | Option

56-lesson56: Documentation

57-lesson57: Demo | standard library
`rustup doc` will load the rust library docs.

58-lesson58: Activity | utilizing standard library functionality

59-lesson59: Working with data | result
How work with functions that may fail under certain circumstances and how the Result data type, maybe used to handle these types of
situations.

The Result is a data type that contains one of two different types of data. It either contains Successful data or contains Error data.
It's used in scenarios where an action needs to be taken but has the possibility of failure, like copying a file perhaps the file was
previously deleted or connecting to a website.
A Result can be used in all of these scenarios to detail the error on why you'd be unable to connect to a website.

Similar to the Option type, the Ok(<value>) and Error(<value>) variants, are always available for use without using the :: to get access to them.
This is useful when you're working with the functionality that can potentially fail.

60-lesson60: Demo | result
If you want to return nothing from a function, you can use the unit type and that is just pair of parentheses.

Learn: By using the question operator, it will automatically perform a match operation and what will happen is, if the result is an OK() variant, then the inner data(the
 data that is returned by Ok() variant), will get returned by calling that function which we used question mark operator on it and if it's the
 Err() variant, then the error that is returned by calling Err(e), is gonna get automatically returned as the Err variant from the function.
So when we have:
EX)
fn pick_choice(input: &str) -> Result<(), String> {
    let choice = get_choice(input)?;
}

If get_choice() returns Ok() variant, it would be stored in choice variable but if it returns Err(e) , that error which we named it `e`, would be
automatically returned from pick_choice() function as the Err variant.

We can chain multiple function calls using the question mark operator, even though all of them may return Results that could possibly
fail, so if any one of them fails, then the function will just return automatically and we don't have to use a bunch of match blocks
in order to check each one.

To return nothing with Ok or Err variant, we can say: OK(()) and Err(())

61-lesson61: Activity | Result

62-lesson62: Activity | Result and the question mark operator
Open a18b activity file.

When you're using the question mark operator, your function must return a Result.

63-lesson63: data structure | Hashmap
- a hashmap is a collection that stores data as key-value pairs. Data is located using the key and the data itself is the value.
- it's similar to definitions in a dictionary. So you know what the key is, but you don't know what the value is at that key
- hashmaps are very fast to retrieve data when you're using a key.

When creating new hashmaps, you'll need to use mutable values, because we have to manually insert the data within the hashmap.

In hashmaps, data is stored in random order. So if you insert 1 2 3, you may get back 3 2 1, when you iterate through it. This differs
from vectors where everything comes in the same order as is placed in the vector.

- hashmaps store information as key-value pairs. The key is used to access the value.
- they're very fast to insert and find data, when you have the key.

64-lesson64: Demo | Hashmap

65-lesson65: Activity | hashmap

66-lesson66: Demo | basic closures
A closure is an anonymous(don't have name) function that you can create within your code.

Closures must always be defined within another function. It's also possible to give them a name, but it's not required.*/
/* 67-lesson67: Demo | map combinator:
Map combinator can be used to transform data.

The great thing about .map() is it will only apply, if there actually is a value there. So if we use map() and the Option<T> happens to be None,
then the .map() won't run, just like what match block will do.

One thing to know with map(), is eventually you will have to a match operation, because the return value of .map() on Option<T>, will still be an Option<U> .


68-lesson68: Activity | map combinator
We don't need to specify type annotation with closures, because the compiler is able to figure out the type on it's own, in addition, the compiler
can also figure out the return type of .map() .

69-lesson69: Demo | option combinators
These combinators allow you to manipulate data and manage options.

Difference of or_else() and unwrap_or_else(), is that or_else() still returns optional data, so it needs to return
Some() or None and with the returned data, since it's optional data, we'll still need to match on later.
But when we use unwrap_or_else() , it will take out the data and then place it within the storing variable. So that variable will no
longer have optional data(hence the word `unwrap`, so this function will unwrap the Some(x) to x itself, or in case of None, it will return the
specified default value).
So if we started with some optional data and then call unwrap_or_else() , the original data will be placed in that variable that we want to
store data. If the thing that we're calling unwrap_or_else() on it, has no data, then the default data that we specify in unwrap_or_else()'s closure
will be returned.*/
/* 70-lesson70: Activity | option combinators
TODO

71-lesson71: Demo | iterator
In second plus_one , we HAVE TO specify type annotation because .iter and .collect() operate generically on any kind of data structures. Therefore
we need to tell collect() that we're working with a Vec and we can do so with type annotations(there are other ways to do it, but type annotations
is one of the simplest ways).

By saying: Vec<_> , the compiler will figure out the type of underscore.

Learn: With filter() , whenever you return true, for a particular element, then we will keep that value and if we return false, it will remove
 that value.

The reason .last() returns an Option, is because it is possible to create an empty vector.

The key takeaway with iterators is that they do not actually execute anything, it's just a configuration step.
For example when we have:
ex)
 let count = numbers
        .iter()
        .take(3)
        .collect();
Till .take(3) , we're just configuring. Then as we call .collect() , now collect() will check that configuration.

72-lesson72: Activity | iterator

73-lesson73: demo | range
ranges are useful when you need a list of consecutive values, where you can alternatively collect them into a vector and
then try to create an iterator and then use .filter() .map() and ... the values.

74-lesson74: demo | if let
If there was a situation where you only cared if there was some data and 'if there was nothing', we don't care what happens,
we can use the if let construct.

With enums, if you're ONLY interested in one specific variant of that enum, we can use if let to make it a bit shorter. But if you do want to handle
the other variant, you can use else statement, but in these situations, it's better to use match statement.

75-lesson75: demo | while let*/
/* 76-lesson76: demo | modules

77-lesson77: activity | inline modules

78-lesson78: demo | testing
When you're writing test, it's a good idea to test sth that's on the extreme end(start and end?) of the spectrum. This means
if your function deals with numbers, you would wanna try the smallest possible number and the biggest possible number and then it's safe
to assume that everything in between will probably work just fine.

79-lesson79: activity | testing
activity 22

80-lesson80: demo | external crates

81-lesson81: activity | external crates

82-lesson82: managing code | external creates
- external modules allows code to be compartmentalized

External modules can either be a directory or a file. When using a directory as a module, it must contain the file `mod.rs` and it can optionally contain
additional modules.

Look at the pic-1 .

The name in [lib], is the name that we use when accessing the module through code.
The path is where the module file is located relative to the root of the project.
In pic-1 you see that the project structure is split into two directories: bin and lib.
All rust source files in the bin directory will be compiled as individual binary files, having the same name as the source files.
In that pic, we'll have one binary compiled into a file named app .

mod.rs files are used to indicate that a directory contains modules. The content of mod.rs determines how and if other code can access
the modules in a directory.

For example the coded directory can not be a module without the mod.rs file.

Omitting pub restricts access to only the containing module and sub-modules.

83-lesson83: activity | external crates
a26c.rs

The first thing we need to do to create external modules, is to create a library crate. Whenever you have a binary project,
in order to create a library crate, you need to create some source files and modify your cargo.toml by adding a key named [lib] and then add name and path
there.
path in [lib] of cargo.toml means where we're gonna store the code for that external crate.
activitylib.rs will be our external crate or external module.

The activitylib.rs actually just allows us to make additional modules. So the activitylib.rs is a module itself, it just defines
a crate which contains modules, although you can put functions directly into that crate itself too.

Now that we created our crate(activitylib.rs) and our two modules, we can go to a26c.rs and start move things around.

84-lesson84: demo | user input
io has so many possibilities of things going wrong, that there's a special Result provided by the io module that automatically has the
error type defined for you. So you don't need to include the error type when you're using an io::Result<> , because the error type
is already provided. So we're only providing the successful data type or the Ok in this case and it's gonna be a String.

A buffer is some space set aside that some other functionality can use and operate with. In our case, we're creating a buffer that is a String buffer.

Buffers have to be mutable, since we're gonna modify the buffer later on.

By saying: io::stdin().read_line(&mut buffer)?; , it means that .read_line() is gonna read a line and then save that
line into that buffer that is borrowing.
Important: The ? at the end of that line indicates that that function may possibly fail. If that fails, we will return an error from the function
 and the data will not have been read from the terminal. However if the data is read properly, then it will be automatically available
 within buffer variable. Then we return that buffer by using Ok() .

When you're doing a read_line() function, when you press enter, the enter is actually included as part of the data and we don't want to include it, so we use
.trim() and trim() simply trims any whitespace.*/
/* 85-lesson85: activity | user input

86- project

87-lesson 87:

88-lesson 88:

89-lesson89:

90-lesson90:

91-lesson91:
Let's use a HashMap for inner field of Bills struct instead of Vec.

92-lesson92:
Let's implement stage 3.

The question mark operator only works if the containing function also returns an Option. But in our case, since the question mark operator is being used in
the main function, it doesn't return anything, so we can not use the question mark operator there. To solve this, we're gonna create a new
function that returns an Option and then just move everything into it, that way we can utilize the question mark operator.

93-lesson93: fundamentals | traits
- traits are a way to specify that some functionality exists
- they are used to standardize functionality across multiple different types. Standardization permits functions to operate on multiple different
  types. The end result is that you end up with code de-duplication.

Normally with functions you would need to write multiple functions if you wanna work with multiple different types and if those types all exhibit some similar
functionality, then you can use a trait instead. So you would only need to write one function that operates upon a trait and then you can call
that function with multiple different data types as long as they all implement that trait.

- traits define similar functionality for different types
- trait functions are just regular functions and they can accept arguments and they can return values
- in general, you almost always want to have a reference to self in your trait functions
- you can use `impl Trait` as a function argument to pass data via traits

94-lesson94: demo | traits
We have shared functionality with via a trait.

95-lesson95: activity | traits
see a25.rs .

impl Perimeter for Square(square is a struct) means implement the Perimeter functionality for Square structure.

96-lesson96: demo | Default trait
The Default trait is used to create new structures and enumerations with a default value.

Important: The reason you would want to implement Default trait, is because there are other crates along with code within the standard
 library that will attempt to use default when applicable. In general, it's a good idea to implement Default for any struct or enum where it would
 make sense to have a default value, because it only makes your code easier to use.
 If you have defined a new() function for your struct or enum, that does not have any arguments, then you'd want to use Default trait, instead of new() .
 However in lesson-96.rust , since we can specify the weight argument for new() when we create a new Package, it makes sense to have a new() function
 and a default() function, where the new() function specifies the weight and the default() just gives you a pre-determined weight.

97-lesson97: Shared functionality | generic functions
Generic functions allow you to write functions that work with multiple types of data.
- These functions allow multiple different data types to be used as a function parameter. Instead of providing a data type as you normally would as a
  function parameter, you provide a trait. The function can then be used with any type of data that has an implementation of that trait.
  This can be done because traits exhibit behaviors.
- a generic function can then operate on the behavior defined on that trait instead of an explicit data type.

Rusts type system guarantees that any type of data used as an argument to a generic function, implements the trait required by the function.

So generic functions allow you to write a single function that can be applied to multiple types of data, as long as they implement a trait.

The things within <> , define the generic trait bounds. Those are also called generic constraining, because we're constraining
the function to only the traits that are specified within those <> .

Syntax 1 for having the parameters to implement a trait:
fn my_func(param1: impl Trait1, param2: impl Trait2) {}

Syntax 2: Used with one or two generic types that only require one or two traits:
fn my_func<T: Trait1, U: Trait2>(param1: T, param2: U) {}

Syntax 3: Once you have two or more traits or two or more generic types
fn my_func<T, U>(param1: T, param2: U)
where
    T: Trait1 + Trait2,
    U: Trait1 + Trait2 + Trait3,
 {}

Recap:
fn func(param: impl Trait) {}
fn func<T: Trait>(param: T) {}
fn func<T>(param: T) where T: Trait {}

The third syntax is always more clear once you have multiple trait bounds on a single generic type.

A compilation process where generic function expands to multiple function that are defined for each trait that we implement for the structs or enums
to be used in the generic function, is called Monomorphization.*/
/* 98-lesson98: demo | generic functions
The first thing we need to utilize generics, is a trait that we can implement for multiple different data types.

When we have: fn process_item<T: CheckIn>() {}
it means T must implement CheckIn trait.

Everytime you create that process_item() function and run it with different pieces of data, you'll increasae the
size of your binary, however, your code will run extremely fast, because each type of data knows exactly where the function is located in the process memory.


99-lesson99: Activity | generic functions
a29.rs

100-lesson100: shared functionality | generic structures
- generic structures allow you to store any type of data within a structure.
- trait bounds restrict the type of data the structure can utilize. These trait bounds are also known as generic constraints.

EX)
If we write:
trait Seat {
    fn show(&self);
}

struct Ticket<T: Seat> {
    location: T
}
fn ticket_info<T: Seat>(ticket: Ticket<T>) {
    ticket.location.show();
}
let airline = Ticket {location: Airline::FirstClass};
let concert = Ticket {location: Airline::FrontRow};

after expanding will be:
struct AirlineTicket {
    location: AirlineSeat
}
struct ConcertTicket{
    location: ConcertSeat
}
fn airplane_ticket_info(ticket: AirlineTicket{
    ticket.location.show();
}
fn concert_ticket_info(ticket: ConcertTicket{
    ticket.location.show();
}

so we end up with 2 data structures, This process happens automatically by the compiler. However, the implications of this occurring are larger in
comparison to generic functions.
If we do place those created tickets in a vector, although they're of type Ticket, we get an error and the error says that we have a mismatched type.
Learn: This is because the generic structure is actually multiple different non-generic structures and as we know, we can not place
 two different types into the same vector using this technique.
 So we can not mix generic structures in a single collection.
Generic structures expand to structures of specific type and you can not mix different types within one type of collection.*/
/* 101-lesson101: generic structures | impl blocks
When we implement functionality on generic structures, we have two options: 1) generic implementations 2) concrete implementations
- generic implementations allow functionality to be added for any type that can be used with the generic structure
- concrete implementation allow functionality to be added to the specific type indicated as part of the implementation. Concrete implementations
  are still restricted by the types that can allow be used with the generic structure.

Important: When we have: impl Game for BoardGame {} you read: BoardGame implements the Game trait.

EX) concrete implementation
struct PlayRoom<T: Game> {}
impl PlayRoom<BoardGame> {
    pub fn cleanup(&self) {}
}
this means we're implementing the PlayRoom type, only when it contains a BoardGame

EX) generic implementation
struct Name<T: Trait1 + Trait2, U: Trait3> {
    field1: T,
    field2: U,
}

impl <T: Trait1 + Trait2, U: Trait3> Name<T, U> {
     |                             |     |    |
     -------------------------------     -----
            generic specification       generic types

    fn func(&self, arg1: T, arg2: U) {}
}
This is the syntax for implementing functionality that is generic over any type is similar to the generic specification on the struct itself.

- concrete implementations only apply to the type indicated in the angle braces
- generic implementations apply to all types that also implement the indicated trait
two syntax's are available for generic implementation blocks:
impl <T: Trait1 + trait2, U: Trait3> Name<T, U> {
    fn func(&self, arg1: T, arg2: U) {} // using T and U here as function parameter types is optional within the implementation block
}
or:
impl<T, U> Name<T, U>
where
    T: Trait1 + Trait2
    U: Trait3
{
      fn func(&self, arg1: T, arg2: U) {} // using T and U here as function parameter types is optional within the implementation block
}

102-lesson102: demo | generic structures

103-lesson103: activity | generic structures
a30.rs
If a trait doesn't have any functions defined in it, we're just using that trait as a marker, so it is called a marker trait and it's just to signal to
the compiler that sth must implement that trait.

104-lesson104: fundamentals | advanced memory
- all data has a memory address and these addresses determine the location of data in memory
- offsets can be used to access adjacent addresses and these are also called indexes/indices . So this means we can access
data before or after the memory address by using an offset.

There are two ways that memory is managed by system:
1) stack: Stacks have their data placed sequentially, so everything is packed right next to each other. There's a limited amount of
space available on stack. When writing programs, all variables you create are stored in stack. This does not mean that
all the data is on the stack, just variables. Stacks are super fast to work with because they use offsets to access data which just requires
adding or subtracting a number from the memory address.

Knowing the size of the data a head of time is crucial, that way we can utilize offsets. This is why vectors only allow a single type to be contained
within. Because it uses offsets to jump between data and if everything is the same size, then it can quickly and efficiently jump
to any element within the vector.

It's only possible to remove items from top of the stack. If we remove some bytes from inside the stack and not top of it,
then we would have empty space in there and we couldn't reclaim it, because we're not able to move things around, we can just remove from top and add to top.
So if you want to remove sth from among the stack, you need to remove all the bytes from top of the stack till there and then do your specific task.

2) heap:
The other way memory is managed is by using a heap. With the heap, data is placed in memory algorithmically. This makes it slower than stack, since the
address needs to be calculated. However the tradeoff is we get unlimited theoretical space. In reality, this is limited by your hardware.
The heap uses pointers to point to where the memory is. Pointers are fixed size depending on the architecture of the computer.
64bit PCs have a pointer size of 64bits for example and the rust data type for pointer is usize.
Vectors and hashmaps are stored on the heap, along with any dynamically sized collection. This is done because a heap can
edit memory anywhere random while stack is limited to adding or removing from the top of the stack.

Learn: To access heap data, first the pointer must be read, then get the memory address and then follow that pointer to the actual memory in the heap. This process is
 called de-referencing.
Once the pointer is de-referenced, then the data to which it is pointing to, can be accessed.

If a vector(the things that can be stored on heap) runs out of space, the memory can be copied elsewhere and the pointer on the stack can then be updated to
point to the NEW heap location of that memory.
Now see the picture: if we wanted to add two more elements to that orange vector, currently, we only have room for one. So what will happen is,
that entire vector(with F00), will get copied somewhere else that is not yet occupied and then the orange pointer in that stack will be updated
to point to that new location and then you'll have more memory available to increase the size of your vector.

Note: As you can see the green boxes and then the empty boxes after the third green box is the memory available for that green vector or hashmap, currently and if
more memory were needed, the entire of those boxes will get copied to other place and the pointer of it will be updated.

To put data on the heap, we use Box::new() which box up the data and then it will be moved on to the heap.

EX)
struct Entry {
    id: i32,
}

fn main() {
    let data = Entry {id: 5};
    let data_ptr: Box<Entry> = Box::new(data);
    let data_stack = *data_ptr; // move the data back to the stack(de-reference the data using *)
}

When you get an error like: doesn't have a size know at compile time, the reason we get this error is because the size of the object
may be different when you run the program and it's unable to store it in the stack. Therefore, the fix would be
to Box the object and store it on the heap. That way the stack will just have a pointer which is always usize

stack:
- it has sequential memory addresses
- used for all variables in the program
- they do have a limited size
- must know data size(size of data) ahead of time, that way you can properly utilize the offsets

heap:
- memory addresses are algorithmically calculated
- used when you're working with large amounts of data
- it has theoretically unlimited size
- you can store dynamically sized data or data where you don't know the exact size(dynamically sized data/unknown sized data)*/
/* 105-lesson105: shared functionality | trait objects
Trait objects offer a way to dynamically change program behavior aat runtime.
- are dynamically allocated objects. You can think of them as generics. These types are calculated when your program runs instead of when you
  compile your program. Determining these information at runtime is called dynamic dispatch. Contrast this to generics, which determine the type at
  compile time, using a process called static dispatch.

- one benefit of using trait objects, is mixed type collections. Normally collections require that each item be the same type, but
  with trait objects, each one can have a different type and exist in the same collection. This makes it easy to work with data that has
  similar purposes. For example we can place Employees, Managers and Supervisors into a single vector.

- using trait objects in this way enables polymorphic program behavior. Since we can have a vector with multiple different types, we can create a
  program that behaves differently for each type in the vector.

- trait objects also work well with rapidly evolving program requirements. If we create a new structure, we can turn it into a
  trait object and just use it with other trait objects. For example, we could add a Contractor as a trait object and bundle it with
  Employees, Managers and Supervisors and then perform some actions with all of them.

- These benefits has a drawback. Dynamic dispatch has small performance penalty in terms of speed and memory usage.

To create trait objects, we're gonna make heavy use of type annotations.

Creating a trait object:

trait Clicky {
    fn click(&self);
}

struct Keyboard;

impl Clicky for Keyboard {
    fn click(&self) {
        println!("clicked");
    }
}

fn main() {
    // first way for creating a trait object
    let keeb = Keyboard;
    let keeb_obj: &dyn Clicky = &keeb; // here we're creating a trait object

    // second way for creating a trait object(immediately borrow the structure that you created)
    let keeb: &dyn Clicky = &Keyboard;

    // third way
    let keeb: Box<dyn Clicky> = Box::new(Keyboard);
}

Usually we don't use the second approach, we use first or third one. Because in the second way, storing a reference
into a vector, isn't always useful depending on what you're trying to accomplish.

When we put sth into a Box<> , it let's us move things around.
So far, we've been borrowing everything by using & and those become problematic when you try to use them in different scenarios.
However if you put it in a Box, you can move it around wherever you want.

There's another way to create trait objects and that is to not even include any type annoations. Now it may seem not working at first, but we DO have
type annotations in the function parameters:

Trait object parameter - BORROW:
fn borrow_clicky(obj: &dyn Clicky) {
    obj.click();
}

fn main() {
    let keeb = Keyboard;
    borrow_clicky(&keeb);
}
So as long as you have tpye annotatiosn somewhere in your program, then rust will automatically convert it into a trait object, when you try to use it.
So this fourth way is the way you want to use trait objects, that way you don't need to annotate every single one as a trait object. So you just annotate
the target of the trait object or whatever is using it or the collection, you annotate that with dyn keyword and then anything going into that parameter,
will become a trait object or in this case, &dyn Clicky.

The same thing applies with Boxes:
Trait object parameter - MOVE:
fn move_clicky(obj: Box<dyn Clicky>) {
    obj.click();
}

fn main() {
    let keeb = Box::new(Keyboard);
    move_clicky(keeb);
}
In this example, we don't need any type annotations when declaring the keeb variable, because we have the type annotation in the target function(function that
receives a trait object).

If you want to move trait objects, you'll put them in Boxes and if you want to borrow trait objects, you can just borrow them directly, you don't need to put
them into a Box.

How to create a heterogeneous vector:
Learn: A heterogeneous vector is a vector that contains multiple DIFFERENT types instead of just the one type per vector.
struct Mouse;
impl Clicky For Mouse {
    fn click(&self) {
        println!("clicked");
    }
}

fn make_clicks(clickeys: Vec<Box<dyn Clicky>>) {
    for clicker in clickeys {
        clicker.click();
    }
}

fn main() {
    /* first way, annotating each variable: */
    let keeb: Box<dyn Clicky> = Box::new(Keyboard);
    let mouse: Box<dyn Clicky> = Box::new(Mouse);
    let clickers = vec![keeb, mouse];

    /* second way, annotate the vector and then anything that's boxed, will attempt to be turned into a dynamic Clicky trait object:  */
    let keeb = Box::new(Keyboard);
    let mouse = Box::new(Mouse);
    let clickers: Vec<Box<dyn Clicky>> = vec![keeb, mouse];

    make_clicks(clickers);
}
This was the common use case for trait objects which is just placing them all into a vector and then iterating through each one and performing some action
that will be different for each one.

The second approach is easier.

- trait objects allow for composite allocations
- they are slightly less performant than using generics and that's because they get calculated at runtime instead of at compile time
- use the dyn keyword when working with trait objects, if you forget the dyn keyword, you'll get a warning, but that's gonna become
a hard error in a future version of rust
- trait objects can be borrowed using a reference or they can be moved using a Box. Usually you want to use a Box when you store objects in a vector,
otherwise , you'll start running into multiple errors and it usually just makes more sense to store Boxes in a vector that way the vector can OWN all the data.
*/
/* 106-lesson106: demo | trait objects:
Trait objects allow you to store multiple different types of data in a single collection.

Learn: In &Vec<Box<dyn Sale>> , we have to put the Sale trait into a Box, because when we're working with a Sale, it's a trait, it's not an actual type,
 traits don't have sizes so they can't properly be calculated in order to layout in memory correctly. However, when we place it within a Box<> ,
 we know that a Box is a usize, because it's a pointer, so we're able to store any amount of pointers within the vector, because they're all the same
 size.

Using trait objects in this way, allow us to combine multiple different structures into a single vector and then perform an action on all of the
different structures to get the result that we wanted.

107-lesson107: activity | trait objects:
a31.rs

If you use curly brackets instead of semi-colon, you're providing a default implementation of the function of a trait.
When we have a semi-colon at the end of function in a trait, that means that the implementor MUST implement that function, but if we
provide {} , we can implement a default function that all of the implementors can use, but they CAN override that default implementation if they want.

Learn: Traits can only access data through functions that are implemented on a structure. Traits are unable to access any fields in a structure,
 because each structure might have a different field. So we have to rely completely on functions to get any values.

Important: When we have: &Vec<Box<dyn Material>> , it means we can put different types of structures within that vector, because we're
 boxing that data and a box always has the usize type, and usize is always the same, so it's no problem to put it into a vector.
 Remember we have to have the dyn keyword so that rust knows that we're working with dynamic dispatch.

In our case, the vector or the materials variable, needs to be annotated in order to use dynamic dispatch.

108-lesson108: ownership | lifetimes
Lifetimes are a way to specify to the compiler, how to handle borrowed data and this will enable you to store borrowed data in structures and return borrowed
data from functions.

Ownership review:
- data in rust programs must have an owner and owner is responsible for cleaning up data. This is how rust handles memory management.
- by default, data can only have one owner. It is possible to have multiple owners.
- functions, closures, structs, enums and scopes are owners of data. So whenever you store data in a structure or send data to a function or closure,
those become the owner of that data
- data can be moved from one owner to another, through assigning variables and through calling functions and now it's up to them to cleanup that moved data.
- it is also possible to borrow data from an owner although the owner still needs to clean up the data. Borrowing is done using the & symbol and the
code borrowing the data is never allowed to cleanup that data.

lifetime:
Lifetimes are a way to inform the compiler that borrowed data will be valid at a specific point in time.
Lifetimes are needed for:
- storing borrowed data in structs or enums
- returning borrowed data from functions

- all data has a lifetime, but the compiler is able to automatically calculate the lifetimes in many cases.
- 'static data stays in memory until the program terminates

Learn:
 struct RobotArm<'a> {
    part: &'a Part,
 }
 in this example, the `a is telling rust that Part exists BEFORE the existance of RobotArm, therefore we're able to access it.

Lifetime annotations indicate that there exists some owned data that:
- "lives at least as long" as the borrowed data. So let's say you create a struct and you want to borrow from it, you'll have to ensure that the
structure lives at least as long as that borrow, in order to use it. Because if we delete the struct, then you can't borrow from it because
it's been deleted.
- "outlives or outlasts" scope of a borrow. So if you create some data and you create a new scope, then within that scope you can borrow as much as you want.
Once that scope ends, your borrow is destroyed, but the data you create before the scope, still lying around. So that's totally ok to do.
- "exists linger that the scope of a borrow. For example you create a configuration struct in the beginning of your program and then later in your
program, borrow from it without issue or store borrowed data from it, no problem. Because that config struct exists longer than scope.

structures utilizing borrowed data must:
always be created after the owner was created and they must always be destroyed BEFORE the owner is destroyed. Because you're unable to borrow from sth, if
it's been destroyed.

Lifetimes allow:
Borrow the data in a structure and they allow return references from functions

Lifetimes are the mechanism that tracks how long a piece of data resides in memory.
Lifetimes are usually elided, but they can be specified manually when needed
Lifetimes are checked by the compiler*/
/* 109-lesson109: demo | lifetimes
Whenever you have lifetime annotation on a struct, you'll also need to include them on the impl blocks of that struct and to do that,
they have to exist directly after the impl keyword and after the name of Struct that you're implementing it.

110-lesson110: activity | lifetimes & structures
a32.rs
Learn: 'static indicates that the data lives as long as the program is running. So we could access that at any point in the program.

So whenever you have lifetime annotations, the data has to exist BEFORE the struct or ... .

111-lesson111: activity | lifetimes & functions
a33.rs

What happens when we return a borrowed value from a function, is rust needs to know where it came from(if we had more than one parameters for function).

112-lesson112: fundamentals | custom errors
Why you would want to use custom errors:
- functions may fail in more than one way
- useful to communicate the failure reason
- when you're creating custom errors, you'll generally want to use error enumeration, because enums make it easy to add new errors and they can be easily
  matched upon later.

Having a proper error type has a few requirements:
- you need to implement the Debug traits which can be derived(in other words, we can use the derive macro, so this one is super easy!). The Debug
trait is for displaying "what happened" in the context of development and debugging.
- implement the Display trait. The Display trait is considered a user facing trait. So the message should reflect this.
- implement the Error trait itself, so we can get interuptebility with other errors. Implementing the Error trait can actually be done with an empty
  implementation block, because the trait has default implementations.
  So writing impl details is actually optional, because the trait has a default implementation for everything needed.

Manual error creation:
#[derive(Debug)]
enum LockError {
    MechanicalError(i32),
    NetworkError,
    NotAuthorized,
}

use std::error::Error;
impl Error for LockError {} // with empty curly brackets, it means we want the default that is already part of the trait

now implement the Display trait:
use std::fmt;

impl fmt::Display for LockError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            LockError::MechanicalError(ref code) => write!(f, "mechanical error: {}", code),
            LockError::NetworkError => write!(f, "network error"),
            LockError::NotAuthorized => write!(f, "not authorized"),
        }
    }
}

Do these automatically with thiserror crate:
use thiserror::Error;

#[derive(Error, Debug)]
enum LockError {
    #[error("mechanical error: {0}")]
    MechanicalError(i32),
    #[error("network error")]
    NetworkError,
    #[error("not authorized")]
    NotAuthorized,
}

How to use this in a program:
fn lock_door() -> Result<(), LocKError> {
    // ... some code ...
    Err(LockError::NetworkError)
}

Error conversion:
use thiserror::Error;

#[derive(Error, Debug)]
enum NetworkError {
    #[error("Connection timed out")]
    Timeout,
    #[error("Unreachable")]
    Unreachable
}

enum LockError {
    #[error("mechanical error: {0}")]
    MechanicalError(i32, i32),
    #[error("Network Error")]
    Network(#[from] NetworkError),
    #[error("Not Authorized")]
    NotAuthorized,
}

Pro tips: Do's:
- prefer to use error enums over strings.
  Enums, concisely communicates what went wrong with the function and they are checked by the compiler, so you can't mess up.
  In addition, the variants can be matched on.
  strings are OK when prototyping or you don't know exactly what kind of error you should generate, or if the problem domain
  isn't fully understood, strings are fine to use.

- keep errors specific. Basically error enums should either be module specific or function specific. When working with modules,
  all the functions within, should be related and should theoratically mostly have the same type of errors. Make an error that
  covers things that go wrong within that one module, would make sense, since all the functions should have same semantics.

some functions may have many error conditions, or an error that is just out of place when compared to the rest of the surrounding code. It's better to
make dedicated errors for individual functions, instead of trying to make it fit in some module level error.

- try to use match as much as possible.

More pro tips: Don'ts:
- never put unrelated errors into a single enum.
  + If you do this, as the problem domain expands, eventually the enum will become unwieldy, because it
    would have every error in it and matching would become problematic.
  + Since enum variants get checked across the entire codebase, if you put all the errors into a single enum, you'll have to update code that never
    handles a specific error, just because it happens to utilize the enum. This makes updating errors difficult.
  + also lots of error variants in a single enum, make it impossible to know how a function can fail. This means you'll either
  spend a lot of time reading the functions you don't need to, or matching on errors that will never happen.

- custom error enums communicate exactly what went wrong in a function
- errors require three trait implementations:
  + Debug trait (can easily be derived)
  + std::error::Error (empty impl block)
  + Display (manual or crate)
- use the thiserror crate to easily implement all required traits
- keep error enums module or function specific.
  + don't put too many variants in one error*/
/* 113-lesson113: Demo | custom errors */

