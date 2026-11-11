# Fibonacci
Finds a Fibonacci number. The rule: add the last two numbers to get the next one. 1, 1, 2, 3, 5, 8, 13, 21, 34, 55...

## How it works
I keep the last two numbers, a and b. Each round: new = a + b, a becomes b, b becomes the new number.

## Bugs I found
- **The `let` bug:** I wrote `let` inside the loop, so it made a new a and b instead of changing the old ones. It printed 1 instead of 55.
- **The i32 bug:** the 50th number is too big for an i32, so it overflowed. I changed it to i128.
