# Refactoring Notes

## Refactor One: To Generics

I feel like this has been a qualified success. The big problem is that it reveals the underlying 
issues with the structs themselves. They're too heavy. There's no reason they can't be light. 

## Refactor Two: Simplification

GOAL: I want my structs to be copyable. Heavy stuff should come through function calls, not stored automatically. 

The big lesson I've been learning from doing this refactoring, and more importantly in adding all of the 
doctests, os that they structs are too clunky. They are a pain in the ass to instantiate. 

# Jan 14th 2025

The IDE is so slow with the dupes in code and having added so many doc tests. I need to get to a point where I can
trim it all down.

# Jan 15th 2025

TODO: Possible REFACTOR - Add symbol char to struct to avoid need for localization
call. We will save this for after REF2 is complete.

## Removing ::new()

I've decided to remove the Rank and Suit ::new() instantiation function and just use ::new(char). 
This feels more idiomatic to me. 

