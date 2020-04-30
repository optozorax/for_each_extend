# for_each_extend

This crate provides method to extend vector while iterating on it.

```rust
let mut a = vec![1, 2, 3, 4];
a.for_each_extend(|x| {
	if *x.current() == 3 {
		x.push(5);
		x.push(6);
		x.push(7);
		x.push(8);
	}
});

assert_eq!(a, vec![1, 2, 3, 4, 5, 6, 7, 8]);
```