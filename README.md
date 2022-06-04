# pathtorust
<p> <img src="https://www.rust-lang.org/static/images/rust-logo-blk.svg"></p> <br>
<p> Easy learning path to Rust programming language.</p>
<div>Curated by <a href="https://twitter.com/oyewodayo"> Temidayo Oyewo</a></div>
<div>Twitter page <a href="https://twitter.com/pathtorust"> PathToRust</a> - https://twitter.com/pathtorust</div>

<hr>
<strong>Installation guide</strong>
<br>
Whatever the operating system you are using, click on <a href="https://www.rust-lang.org/tools/install">RustLang</a> and follow the steps given to have it installed on your system.
</div>

String complexity

# `_` expressions

> **<sup>Syntax</sup>**\
> _UnderscoreExpression_ :\
> &nbsp;&nbsp; `_`
Underscore expressions, denoted with the symbol `_`, are used to signify a
placeholder in a destructuring assignment. They may only appear in the left-hand
side of an assignment.

An example of an `_` expression:

```rust
let p = (1, 2);
let mut a = 0;
(_, a) = p;
```

