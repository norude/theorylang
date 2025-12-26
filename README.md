# Theorylang

A rust-based toy language to explore type theory, category theory and other stuff functional bros like

## Goals
Make the source code as simple as possible.
Use a parser generator.

## Backend
I don't really care, so it'll probably be an interpreter, while it is still untyped, and compiled to llvm-ir later

## Syntax
be simple, Have both postfix and prefix versions of everything

## Expressions and statements
Most things should be Expressions, but it's kinda hard to 

## Types
Types are the basic building block of abstraction and are kinda magic.
An `i32` for example is not fundumental to a language being a language,
so there should be a way to define a type on its own, in terms of bits or something.
Composing types should also be fundumental. i32*i32 is a good notation,
i32+i32 is not so good and kinda strange in terms of bits.
Maybe types really should be always defined in terms of bit value ± alignment.
Functions on the other hand, are very fundumental to a language.
But the type of a function should just have one value: itself ({is this true for generics?}).
SuperTypes (e.g traits) should then abstract over them.
Think over: Associated functions, associated values and other associated stuff and their visibility


## SuperTypes
Traits?? Or something else? Not sure. Associated items seem to be important for them

## SuperSuperTypes
Super-Typing should be infinite, but I have not the slightest idea, how this would look for stuff more abstract that Traits
