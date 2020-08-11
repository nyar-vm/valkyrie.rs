



```vk
using {
    ~ wow!
    `python`.torch as tf
}

using lib1.*
using lib2::*
using lib3 as z
using lib4 as _
using lib5.{
    a as b
    c as d
    e::f::{
        g as h
    }
}
using torch
using numpy as np
using pandas.database as db

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