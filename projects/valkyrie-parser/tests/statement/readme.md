



```vk
using {
    ~ wow!
    `python`.torch as tf
}



using .mod
using ..mod as z
using lib.*
using mod::*
using "./mod" as y
using "../lib/mod"::{
	a as b
	c as d
	e.f.{g as h}
}
```



```scala
class IntegerSet {

}





class IndexersExample {
	_size: f64;
    name: String,
}

extends IndexersExample {
	set Size(value) {
		self._size = value as float
	}
	view index(i: int) {
		return self._size
	}
	set view index(i: int) {
		self._size = value as float
	}
	set view index(c: char, i: int) {
		self._size = value as float
	}
}
```