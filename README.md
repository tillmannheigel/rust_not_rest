# Rust not rest

There is a german saying, if "you rest you will rust". To not rest and therefore rust, this is a apprentice piece for understanding a little bit more about Rust. 

Now to some more Rust Language related topics...


## Ownership

One of the core features of Rust ist ownership. In contrast to garbage collection (e.g. Java) or explicit memory allocation and de-allocation, Rust manages the computer's memory through a system of ownership rules which are checked at compile time. 

### Stack and Heap

The stack as part of the memory follows the LIFO (last in first out) principle, and must work with data of a fixed size. Data with an unkown size or possibly changing size, must be stored on the heap. As the heap is less structured (because of the dynamicness), it is slower than pushing data onto the stack.  

### Ownership Rules

* Each value in Rust has a variable that's called its owner.
* There can be only one owner at a time. 
* When the owner goes out of scope, the value will be dropped. 

(src: https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html#ownership-rules)

### Variable Scope

```
let s = "my_str';
```

 


## Usage

```python
import foobar

# returns 'words'
foobar.pluralize('word')

# returns 'geese'
foobar.pluralize('goose')

# returns 'phenomenon'
foobar.singularize('phenomena')
```

## License
[MIT](https://choosealicense.com/licenses/mit/)
