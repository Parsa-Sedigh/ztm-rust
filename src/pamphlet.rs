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
Till .take(3) , we're just configuring. Then as we cal .collect() , now collect() will check that configuration.

72-lesson72: Activity | iterator

73-lesson73: demo | range
ranges are useful when you need a list of consecutive values, where you can alternatively collect them into a vector and
then try to create an iterator and then use .filter() .map() and ... the values.

74-lesson74: demo | if let
*/

