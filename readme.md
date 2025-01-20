# Chess Engine

A chess engine implementation in rust.

## Overview

This project implements a chess engine capable of playing chess according to standard rules.

## Features

- Basic chess rules implementation
- Move validation
- Board representation
- Stockfish like engine

## TODO

- [x] alpha beta pruning
- [x] Zobrist Hashing (for efficient board state representation)
- [ ] Transposition Tables: implemented but leak memory so has to be fixed
- [ ] Move Ordering
- [ ] Quiescence Search
- [ ] Null Move Pruning
- [ ] Futility Pruning
- [ ] Late Move Reductions
- [ ] Principal Variation Search (PVS)
- [ ] Aspiration Windows
- [ ] ProbCut
- [ ] Razoring
- [ ] Extensions (e.g., check extensions, singular extensions)
- [ ] Lazy SMP (for parallel search)
- [ ] Static Exchange Evaluation (SEE)
- [ ] Endgame Tablebases