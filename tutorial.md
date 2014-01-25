My presentation of Lisp is a rip-off of Paul Graham's article "The Roots of Lisp." You might enjoy taking a look at that.


# Lisp expressions

A **Lisp expression** is either:

 - a sequence of letters, called an **atom**
 - or, a **list** of zero or more expressions

Examples of atoms:

 1. foo
 2. WashingtonDC

Examples of lists:

 1. ()
 2. (foo)
 3. (foo bar)
 4. (Stacks (of FOO) for everyone)
 5. ((((Bah))) (HUMBUG))

In the first list example, we have a list of zero expressions, commonly called the *empty list*. The fourth list example is a list of four expressions, the second expression being a list itself. You should see how lists are capable of nesting to arbitrary depths.


We need a data structure to represent Lisp expressions. We will use a Rust enum type, which fits quite nicely:

    enum Expression {
        Atom(~str),
        List(~[Expression]),
    }

`Atom` and `List` are the *constructors* of the `Expression` type. It means that when we do `let x = Atom(~"foo")`, then `x`'s type is `Expression`. So `Atom` and `List` actually functions that return `Expression` values.

The `List` constructor is obviously a bit funky since it takes as a parameter a vector of `Expression` values. But `List` is supposed to be part of the definition of `Expression`, so that's clearly impossible, right? Well, no, it's the same as our definition of Lisp expressions, which were either atoms or lists of Lisp expressions.

Here are the corresponding representations of the above examples in Rust:

 1. `Atom(~"foo")`
 2. `Atom(~"WashingtonDC")`

and

 1. `List(~[])`
 2. `List( ~[ Atom(~"foo") ] )`
 3. `List( ~[ Atom(~"foo"), Atom(~"bar") ] )`
 4. `List( ~[ Atom(~"Stacks"), List( ~[ Atom(~"of"), Atom(~"FOO") ] ), Atom(~"for"), Atom(~"everyone") ] )`
 5. `List( ~[ List( ~[ List( ~[ List( ~[ Atom(~"Bah") ] ) ] ) ] ), List( ~[ Atom(~"HUMBUG") ] ) ] )`


# Expressions and their values (Please Excuse My Dear Aunt Sally)
You've probably spent a great deal of time learning how to simplify arithmetic expressions like `(1 + 1)`, `(20 - 8)` and `(823.76 / 9.8) * 7`. I said "simplifying", but what are we actually doing here? Consider the arithmetic expression:

    (1 + 2) * 5

This is a sequence of parentheses, numbers and arithmetic operations. What we do when we simplify such an expression is to systematically reduce it until it is a single number:

    (1 + 2) * 5
    => 3 * 5
    => 15

We can call `15` the *value* of the expression `(1 + 2) * 5`. It is the combination of the order of operations and the meanings of the arithmetic operations (+, -, *, /) that uniquely define the value of any arithmetic expression.

Except that's not true. There's are infinitely many expressions that do not have values. Here's one:

    5 / 0

Division by zero is not allowed, so this expression has no value.

Similar to the case for arithmetic expressions, some Lisp expressions have values. It is now our task to define precisely which Lisp expressions have values and what they are. 

A Lisp interpreter is little more than a machine for turning Lisp expressions into values (or possibly error messages).
