# Refactoring Notes

## Refactor One: To Generics

I feel like this has been a qualified success. The big problem is that it reveals the underlying 
issues with the structs themselves. They're too heavy. There's no reason they can't be light. 

## Refactor Two: Simplification

GOAL: I want my structs to be copyable. Heavy stuff should come through function calls, not stored automatically. 

The big lesson I've been learning from doing this refactoring, and more importantly in adding all of the 
doctests, os that they structs are too clunky. They are a pain in the ass to instantiate. 