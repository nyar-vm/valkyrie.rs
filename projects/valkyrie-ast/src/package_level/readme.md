

```vk
type Action<..T> = Function<..T, ()>
```




venus

using


collector api

```csharp
trait Constructor {
    hidden constructor() -> Self;
}
trait Destructor {
    hidden unique destructor() -> Self;
}

class A<T> 
{
    @[get, set]
    _field: int = 1;    
}

forall A, B {
    A <: B
}
extend A: Disposable {
    forall T {
        A<T> <: A
    }
    define A() {
    
    }
    ctor Self() {}
    ctor Self(args) {}
    
    get field() {
        _field
    }
    mut field(value) {
        _field = value
    }
    set field(value) {
        _field = value
    }
    
    dispose() {} 
        
    dtor Self() {}
    
    indexer Self() {}
    
    infix `+`() {}
    
    
}


let a = new Name<G>(args) {
    [a]: 2,
    Size: Math.PI,
    ['C',4]: "Middle C",
    Pair(0, 2),
    term,
    other,
}

let a = new Name<G>(args) {}
a[a] = 2 // indexer
a['C',4] = "Middle C" // indexer
a.`$setter$size`(Math.PI)
a.collect(Pair(0, 2)) // collector
a.collect(term) // collector
a.collect(other) // collector
```


```vk
for i in [1, 2, 3] if i % 2 == 0 {
    do_something(i)
}
else {
    do_something_else()
}

let mut $iter = IntoIterator::into_iterator([1, 2, 3])
let mut $next = $iter.next()
let mut $no_run = true;
while $next.is_some() {
    let i = $next!;
    if i % 2 == 0 {
        $no_run = false;
        do_something(i);
    }
    $next = $iter.next()
}
if $no_run {
    do_something_else();
}


let mut $iter = IntoIterator::into_iterator([1, 2, 3])
let mut $next = $iter.next()
while $next.is_some() {
    let i = $next!;
    do_something(i);
    $next = $iter.next()
}
```



```vk

```

