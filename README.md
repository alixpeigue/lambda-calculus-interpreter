# Rust lambda calculus interpreter

This is a simple interpreter for lambda calculus

## Use

You can use the interpreter in two modes :

 - Interactive mode if you provide no arguments
 - File mode if you provide the name of a file

## Syntax

### Abstractions

You can define an abstraction over a variable this way : `\x.x+1`

This function takes a parameter and adds 1 to it.

To take multiple parameters, you can use currying : `\x.\y.x+y`


### Applications

To apply a function, you can use the following syntax : `(\x.x+1) 2` (returns 3)

The same goes for multiple arguments : `(\x.\y.x+y) 1 2` (returns 3)

By using currying, you can create partial applications : `(\x.\y.x+y) 1` is the same as `\y.1+y`

Arguments are evaluated before being passed in the function, even if the function doesn't use them

Example : in `(\x.2) 1+2`, `1+2` is evaluated.

#### Functions can be passed as parameters :

`\f.\x.f x` takes two parameters : a function f and a value x, then applies f to x.

Example : `(\f.\x.f x) (\x.x+1) 1` is equivalent to `(\x.x+1) 1` which returns `2`

### Conditionals

You can make a conditional using the ternary operator syntax :
`cond ? true_branch : false_branch`
example : `8>5 ? 1 : 0`

Expressions not are not evaluated if they are in the branch that doesn't correspond to the condition.


### Recusion

Despite all functions being lambdas (anonymous), you can create recursive functions by using a fixed point operator.

The interpreter being eager, you can use the eager version of the Y-combinator : `\f.(\x.f (\.v.x x v)) (\x.f (\v.x x v))`

Example for creating a recursive fibonacci function :


`(\f.(\x.f (\v.x x v)) (\x.f (\v.x x v))) (\f.\x.x<2 ? 1 : (f x-1) + (f x-2))` 

Calculating fib(25):

`(\f.(\x.f (\v.x x v)) (\x.f (\v.x x v))) (\f.\x.x<2 ? 1 : (f x-1) + (f x-2)) 25` (returns 121393)

### Priorities

Operator priority :
 - parentheses
 - `*` and `/`
 - `+` and `-`
 - `=` and `!=`
 - `>`, `<`, `>=`, `<=`
 - Application
 - Conditional
 - Abstraction

Arithmetic and conditional operators of same priority are done from left to right, for example : `a+b-c` will be executed as `(a+b)-c` and `a=b!=c` as `(a=b)!=c`

Applications are done left to right so for example `a b c` is the same as `(a b) c`

Application have higher priority than conditionals so `a?b:c d` is the same as `a?b:(c d)`

Applications have higher priority than abstractions so `\f.\x.f x` is equivalent to `\f.\x.(f x)`


## Errors

No type checking is done when parsing the program, so for example, `1+1 2` is valid code, but will throw a runtime error because `1+1` isn't a function.
