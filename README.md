# vsymb
A simple CLI tool to compute the symbol of some Vassiliev invariants on a chord diagram.

Currently implements computers for the symbol of the Conway and (modified) Jones polynomials.

Reference: Chapter 3.6.2 (Jones) and Exercise 3.16 (Conway) of Introduction to Vassiliev Knot Invariants by Chmutov-Duzhin-Mostovoy. 

## Usage
```vsymb <invariant> <code>```

`<invariant>` : either `-c` or `-j` for the Conway or Jones polynomial.

`code` : code corresponding to the chord diagram. Any unicode grapheme is acceptable as a codeword.

e.g.
```
> vsymb -j 121323
-6
```
```
> vsymb -c 12123🦀3🦀
1
```
