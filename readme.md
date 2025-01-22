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

### Basic Optimization
- [x] Alpha beta pruning
- [x] Iterative deepening
- [x] Zobrist Hashing (for efficient board state representation)
- [x] Transposition Tables
- [x] Move Ordering
    - [x] With shallow depth foreseeing.
    - [ ] Heuristic move ordering.
    - [ ] MVV-LVA
- [ ] Time management
    - [ ] Basic time allocation (total time / expected moves remaining)
    - [ ] Anytime search capability
- [ ] Move urgency factors
   - [ ] Only legal move
   - [ ] Obvious captures/threats

### Search Optimization
- [ ] Quiescence Search
- [ ] Null Move Pruning
- [ ] Futility Pruning
- [ ] Late Move Reductions
- [ ] Razoring
- [ ] Lazy SMP (for parallel search)
- [ ] Opening Optimization
    - [ ] Books based : precomputed known good opening moves
    - [ ] Knowledge based : specific rules for opening (like how many square from the center are attacked)
- [ ] Endgame tables

### Advanced Search Techniques (will be done at the end)
- [ ] Aspiration Windows
- [ ] Principal Variation Search (is an improvement of alpha beta pruning)
- [ ] ProbCut
- [ ] Static Exchange Evaluation (SEE)
- [ ] Extensions
    - [ ] Check extensions (extend depth for checks).
    - [ ] Singular extensions (extend depth for unique critical moves).

### Parallelization
- [ ] Lazy SMP (thread-safe transposition tables required)
