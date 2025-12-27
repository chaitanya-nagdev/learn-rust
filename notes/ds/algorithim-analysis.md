# Algorithm Analysis

- If we have **program** we can run the program with different test inputs and record time elapsed **ephoch**
  or normal elapsed but then it will depend on CPU cycle used but that also won't give us consistent result.

  So we have to figure out an independent way to determine the by plotting a graph with n-size & t time taken
  by program and plot it on x-y axis independently.

**Main Goal**

1. Figure out relative efficiency of two algorithm in a way independent of hardware & software
2. We can study performance by high-level description without need for implementation
3. Takes into account all possible inputs.

**Counting Primitive**

- we can count primitive operation such as 't' and then correlate to actual running time in a specific
  computer.
- We can map this as a function f(n), where n=size of input.

**Seven Function**

- we have set of 7 function which we can use to determine size of program.

#### 1) Constants Function

- f(n) = c : This c can be anything like 5,10,2^10, so it does not matter what is size of input
  it will take same time or give same output.

  g(n) = 1 : This is most fundamental constants function so this will be used for analysis.

  f(n) = cg(n)

---

#### 2) Logarithm Function

logb(N) = x so we say b^x = N

- “x” is the **power** you raise the base (b) to get (N).
- So yes: logarithm is **just the unknown exponent**, the “power needed” to reach a number.

---

2. When to use exponential vs logarithm

- **Exponential** ((b^x)): You know the **base** and **power**, want the **result**.
- **Logarithm** ((\log_b N)): You know the **base** and **result**, want the **power**.

---

3. Example in computer science

Many algorithms divide data repeatedly (like binary search or merge sort).

- **Size of input:** (N)
- **Division factor:** (b = 2) (common in computers, base 2)
- **Goal:** How many times do we divide until we reach 1?

Mathematically:

log2(N) = x So 2^x = N : So to reach from N to 1 we have to perform X operation thus complexity is LOG(N) something like that

- (x) = number of steps (or operations) the algorithm will perform.
- That’s why we often see **O(log N)** in time complexity.

* So we use to figure out how many times thing will run as we know input size & repeatation.

---

✅ **In short:**

- Logarithms tell you **how many times you have to multiply or divide by a base to reach a number**.
- Exponential tells you **what you get when you multiply a base by itself some number of times**.
- In algorithms: Logarithms help calculate **number of steps** when repeatedly dividing data (base often 2 in computing).

**Proposition 3.1 (Logarithm Rules): Given real numbers a > 0, b > 1, c > 0
and d > 1, we have**

1. logb (ac) = logb a + logb c
2. logb (a/c) = logb a − logb c
3. logb (ac ) = c logb a
4. logb a = logd a/ logd b
5. blogd a = alogd b (b power log base d a like that )

- rule 4 gives us way to calculate : Log base 10 for Log base 2 N which is like Log base 2 of N can be written as
  LOG N / LOG 2 here base will be 10

---

#### 3) LINEAR FUNCTION

f(n) = n

- That is, given an input value n, the linear function f assigns the value n itself.
- Ex : Comparing X with each item in list, as size of list increase and is N we have to compare N times thus
  this function indicates that.
- Best running time we can hope for to acheive for any algorithm that processes each object n.

#### 4) N-Log-N Function

- f(n) = nlog(n)
- This is grows rapidly that linear function and lot less rapidly then quadratic, we prefer this over qudratic
- This indicate our program perform logN operation N times for an input of size n, thus like sorting arbitary values
  require time proportional to nlogn

#### 5) Quadratic Function

- f(n) = n^2
- Appear because program has inner loop running same as outer loop

Comes in picture for if inner loop performs n operation for every operation of outer loop

1 + 2 + 3 ... (n-1) + n = n(n+1) / 2 : Carl Gauss equation

#### 6) Cubic OR Polynomials

- f(n)=n^3
- Most of the functions we have listed so far can each be viewed as being part of a
  larger class of functions, the polynomials. A polynomial function has the form,

  f (n) = a0 + a1 n + a2 n2 + a3 n3 + · · · + ad nd ,

- where a0 , a1 , . . . , ad are constants, called the coefficients of the polynomial, and
  ad = 0. Integer d, which indicates the highest power in the polynomial, is called
  the degree of the polynomial.

- For example, the following functions are all polynomials:
- f (n) = 2 + 5n + n2
- f (n) = 1 + n3
- f (n) = 1
- f (n) = n
- f (n) = n2

#### 7) Exponential Function

- f (n) = b^n
- Proposition 3.4 (Exponent Rules): Given positive integers a, b, and c, we have

1. (b^a )^c = b^ac
2. b^a b^c = b^a+c
3. b^a /b^c = b^a−c

### Comparing Growth

| Function     | Time Growth |
| :----------- | ----------: |
| Constant     |           1 |
| Logarithmic  |       log n |
| Linear       |           n |
| Linearithmic |     n log n |
| Quadratic    |          n² |
| Cubic        |          n³ |
| Exponential  |         a^n |

- Cubic,exponential are considered non-feasible for pratical usage due to there growth rate
- Program with lower growth rate are preffered.
- We can use ceiling & floor function to make things integer as they are not always integer as
  we use integer to show number of operation
  Floor |x| : largest integer less than or equal to x
  Celing |x| : largest integer greater than or equal to x

### Asymptotic Analysis

- PENDING FROM PAGE 123 & 3.3
