## Tests

```vk
pub fn check_raw(list: &[u8]) {
    let mut iter = list.iter();
    let mut next = iter.next();
    while next.is_some() {
        @1
        let i = next.unwrap();
        if i % 4 == 0 {
            goto @1
        }
        else if i & 2 == 0 {
            goto @2
        }
        else {
            goto @3
        }
    }
    @2
    println!("Even!")
    @3
}
```
