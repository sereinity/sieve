# Todo

## v1

### Optimizations
  - [ ] Dynamic vector capacity
  - [ ] Don't over allocates batches
  - [ ] Change storage engine to `Vec<usize>`
  - [X] Don't loop from two for each prime, estimate a good starting point

### Bug
  - [X] Minimal batch size of 1 generates a bug on «4»

### User interface
  - [X] Deep logging
  - [ ] Alternative print in Hexa instead of dots

### Multi-threading
  - [ ] Allocations
  - [ ] Batch run
  - [ ] Option to remove useless batches

### Analysis
  - [ ] Look for patterns
  - [ ] Use fixed batch size ?
  - [ ] Choose minimal/maximal batch size if not


## v2

### Infinite sievieng
  - [ ] Save / load completed batches
  - [ ] Completed batch pool
  - [ ] Thread pool

### Look for primes of a simple multiple
  - [ ] `iter_primes_from()` loads/unloads batches
  - [ ] Start looking from √value
  - [ ] Optimize batche lookups by statistic on multiple sources (RSA)


## v3

### Multi instances
  - [ ] Store in a network database
  - [ ] Mining instances
    - [ ] Cache firsts batches
  - [ ] Lookup instances
    - [ ] → alert if maybe not in database

### API
  - [ ] Primes for
    - [ ] Query on lookup instances
    - [ ] Async
    - [ ] Pull/Push + retry
  - [ ] Is prime
    - [ ] True/False/Unknown/Loading
    - [ ] Async
    - [ ] Pull/Push + retry

### WebUI
  - [ ] Convert RSA to numbers

### Optimizations
  - [ ] Prioritize mining from lookuped values


## v4

### Mining statistics
  - [ ] Mean batch completion time
  - [ ] Mean batch lookup from database
  - [ ] SLI/SLO

### API statistics
  - [ ] Response time
  - [ ] Answer time
  - [ ] SLI/SLO

### Estimate completion date for a query
  - [ ] Improve the retry period
  - [ ] Inform the user
