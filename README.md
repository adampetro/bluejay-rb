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
               graphql   174.000  i/100ms
               bluejay     1.918k i/100ms
  Calculating -------------------------------------
               graphql      1.855k (± 1.6%) i/s -      9.396k in   5.065922s
               bluejay     19.211k (± 1.9%) i/s -     97.818k in   5.093590s
  
  Comparison:
               bluejay:    19211.2 i/s
               graphql:     1855.2 i/s - 10.36x  (± 0.00) slower
  
  Profiling Ruby memory allocations:
  Calculating -------------------------------------
               graphql    46.560k memsize (     2.736k retained)
                         421.000  objects (    41.000  retained)
                          12.000  strings (    11.000  retained)
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
               graphql    76.000  i/100ms
               bluejay     1.863k i/100ms
  Calculating -------------------------------------
               graphql    769.155  (± 2.5%) i/s -      3.876k in   5.042593s
               bluejay     15.666k (±14.0%) i/s -     78.246k in   5.092916s
  
  Comparison:
               bluejay:    15666.4 i/s
               graphql:      769.2 i/s - 20.37x  (± 0.00) slower
  
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
  <summary>Parse small (Ruby 3.2, YJIT enabled)</summary>

  ```
  Profiling IPS:
  Warming up --------------------------------------
               bluejay    34.094k i/100ms
               graphql     6.013k i/100ms
  Calculating -------------------------------------
               bluejay    378.550k (± 6.3%) i/s -      1.909M in   5.068259s
               graphql     66.332k (± 3.4%) i/s -    336.728k in   5.083624s
  
  Comparison:
               bluejay:   378550.3 i/s
               graphql:    66331.9 i/s - 5.71x  (± 0.00) slower
  
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
               bluejay    40.542k i/100ms
               graphql     5.898k i/100ms
  Calculating -------------------------------------
               bluejay    391.927k (± 3.3%) i/s -      1.987M in   5.074515s
               graphql     59.036k (± 2.4%) i/s -    300.798k in   5.098254s
  
  Comparison:
               bluejay:   391927.4 i/s
               graphql:    59036.0 i/s - 6.64x  (± 0.00) slower
  
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
               bluejay   185.000  i/100ms
               graphql    29.000  i/100ms
  Calculating -------------------------------------
               bluejay      2.001k (± 4.6%) i/s -      9.990k in   5.003540s
               graphql    307.259  (± 2.0%) i/s -      1.537k in   5.004389s
  
  Comparison:
               bluejay:     2001.0 i/s
               graphql:      307.3 i/s - 6.51x  (± 0.00) slower
  
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
               bluejay   213.000  i/100ms
               graphql    26.000  i/100ms
  Calculating -------------------------------------
               bluejay      2.114k (± 2.4%) i/s -     10.650k in   5.041848s
               graphql    264.455  (± 1.9%) i/s -      1.326k in   5.016031s
  
  Comparison:
               bluejay:     2113.5 i/s
               graphql:      264.5 i/s - 7.99x  (± 0.00) slower
  
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
               graphql   490.000  i/100ms
               bluejay     5.776k i/100ms
  Calculating -------------------------------------
               graphql      4.954k (± 1.6%) i/s -     24.990k in   5.045508s
               bluejay     57.782k (± 2.1%) i/s -    294.576k in   5.100498s
  
  Comparison:
               bluejay:    57781.9 i/s
               graphql:     4954.2 i/s - 11.66x  (± 0.00) slower
  
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
               graphql   322.000  i/100ms
               bluejay     5.771k i/100ms
  Calculating -------------------------------------
               graphql      3.229k (± 3.1%) i/s -     16.422k in   5.090853s
               bluejay     57.412k (± 2.5%) i/s -    288.550k in   5.029144s
  
  Comparison:
               bluejay:    57411.9 i/s
               graphql:     3229.3 i/s - 17.78x  (± 0.00) slower
  
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
               graphql   276.000  i/100ms
               bluejay     7.498k i/100ms
  Calculating -------------------------------------
               graphql      2.348k (±11.8%) i/s -     11.592k in   5.011382s
               bluejay     72.191k (± 1.8%) i/s -    367.402k in   5.090947s
  
  Comparison:
               bluejay:    72190.7 i/s
               graphql:     2348.4 i/s - 30.74x  (± 0.00) slower
  
  Profiling Ruby memory allocations:
  Calculating -------------------------------------
               graphql    56.872k memsize (     0.000  retained)
                         518.000  objects (     0.000  retained)
                          50.000  strings (     0.000  retained)
               bluejay   640.000  memsize (     0.000  retained)
                           1.000  objects (     0.000  retained)
                           1.000  strings (     0.000  retained)
  
  Comparison:
               bluejay:        640 allocated
               graphql:      56872 allocated - 88.86x more
  ```
</details>

<details>
  <summary>Schema dump (Ruby 3.2, YJIT disabled)</summary>

  ```
  Profiling IPS:
  Warming up --------------------------------------
               graphql   202.000  i/100ms
               bluejay     5.212k i/100ms
  Calculating -------------------------------------
               graphql      1.982k (± 9.1%) i/s -      9.898k in   5.041775s
               bluejay     73.624k (± 2.4%) i/s -    370.052k in   5.029246s
  
  Comparison:
               bluejay:    73624.0 i/s
               graphql:     1981.9 i/s - 37.15x  (± 0.00) slower
  
  Profiling Ruby memory allocations:
  Calculating -------------------------------------
               graphql    56.872k memsize (   944.000  retained)
                         518.000  objects (    12.000  retained)
                          50.000  strings (     4.000  retained)
               bluejay   640.000  memsize (     0.000  retained)
                           1.000  objects (     0.000  retained)
                           1.000  strings (     0.000  retained)
  
  Comparison:
               bluejay:        640 allocated
               graphql:      56872 allocated - 88.86x more
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
