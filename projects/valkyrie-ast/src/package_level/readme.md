




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
    otehr,
}

let a = new Name<G>(args) {}
a[a] = 2 // indexer
a['C',4] = "Middle C" // indexer
a.`$setter$size`(Math.PI)
a.collect(Pair(0, 2)) // collector
a.collect(term) // collector
a.collect(otehr) // collector
```
