# Find digits

I received a math exercice where I needed to find the last decimal digits of a huge number. That number is `13^(7^n)`, where `n` is based on your date of birth. For example, if you are born on  the 23rd of March, n = 2303. If you are born on the 4th of October, n = 411.

This program does exactly that. It asks you for your `n`, then for the number of digits `NB` you want to get, and it computes everything for you.

## How do one can find the last decimal digits of a number like that

I'll explain continuously with example, where you are born on the 2nd of December and want to get 2 digits.

1. Express your `7^n` (so `7^212`) as a number in the form `10d + q`. `d` represents here the the number of dozens in `7^212` and `u` the number of units. You only need to find `u` by finding the remainder in the euclidean division of `7^n` by `10^NB`. In our case, we find `7^212 = 10d + 1`. So `13^(7^212) = 13^(10d + 1)`.
1. Find a power of 13 that congrues to 1 modulo 10^NB, or in other word, a power of 13 whose NB last digits are equal to 1. We find using a calculator or a computer that `13^20 === 1 [10^2]`. (because `13^20 = ...01`)
1. Find the smallest power of 13 that congrues to `13^(7^506)`, that's to say a power that ends with the same NB last digits as `13^(7^506)`. This is far from obvious, but we find it by calculating the remainder of the last digits of the exponent by the power that we found in step 2. In our case, `1 % 20 = 1` (% = remainder), so we can deduce that `13^(7^212) === 13 [10^2]`
1. Find the last digits of the power you got in step 3, because they are the same as your big big number. You can find fast algorithms for finding these by searching "modular power" on the internet ; that's the one I use internally (BigUint::modpow).
