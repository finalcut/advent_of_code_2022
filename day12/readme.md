# trying to understand how the BFS actually works

using our basic sample data and starting from the end working back to the beginning (E => S) we have this as the result of the process.

At a certain point I put the letter with it's coordinates like cXY where c is the letter and x is the column and Y is the row starting at the top left of `test.txt`

```
1   S a b q p o n m
2   a b c r y x x l
3   a c c s z E x k
4   a c c t u v w j
5   a b d e f g h i

1   2 3 4 5 6 7 8
```
```
[z]
start
[] => [z]
1
[] => [y]
2
[] => [x]
3
[] => [x]
4
[] => [x]
5
[] => [w]
6
[] => [v]
7
[] => [u]
8
[] => [t]
9
[] => [s]
10
[] => [r]
11
[] => [q]
12
[] => [p]
13
[] => [o]
14
[] => [n]
15
[] => [m]
16
[] => [1]
17
[] => [k]
18
[] => [j]
19
[] => [i]
20
[] => [h]
21
[] => [g]
22
[] => [f]
23
[] => [e]
24
[] => [d]
25
[] => [c]
26
[] => [[c24,c33]]
[[c33]] => [b42]
27
[b42] => [[c32, c23]]
28
[[c32]] => [[c23], a14]
[[c23]] = [a14, [b12, b21]]
29
[a14] =[[b12, b21], [a15, a13]]
30
[b12] = [b21], [a15, a13], S]
31
```

To further illustrate here is how the queue looks as the iterations go which should match what I have above basically

in this you see the a tuple for each element in the queue that represents (letter, x, y) except it uses a zero based index for x and y where I used a 1 based index in my manual walk through.

```
queue: [(z, 2, 5)]
queue: [(z, 2, 4)]
queue: [(y, 1, 4)]
queue: [(x, 1, 5)]
queue: [(x, 1, 6)]
queue: [(x, 2, 6)]
queue: [(w, 3, 6)]
queue: [(v, 3, 5)]
queue: [(u, 3, 4)]
queue: [(t, 3, 3)]
queue: [(s, 2, 3)]
queue: [(r, 1, 3)]
queue: [(q, 0, 3)]
queue: [(p, 0, 4)]
queue: [(o, 0, 5)]
queue: [(n, 0, 6)]
queue: [(m, 0, 7)]
queue: [(l, 1, 7)]
queue: [(k, 2, 7)]
queue: [(j, 3, 7)]
queue: [(i, 4, 7)]
queue: [(h, 4, 6)]
queue: [(g, 4, 5)]
queue: [(f, 4, 4)]
queue: [(e, 4, 3)]
queue: [(d, 4, 2)]
queue: [(c, 3, 2)]
queue: [(c, 3, 1), (c, 2, 2)]
queue: [(b, 4, 1), (c, 2, 1), (c, 1, 2)]
queue: [(a, 4, 0), (b, 1, 1), (b, 0, 2)]
queue: [(a, 3, 0), (a, 1, 0), (a, 0, 1)]
queue: [(a, 2, 0), (s, 0, 0)]
```

Finally this last bit shows the grid - as each point gets visited it is changed to be a zero.  Find the E in the first grid and then you can follow the path the algorithim takes. In the second grid the E and the first visited node both become a 0


```
grid:
[[S, a, b, q, p, o, n, m],
 [a, b, c, r, y, x, x, l],
 [a, c, c, s, z, E, x, k],
 [a, c, c, t, u, v, w, j],
 [a, b, d, e, f, g, h, i]]
grid:
[[S, a, b, q, p, o, n, m],
 [a, b, c, r, y, x, x, l],
 [a, c, c, s, 0, 0, x, k],
 [a, c, c, t, u, v, w, j],
 [a, b, d, e, f, g, h, i]]
grid:
[[S, a, b, q, p, o, n, m],
 [a, b, c, r, 0, x, x, l],
 [a, c, c, s, 0, 0, x, k],
 [a, c, c, t, u, v, w, j],
 [a, b, d, e, f, g, h, i]]
grid:
[[S, a, b, q, p, o, n, m],
 [a, b, c, r, 0, 0, x, l],
 [a, c, c, s, 0, 0, x, k],
 [a, c, c, t, u, v, w, j],
 [a, b, d, e, f, g, h, i]]
grid:
[[S, a, b, q, p, o, n, m],
 [a, b, c, r, 0, 0, 0, l],
 [a, c, c, s, 0, 0, x, k],
 [a, c, c, t, u, v, w, j],
 [a, b, d, e, f, g, h, i]]
grid:
[[S, a, b, q, p, o, n, m],
 [a, b, c, r, 0, 0, 0, l],
 [a, c, c, s, 0, 0, 0, k],
 [a, c, c, t, u, v, w, j],
 [a, b, d, e, f, g, h, i]]
grid:
[[S, a, b, q, p, o, n, m],
 [a, b, c, r, 0, 0, 0, l],
 [a, c, c, s, 0, 0, 0, k],
 [a, c, c, t, u, v, 0, j],
 [a, b, d, e, f, g, h, i]]
grid:
[[S, a, b, q, p, o, n, m],
 [a, b, c, r, 0, 0, 0, l],
 [a, c, c, s, 0, 0, 0, k],
 [a, c, c, t, u, 0, 0, j],
 [a, b, d, e, f, g, h, i]]
grid:
[[S, a, b, q, p, o, n, m],
 [a, b, c, r, 0, 0, 0, l],
 [a, c, c, s, 0, 0, 0, k],
 [a, c, c, t, 0, 0, 0, j],
 [a, b, d, e, f, g, h, i]]
grid:
[[S, a, b, q, p, o, n, m],
 [a, b, c, r, 0, 0, 0, l],
 [a, c, c, s, 0, 0, 0, k],
 [a, c, c, 0, 0, 0, 0, j],
 [a, b, d, e, f, g, h, i]]
grid:
[[S, a, b, q, p, o, n, m],
 [a, b, c, r, 0, 0, 0, l],
 [a, c, c, 0, 0, 0, 0, k],
 [a, c, c, 0, 0, 0, 0, j],
 [a, b, d, e, f, g, h, i]]
grid:
[[S, a, b, q, p, o, n, m],
 [a, b, c, 0, 0, 0, 0, l],
 [a, c, c, 0, 0, 0, 0, k],
 [a, c, c, 0, 0, 0, 0, j],
 [a, b, d, e, f, g, h, i]]
grid:
[[S, a, b, 0, p, o, n, m],
 [a, b, c, 0, 0, 0, 0, l],
 [a, c, c, 0, 0, 0, 0, k],
 [a, c, c, 0, 0, 0, 0, j],
 [a, b, d, e, f, g, h, i]]
grid:
[[S, a, b, 0, 0, o, n, m],
 [a, b, c, 0, 0, 0, 0, l],
 [a, c, c, 0, 0, 0, 0, k],
 [a, c, c, 0, 0, 0, 0, j],
 [a, b, d, e, f, g, h, i]]
grid:
[[S, a, b, 0, 0, 0, n, m],
 [a, b, c, 0, 0, 0, 0, l],
 [a, c, c, 0, 0, 0, 0, k],
 [a, c, c, 0, 0, 0, 0, j],
 [a, b, d, e, f, g, h, i]]
grid:
[[S, a, b, 0, 0, 0, 0, m],
 [a, b, c, 0, 0, 0, 0, l],
 [a, c, c, 0, 0, 0, 0, k],
 [a, c, c, 0, 0, 0, 0, j],
 [a, b, d, e, f, g, h, i]]
grid:
[[S, a, b, 0, 0, 0, 0, 0],
 [a, b, c, 0, 0, 0, 0, l],
 [a, c, c, 0, 0, 0, 0, k],
 [a, c, c, 0, 0, 0, 0, j],
 [a, b, d, e, f, g, h, i]]
grid:
[[S, a, b, 0, 0, 0, 0, 0],
 [a, b, c, 0, 0, 0, 0, 0],
 [a, c, c, 0, 0, 0, 0, k],
 [a, c, c, 0, 0, 0, 0, j],
 [a, b, d, e, f, g, h, i]]
grid:
[[S, a, b, 0, 0, 0, 0, 0],
 [a, b, c, 0, 0, 0, 0, 0],
 [a, c, c, 0, 0, 0, 0, 0],
 [a, c, c, 0, 0, 0, 0, j],
 [a, b, d, e, f, g, h, i]]
grid:
[[S, a, b, 0, 0, 0, 0, 0],
 [a, b, c, 0, 0, 0, 0, 0],
 [a, c, c, 0, 0, 0, 0, 0],
 [a, c, c, 0, 0, 0, 0, 0],
 [a, b, d, e, f, g, h, i]]
grid:
[[S, a, b, 0, 0, 0, 0, 0],
 [a, b, c, 0, 0, 0, 0, 0],
 [a, c, c, 0, 0, 0, 0, 0],
 [a, c, c, 0, 0, 0, 0, 0],
 [a, b, d, e, f, g, h, 0]]
grid:
[[S, a, b, 0, 0, 0, 0, 0],
 [a, b, c, 0, 0, 0, 0, 0],
 [a, c, c, 0, 0, 0, 0, 0],
 [a, c, c, 0, 0, 0, 0, 0],
 [a, b, d, e, f, g, 0, 0]]
grid:
[[S, a, b, 0, 0, 0, 0, 0],
 [a, b, c, 0, 0, 0, 0, 0],
 [a, c, c, 0, 0, 0, 0, 0],
 [a, c, c, 0, 0, 0, 0, 0],
 [a, b, d, e, f, 0, 0, 0]]
grid:
[[S, a, b, 0, 0, 0, 0, 0],
 [a, b, c, 0, 0, 0, 0, 0],
 [a, c, c, 0, 0, 0, 0, 0],
 [a, c, c, 0, 0, 0, 0, 0],
 [a, b, d, e, 0, 0, 0, 0]]
grid:
[[S, a, b, 0, 0, 0, 0, 0],
 [a, b, c, 0, 0, 0, 0, 0],
 [a, c, c, 0, 0, 0, 0, 0],
 [a, c, c, 0, 0, 0, 0, 0],
 [a, b, d, 0, 0, 0, 0, 0]]
grid:
[[S, a, b, 0, 0, 0, 0, 0],
 [a, b, c, 0, 0, 0, 0, 0],
 [a, c, c, 0, 0, 0, 0, 0],
 [a, c, c, 0, 0, 0, 0, 0],
 [a, b, 0, 0, 0, 0, 0, 0]]
grid:
[[S, a, b, 0, 0, 0, 0, 0],
 [a, b, c, 0, 0, 0, 0, 0],
 [a, c, c, 0, 0, 0, 0, 0],
 [a, c, 0, 0, 0, 0, 0, 0],
 [a, b, 0, 0, 0, 0, 0, 0]]
grid:
[[S, a, b, 0, 0, 0, 0, 0],
 [a, b, c, 0, 0, 0, 0, 0],
 [a, c, 0, 0, 0, 0, 0, 0],
 [a, 0, 0, 0, 0, 0, 0, 0],
 [a, b, 0, 0, 0, 0, 0, 0]]
grid:
[[S, a, b, 0, 0, 0, 0, 0],
 [a, b, 0, 0, 0, 0, 0, 0],
 [a, 0, 0, 0, 0, 0, 0, 0],
 [a, 0, 0, 0, 0, 0, 0, 0],
 [a, 0, 0, 0, 0, 0, 0, 0]]
grid:
[[S, a, 0, 0, 0, 0, 0, 0],
 [a, 0, 0, 0, 0, 0, 0, 0],
 [a, 0, 0, 0, 0, 0, 0, 0],
 [a, 0, 0, 0, 0, 0, 0, 0],
 [0, 0, 0, 0, 0, 0, 0, 0]]
grid:
[[S, 0, 0, 0, 0, 0, 0, 0],
 [0, 0, 0, 0, 0, 0, 0, 0],
 [a, 0, 0, 0, 0, 0, 0, 0],
 [0, 0, 0, 0, 0, 0, 0, 0],
 [0, 0, 0, 0, 0, 0, 0, 0]]
grid:
[[0, 0, 0, 0, 0, 0, 0, 0],
 [0, 0, 0, 0, 0, 0, 0, 0],
 [0, 0, 0, 0, 0, 0, 0, 0],
 [0, 0, 0, 0, 0, 0, 0, 0],
 [0, 0, 0, 0, 0, 0, 0, 0]]
 ```