`class Name(Super): Trait {}`


## Full Form

```scala
constraint T {
    where {
        T: A
    }
}
//? documents
#[mod1, mod2]
mod3 mod4 class module::Name(Base::<T>): DeriveA + DeriveB {
    _field: int = 1;
    
    construct() {
    
    }
    
    construct(args) {
    
    }
    
    #? documents
    extract() {
    
    }
    
    destruct() {
    
    }
    
    infix `+`() {
    
    }
}

extends module::Name: TraitA + TraitB {
   
}
```


## Generics

```v
constraint A {
    alias {
        Type: <A as Iterator>::Item
    }
    where {
        A: Trait1
    }
}
class Structure<B: Trait2> {
    a: A,
    b: B,
}
```


```v
∀ A, B {
    where {
        A: Trait1,
        B: Trait2,
        <A as Trait>::Item: B
    }
}
structure Record<B> {
    a: A,
    b: B,
}
```