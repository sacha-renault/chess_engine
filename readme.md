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

### Basic
- [x] alpha beta pruning
- [x] Zobrist Hashing (for efficient board state representation)
- [ ] Transposition Tables: memory leak identified, working on fix.
- [x] Move Ordering
    - [x] With shallow depth foreseeing.
    - [ ] Heuristic move ordering.

### Search Optimization
- [ ] Quiescence Search
- [ ] Null Move Pruning
- [ ] Futility Pruning
- [ ] Late Move Reductions
- [ ] Razoring
- [ ] Lazy SMP (for parallel search)
- [ ] Endgame Tablebases

### Advanced Search Techniques (will be done at the end)
- [ ] Aspiration Windows
- [ ] Principal Variation Search (is an improvement of alpha beta pruning)
- [ ] ProbCut

### EndGame Optimization
- [ ] Static Exchange Evaluation (SEE)
- [ ] Extensions
    - [ ] Check extensions (extend depth for checks).
    - [ ] Singular extensions (extend depth for unique critical moves).
- [ ] Endgame table base

### Parallelization
- [ ] Lazy SMP (thread-safe transposition tables required)
