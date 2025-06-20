# Chess Engine

A chess engine implementation in rust.

## Overview

/!\ since tree v2 start to be implemented, the engine is kinda buggy, the engine is currently on a really unstable state.

## Features

- Basic chess rules implementation
- Move validation
- Board representation
- Stockfish like engine

## RoadMap

### Basic Optimization
- [x] Alpha beta pruning
- [x] Iterative deepening
- [x] Zobrist Hashing (for efficient board state representation)
- [x] Transposition Tables
- [x] Move Ordering
    - [x] With shallow depth foreseeing.
    - [x] Heuristic move ordering.
    - [x] MVV-LVA
- [ ] Time management
    - [ ] Basic time allocation (total time / expected moves remaining)
    - [ ] Anytime search capability
- [ ] Move urgency factors
   - [ ] Only legal move
   - [ ] Obvious captures/threats

### Search Optimization
- [x] Quiescence Search
- [ ] Null Move Pruning
- [ ] Futility Pruning
- [ ] Late Move Reductions
- [x] Razoring
- [ ] Opening Optimization
    - [x] Books based : precomputed known good opening moves
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
- [ ] Lazy SMP
    - [x] make transposition table thread safe (best way is probably with a RwLock)
    - [ ] make nodes access thread safe
    - [ ] Parallel calculation from the root node

## TODO
Set foreseeing to use qsearch, use static evaluation for stable move and search forward only for unstable.
Fix razoring that is currently pruning good moves.
Investigate on why it returns a None lv when tree size is bigger than max size BEFORE generation
Check how to fix cyclic references with transposition table.
