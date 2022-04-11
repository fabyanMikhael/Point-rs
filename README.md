# Point-rs
A simple library with just one struct which is used to wrap around pointers. This can be used to create pointers and share them across threads without the hassle of synchronization if you really do not care about that.

## usage

```rs
let x = 0;
let y = Pointer(&x);
println!("{}", *y);
```

```rs
let mut x = 0;
let mut y = MutPointer(&mut x);
*y += 10;
println!("{x} == {}", *y);
```
