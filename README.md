# `bluejay-rb`

> **Warning**
> This project is still very early in its development and should be considered highly unstable and experimental. It is incomplete and not ready for production use.

> **Note**
> In an effort to get a working proof-of-concept, documentation and test coverage has been sacrificied. This will be corrected prior to a stable release.

`bluejay-rb` is a GraphQL engine for Ruby written primarily in Rust through the use of [`magnus`](https://github.com/matsadler/magnus).

## Goal

`bluejay-rb`'s goal is to provide a lightning fast GraphQL engine with first-class integration with [Sorbet](https://sorbet.org/). If you do not care deeply about one or both of these goals, then you should seriously consider using the excellent [`graphql-ruby`](https://graphql-ruby.org) gem, which has many more features than `bluejay-rb` in addition to being production-ready and much more customizable.

## Benchmarks

Some benchmarks comparing the performance of `bluejay-rb` against `graphql-ruby` (with C parser) are located at [`/bench`](/bench). The results of some of these benchmarks are included below.

<!---benchmark result start-->
<details>
  <summary>Parse + Execute (Ruby 3.2, YJIT enabled)</summary>

  ```
  Profiling IPS:
  Warming up --------------------------------------
               graphql   185.000  i/100ms
               bluejay     1.918k i/100ms
  Calculating -------------------------------------
               graphql      1.886k (± 1.9%) i/s -      9.435k in   5.005527s
               bluejay     18.918k (± 1.3%) i/s -     95.900k in   5.070143s
  
  Comparison:
               bluejay:    18917.7 i/s
               graphql:     1885.6 i/s - 10.03x  (± 0.00) slower
  
  Profiling Ruby memory allocations:
  Calculating -------------------------------------
               graphql    46.560k memsize (   168.000  retained)
                         421.000  objects (     1.000  retained)
                          12.000  strings (     0.000  retained)
               bluejay     5.256k memsize (    40.000  retained)
                          40.000  objects (     1.000  retained)
                           0.000  strings (     0.000  retained)
  
  Comparison:
               bluejay:       5256 allocated
               graphql:      46560 allocated - 8.86x more
  ```
</details>

<details>
  <summary>Parse + Execute (Ruby 3.2, YJIT disabled)</summary>

  ```
  Profiling IPS:
  Warming up --------------------------------------
               graphql    77.000  i/100ms
               bluejay     1.921k i/100ms
  Calculating -------------------------------------
               graphql    770.721  (± 3.1%) i/s -      3.850k in   5.001051s
               bluejay     19.178k (± 3.5%) i/s -     96.050k in   5.014961s
  
  Comparison:
               bluejay:    19178.4 i/s
               graphql:      770.7 i/s - 24.88x  (± 0.00) slower
  
  Profiling Ruby memory allocations:
  Calculating -------------------------------------
               graphql    46.560k memsize (    21.752k retained)
                         421.000  objects (   197.000  retained)
                          12.000  strings (    12.000  retained)
               bluejay     5.256k memsize (     5.016k retained)
                          40.000  objects (    34.000  retained)
                           0.000  strings (     0.000  retained)
  
  Comparison:
               bluejay:       5256 allocated
               graphql:      46560 allocated - 8.86x more
  ```
</details>

<details>
  <summary>Parse + Execute with large variables (Ruby 3.2, YJIT enabled)</summary>

  ```
  Profiling IPS:
  Warming up --------------------------------------
               graphql   155.000  i/100ms
               bluejay     1.044k i/100ms
  Calculating -------------------------------------
               graphql      1.556k (± 2.9%) i/s -      7.905k in   5.084182s
               bluejay     10.205k (± 1.9%) i/s -     51.156k in   5.014556s
  
  Comparison:
               bluejay:    10205.0 i/s
               graphql:     1556.2 i/s - 6.56x  (± 0.00) slower
  
  Profiling Ruby memory allocations:
  Calculating -------------------------------------
               graphql    97.728k memsize (    40.000  retained)
                         885.000  objects (     1.000  retained)
                           6.000  strings (     0.000  retained)
               bluejay    15.640k memsize (    40.000  retained)
                         228.000  objects (     1.000  retained)
                          15.000  strings (     0.000  retained)
  
  Comparison:
               bluejay:      15640 allocated
               graphql:      97728 allocated - 6.25x more
  ```
</details>

<details>
  <summary>Parse + Execute with large variables (Ruby 3.2, YJIT disabled)</summary>

  ```
  Profiling IPS:
  Warming up --------------------------------------
               graphql    80.000  i/100ms
               bluejay   981.000  i/100ms
  Calculating -------------------------------------
               graphql    807.213  (± 2.0%) i/s -      4.080k in   5.056567s
               bluejay      9.729k (± 2.0%) i/s -     49.050k in   5.043576s
  
  Comparison:
               bluejay:     9729.5 i/s
               graphql:      807.2 i/s - 12.05x  (± 0.00) slower
  
  Profiling Ruby memory allocations:
  Calculating -------------------------------------
               graphql    97.688k memsize (    30.736k retained)
                         884.000  objects (   319.000  retained)
                           6.000  strings (     5.000  retained)
               bluejay    12.600k memsize (   576.000  retained)
                         152.000  objects (     7.000  retained)
                           8.000  strings (     2.000  retained)
  
  Comparison:
               bluejay:      12600 allocated
               graphql:      97688 allocated - 7.75x more
  ```
</details>

<details>
  <summary>Parse small (Ruby 3.2, YJIT enabled)</summary>

  ```
  Profiling IPS:
  Warming up --------------------------------------
               bluejay    38.444k i/100ms
               graphql     6.772k i/100ms
  Calculating -------------------------------------
               bluejay    380.402k (±11.3%) i/s -      1.884M in   5.065716s
               graphql     67.386k (± 2.8%) i/s -    338.600k in   5.029332s
  
  Comparison:
               bluejay:   380402.0 i/s
               graphql:    67385.6 i/s - 5.65x  (± 0.00) slower
  
  Profiling Ruby memory allocations:
  Calculating -------------------------------------
               bluejay     0.000  memsize (     0.000  retained)
                           0.000  objects (     0.000  retained)
                           0.000  strings (     0.000  retained)
               graphql     6.192k memsize (     2.816k retained)
                          70.000  objects (    37.000  retained)
                           6.000  strings (     6.000  retained)
  
  Comparison:
               bluejay:          0 allocated
               graphql:       6192 allocated - Infx more
  ```
</details>

<details>
  <summary>Parse small (Ruby 3.2, YJIT disabled)</summary>

  ```
  Profiling IPS:
  Warming up --------------------------------------
               bluejay    39.276k i/100ms
               graphql     5.847k i/100ms
  Calculating -------------------------------------
               bluejay    385.293k (± 1.3%) i/s -      1.964M in   5.097814s
               graphql     58.784k (± 2.3%) i/s -    298.197k in   5.075678s
  
  Comparison:
               bluejay:   385293.4 i/s
               graphql:    58784.0 i/s - 6.55x  (± 0.00) slower
  
  Profiling Ruby memory allocations:
  Calculating -------------------------------------
               bluejay     0.000  memsize (     0.000  retained)
                           0.000  objects (     0.000  retained)
                           0.000  strings (     0.000  retained)
               graphql     6.192k memsize (     0.000  retained)
                          70.000  objects (     0.000  retained)
                           6.000  strings (     0.000  retained)
  
  Comparison:
               bluejay:          0 allocated
               graphql:       6192 allocated - Infx more
  ```
</details>

<details>
  <summary>Parse large (Ruby 3.2, YJIT enabled)</summary>

  ```
  Profiling IPS:
  Warming up --------------------------------------
               bluejay   199.000  i/100ms
               graphql    29.000  i/100ms
  Calculating -------------------------------------
               bluejay      2.081k (± 1.5%) i/s -     10.547k in   5.070330s
               graphql    298.368  (± 1.3%) i/s -      1.508k in   5.055303s
  
  Comparison:
               bluejay:     2080.6 i/s
               graphql:      298.4 i/s - 6.97x  (± 0.00) slower
  
  Profiling Ruby memory allocations:
  Calculating -------------------------------------
               bluejay     0.000  memsize (     0.000  retained)
                           0.000  objects (     0.000  retained)
                           0.000  strings (     0.000  retained)
               graphql     1.425M memsize (   556.448k retained)
                          16.001k objects (     7.541k retained)
                          50.000  strings (    50.000  retained)
  
  Comparison:
               bluejay:          0 allocated
               graphql:    1425400 allocated - Infx more
  ```
</details>

<details>
  <summary>Parse large (Ruby 3.2, YJIT disabled)</summary>

  ```
  Profiling IPS:
  Warming up --------------------------------------
               bluejay   209.000  i/100ms
               graphql    27.000  i/100ms
  Calculating -------------------------------------
               bluejay      2.048k (± 3.0%) i/s -     10.241k in   5.005114s
               graphql    267.918  (± 3.7%) i/s -      1.350k in   5.047910s
  
  Comparison:
               bluejay:     2048.1 i/s
               graphql:      267.9 i/s - 7.64x  (± 0.00) slower
  
  Profiling Ruby memory allocations:
  Calculating -------------------------------------
               bluejay     0.000  memsize (     0.000  retained)
                           0.000  objects (     0.000  retained)
                           0.000  strings (     0.000  retained)
               graphql     1.425M memsize (     0.000  retained)
                          16.001k objects (     0.000  retained)
                          50.000  strings (     0.000  retained)
  
  Comparison:
               bluejay:          0 allocated
               graphql:    1425400 allocated - Infx more
  ```
</details>

<details>
  <summary>Parse + Validate (Ruby 3.2, YJIT enabled)</summary>

  ```
  Profiling IPS:
  Warming up --------------------------------------
               graphql   474.000  i/100ms
               bluejay     5.514k i/100ms
  Calculating -------------------------------------
               graphql      4.815k (± 2.7%) i/s -     24.174k in   5.025030s
               bluejay     56.064k (± 1.9%) i/s -    281.214k in   5.017662s
  
  Comparison:
               bluejay:    56064.4 i/s
               graphql:     4814.5 i/s - 11.64x  (± 0.00) slower
  
  Profiling Ruby memory allocations:
  Calculating -------------------------------------
               graphql    33.392k memsize (    12.200k retained)
                         383.000  objects (   152.000  retained)
                          17.000  strings (    13.000  retained)
               bluejay    40.000  memsize (    40.000  retained)
                           1.000  objects (     1.000  retained)
                           0.000  strings (     0.000  retained)
  
  Comparison:
               bluejay:         40 allocated
               graphql:      33392 allocated - 834.80x more
  ```
</details>

<details>
  <summary>Parse + Validate (Ruby 3.2, YJIT disabled)</summary>

  ```
  Profiling IPS:
  Warming up --------------------------------------
               graphql   327.000  i/100ms
               bluejay     5.654k i/100ms
  Calculating -------------------------------------
               graphql      3.189k (± 1.8%) i/s -     16.023k in   5.025586s
               bluejay     55.558k (± 2.3%) i/s -    282.700k in   5.091280s
  
  Comparison:
               bluejay:    55558.0 i/s
               graphql:     3189.4 i/s - 17.42x  (± 0.00) slower
  
  Profiling Ruby memory allocations:
  Calculating -------------------------------------
               graphql    33.304k memsize (    12.320k retained)
                         381.000  objects (   155.000  retained)
                          15.000  strings (    13.000  retained)
               bluejay    40.000  memsize (    40.000  retained)
                           1.000  objects (     1.000  retained)
                           0.000  strings (     0.000  retained)
  
  Comparison:
               bluejay:         40 allocated
               graphql:      33304 allocated - 832.60x more
  ```
</details>

<details>
  <summary>Schema dump (Ruby 3.2, YJIT enabled)</summary>

  ```
  Profiling IPS:
  Warming up --------------------------------------
               graphql   200.000  i/100ms
               bluejay     4.801k i/100ms
  Calculating -------------------------------------
               graphql      1.959k (± 9.7%) i/s -      9.800k in   5.086725s
               bluejay     45.271k (±10.5%) i/s -    225.647k in   5.054383s
  
  Comparison:
               bluejay:    45271.3 i/s
               graphql:     1959.1 i/s - 23.11x  (± 0.00) slower
  
  Profiling Ruby memory allocations:
  Calculating -------------------------------------
               graphql    77.788k memsize (     0.000  retained)
                         755.000  objects (     0.000  retained)
                          50.000  strings (     0.000  retained)
               bluejay   699.000  memsize (     0.000  retained)
                           1.000  objects (     0.000  retained)
                           1.000  strings (     0.000  retained)
  
  Comparison:
               bluejay:        699 allocated
               graphql:      77788 allocated - 111.28x more
  ```
</details>

<details>
  <summary>Schema dump (Ruby 3.2, YJIT disabled)</summary>

  ```
  Profiling IPS:
  Warming up --------------------------------------
               graphql    91.000  i/100ms
               bluejay     3.177k i/100ms
  Calculating -------------------------------------
               graphql      1.391k (± 9.9%) i/s -      6.916k in   5.045690s
               bluejay     46.324k (± 3.2%) i/s -    231.921k in   5.012182s
  
  Comparison:
               bluejay:    46324.2 i/s
               graphql:     1390.6 i/s - 33.31x  (± 0.00) slower
  
  Profiling Ruby memory allocations:
  Calculating -------------------------------------
               graphql    77.884k memsize (   675.000  retained)
                         755.000  objects (    10.000  retained)
                          50.000  strings (     7.000  retained)
               bluejay   699.000  memsize (     0.000  retained)
                           1.000  objects (     0.000  retained)
                           1.000  strings (     0.000  retained)
  
  Comparison:
               bluejay:        699 allocated
               graphql:      77884 allocated - 111.42x more
  ```
</details>
<!---benchmark result end-->

## Installation

Install the gem and add to the application's Gemfile by executing:

    $ bundle add bluejay

If bundler is not being used to manage dependencies, install the gem by executing:

    $ gem install bluejay

## Usage

See an example in the [`/example`](/example) directory. Note the usage of [Tapioca](https://github.com/Shopify/tapioca) RBI generation for the DSL at [`/example/sorbet/rbi/dsl/graph`](/example/sorbet/rbi/dsl/graph)

## Development

After checking out the repo, run `bin/setup` to install dependencies. Then, run `rake test` to run the tests. You can also run `bin/console` for an interactive prompt that will allow you to experiment.

To install this gem onto your local machine, run `bundle exec rake install`. To release a new version, update the version number in `version.rb`, and then run `bundle exec rake release`, which will create a git tag for the version, push git commits and the created tag, and push the `.gem` file to [rubygems.org](https://rubygems.org).
