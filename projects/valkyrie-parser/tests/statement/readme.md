



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