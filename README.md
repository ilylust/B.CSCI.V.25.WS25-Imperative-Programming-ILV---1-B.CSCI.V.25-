# Theory part
A || that doesn’t enclose a variable from outside is an anonymous function. Anonymous means “doesn’t have a name.” It works more like a regular function and can be passed into places where a function is required if the signature is the same.

A || that encloses a variable from outside is also anonymous but called a closure. It “encloses” the variables around it to use them.

The core method in iterators is .next(), which returns an Option. Almost all
iterators return Some until they run out of items, and after that, None.

An associated type is the type that goes with a trait. Most traits don’t have them,
but some do.

String literals—You make these when you write let my_str = "I am a &str";.
They last for the whole program because they are written directly into the
binary. They have the type &'static str. The ' means its lifetime, and string
literals have a lifetime called static.

Borrowed str—This is the regular &str form without a 'static lifetime. If you
have a String and pass a reference to it (a &String), Rust will convert it to a
&str when you need it. This is thanks to a trait called Deref. We will learn to use
Deref in chapter 15, but, for the moment, just remember that you can pass in a
&String to a function that takes a &str.