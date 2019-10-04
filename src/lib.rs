///
/// # Recursive Fibonacci
///
/// ```
/// use rust_fibonacci::recursive_fibonacci;
///
/// assert_eq!(8, recursive_fibonacci(6));
/// assert_eq!(2, recursive_fibonacci(3));
/// ```
///
pub fn recursive_fibonacci(n: usize) -> usize {
    print!("{}", n);
    match n {
        0 => panic!("zero is not a right argument to fibonacci_reccursive()!"),
        1 | 2 => 1,
        _ => recursive_fibonacci(n - 1) + recursive_fibonacci(n - 2)
    }
}


///
/// # Recursive Fibonacci with Memoization ( Dynamic Programming )
///
/// ```
/// use rust_fibonacci::dynamic_programming_recursive_fibonacci;
///
/// assert_eq!(2971215073, dynamic_programming_recursive_fibonacci(46));
/// ```
///
pub fn dynamic_programming_recursive_fibonacci(number: usize) -> usize {
    fn fib_memo(n: usize, memo: &mut [Option<usize>]) -> usize {
        memo[n].unwrap_or_else(|| {
            let result = {
                if n > 1 {
                    fib_memo(n - 1, memo) + fib_memo(n - 2, memo)
                } else {
                    1
                }
            };
            memo[n] = Some(result);
            result
        })
    }

    fib_memo(number, &mut vec![None; number + 1])
}

///
/// # Bottom up approach O(N) ( Dynamic Programming )
///
/// ```
/// use rust_fibonacci::bottom_up_approach;
///
/// assert_eq!(2971215073, bottom_up_approach(46));
/// ```
///
pub fn bottom_up_approach(n: usize) -> usize {
    match n {
        1 | 2 => 1,
        _ => {
            let mut bottom_up: Vec<usize> = Vec::new();
            bottom_up.push(1);
            bottom_up.push(1);

            for i in 2..n + 1 {
                bottom_up.push(bottom_up.get(i - 1).unwrap().clone() + bottom_up.get(i - 2).unwrap().clone())
            }

            bottom_up.get(n).unwrap().clone()
        }
    }
}

