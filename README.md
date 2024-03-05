# Superman's Chicken Rescue (Problem 2)

This package contains the function `solve` in [lib.rs](src/lib.rs) which determines the maximum number of chickens Superman can protect.

## Running

The easiest way to run this solution is via cargo. You will be prompted with input in console.
```sh
cargo run
input (chicken_amount roof_size): 5 5
chicken positions: 2 5 10 12 15
result: 2
```
### Constrains
- The `chicken_amount` and `roof_size` is in range [1, 1'000'000].
- The positions must be in ascending order and in range [1, 1'000'000'000].
- The `chicken_amount` and number of position must match.

## Solution

The idea is some kind of `queue` structure.

Before pushing in new position, we peek the first position in queue and calculate the distance between them.
If the distance is greater than the roof size, we pop the first position out of the queue. Keep doing this until the queue is empty or the position is within the roof. Then we get the number of chicken by the queue's `length`.

Since I use vector as input, I can use indexes to achieve the same thing.

```rust
for i in 0..amount {
        // de-queue front chickens that move outside of the roof
        while pos[i] - pos[front_pos] >= roof_size {
            front_pos = front_pos + 1;
        }

        // check max chicken
        if i - front_pos > max {
            max = i - front_pos;
        }
    }
```

## Time complexity

The code iterates through all positions and moves the front position as the roof moves.

In the worst case scenario, the `front_pos` moves to the end, making us visit each position twice. Hence, the time complexity is: `O(n)`

## Memory complexity

The code only uses 2 variables in addition to the input array itself:

```rust
let mut max: usize = 0;
let mut front_pos: usize = 0;
```

Hence, the memory complexity (disregard of the input) is: `O(1)`
