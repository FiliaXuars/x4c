# x4c

./specs
```
32b cpu
4b instructions
2b      cpu memory addressing
26b     main memory addressing

67 108 864b
8MiB
2 097 152 addresses
000000-3ffffff

5kb display 128^2 limited 32 color palette

0-f     0-f     000000-3ffffff
opcode  buffer  address

i = instruction
m = main memory
c = compute memory
- = ignored

[instructions]
0   noop                [---------]
1   jump unconditional  [i-mmmmmmm]
2   jump conditional    [icmmmmmmm]
3   jump to pointer     [ic-------]
4   take                [icmmmmmmm]
5   place               [icmmmmmmm]
6   greater than        [iccc-----]
7   less than           [iccc-----]
8   and                 [iccc-----]
9   or                  [iccc-----]
a   xor                 [iccc-----]
b   nor                 [iccc-----]
c   add                 [iccc-----]
d   sub                 [iccc-----]
e   shift left          [iccc-----]
f   shift right         [iccc-----]
```
