## Class-like Data Types
Lang Version|      0.4    |    1.0   | 1.0          | 9                        | 10
   ---      | :-:         |    :-:   |   :-:        |  :-:                     | :-:
Feature     | Rust struct | C# class | C# struct    | C# record / record class | C# record struct 
inheritance |             |    x     |              |    x                     |                 
equality    | by value*   | by ref   | custom       | by value                 | by value         
pass by           |   ref**     | ref      | value***     | ref                      | value            
named or tuple-like | either | named | named | either | either 

* most common case because is one liner with #[derive(PartialEq)] attribute
** borrow checker guarantees only one mutable reference at a time, avoiding surprises
*** can be passed by reference with parameter modifiers or ref structs in C# 7.2

## Interface-like Data Types
Feature                                           | Trait | Interface 
  ---                                             | :-:   | :-------: 
method signatures that require to be implemented  | x     | x         
default implementations                           | x     | >8.0      
static  methods                                   | x     | x         
instance methods                                  | x     | x         
data (fields)                                     | _     | _         
contructor                                        | _     | _         
objects implement multiple  of them               | x     | x
used as parameters                                | x     | x
used as return types                              | x     | x
inheritance                                       | x     | _
can be generic                                    | x     | x
can be applied over generic set of objects        | x     | _

## Strings

Feature                   | Rust String | C# String | C# StringInfo
  ---                     | :-:         | :-:       | :-:
Collection of             | unicode     | chars     | unicode
## Errors

Other notes:
- C# can mark data types and variables immutable (readonly, const), Rust only variables
Control Features
- Rust makes you opt-in to mutable variables
- shadowing

Feature
cargo and rustup vs nuget and dotnet cli
test runners
iterators
closures
pattern matching

Rust Special
- macros
- concurrent code
- built-in documentation 
- variable shadowing