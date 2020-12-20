





```scala


for pattern in iterator if condition {
    run1
}
otherwise {
    run2
}


let iter = iterator.into_iterator();
let mut no_run = true;
@1
loop {
    @2
    
    @5
    if iter.next().is_none() {
        @6
        goto @4
        @7
    }
    @8
    no_run = false;
    run1
    @3
}
@4

@9
if no_run {
    @10
    run2
    @11
}
@12


```



```scala
'outer: for va in &a {
    for vb in &b {
        if va == vb {
            continue 'outer;
        }
    }
    print("{va}");
}
print("end");


let ret = ();
let mut iter_a = a.into_iter();
loop {
    @1
    let va = iter_a.next();
    if va.is_none() {
        goto @4;
    }
    let va = va.unwrap();
    let mut iter_b = b.into_iter();
    
    loop {
        @2
        let vb = iter_b.next();
        if vb.is_none() {
            goto @3
        }
        let vb = vb.unwrap();
        if va == vb {
            goto @1
        }
        goto @2
    }
    @3
    print("{va}");
    goto @1
}
@4
print("end");
@5
// do drops here
ret


// alloc ret and locals
    let ret = ();
.L0:
    let mut iter_a = a.into_iter();
.L1:
    let va = iter_a.next();
    @JUMP_IF_NONE(va, .L4);
    let va = va.unwrap();
    let mut iter_b = b.into_iter();
.L2:
    let vb = iter_b.next();
    @JUMP_IF_NONE(vb, 3);
    let vb = vb.unwrap();
    @JUMP_IF_EQ(va, vb, .L1);
    @JUMP(.L2);
.L3:
    print("{va}");
    @JUMP(.L1);
.L4:
    print("end");
.L5: 
    // do drops here
    ret


```