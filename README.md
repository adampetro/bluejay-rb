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
               graphql   177.000  i/100ms
               bluejay     1.504k i/100ms
  Calculating -------------------------------------
               graphql      1.850k (± 3.4%) i/s -      9.381k in   5.077797s
               bluejay     15.023k (± 1.5%) i/s -     75.200k in   5.006907s
  
  Comparison:
               bluejay:    15022.6 i/s
               graphql:     1850.0 i/s - 8.12x  (± 0.00) slower
  
  Profiling Ruby memory allocations:
  Calculating -------------------------------------
               graphql    46.560k memsize (     0.000  retained)
                         421.000  objects (     0.000  retained)
                          12.000  strings (     0.000  retained)
               bluejay     5.216k memsize (    40.000  retained)
                          39.000  objects (     1.000  retained)
                           0.000  strings (     0.000  retained)
  
  Comparison:
               bluejay:       5216 allocated
               graphql:      46560 allocated - 8.93x more
  ```
</details>

<details>
  <summary>Parse + Execute (Ruby 3.2, YJIT disabled)</summary>

  ```
  Profiling IPS:
  Warming up --------------------------------------
               graphql    74.000  i/100ms
               bluejay     1.555k i/100ms
  Calculating -------------------------------------
               graphql    762.448  (± 2.5%) i/s -      3.848k in   5.050512s
               bluejay     15.395k (± 2.2%) i/s -     77.750k in   5.052958s
  
  Comparison:
               bluejay:    15395.4 i/s
               graphql:      762.4 i/s - 20.19x  (± 0.00) slower
  
  Profiling Ruby memory allocations:
  Calculating -------------------------------------
               graphql    46.560k memsize (    21.752k retained)
                         421.000  objects (   197.000  retained)
                          12.000  strings (    12.000  retained)
               bluejay     5.216k memsize (   336.000  retained)
                          39.000  objects (     2.000  retained)
                           0.000  strings (     0.000  retained)
  
  Comparison:
               bluejay:       5216 allocated
               graphql:      46560 allocated - 8.93x more
  ```
</details>

<details>
  <summary>Parse small (Ruby 3.2, YJIT enabled)</summary>

  ```
  Profiling IPS:
  Warming up --------------------------------------
               bluejay    37.858k i/100ms
               graphql     6.453k i/100ms
  Calculating -------------------------------------
               bluejay    375.305k (± 2.4%) i/s -      1.893M in   5.046960s
               graphql     64.386k (± 0.7%) i/s -    322.650k in   5.011463s
  
  Comparison:
               bluejay:   375305.2 i/s
               graphql:    64385.7 i/s - 5.83x  (± 0.00) slower
  
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
               bluejay    38.130k i/100ms
               graphql     5.614k i/100ms
  Calculating -------------------------------------
               bluejay    383.634k (± 1.0%) i/s -      1.945M in   5.069520s
               graphql     55.521k (± 3.9%) i/s -    280.700k in   5.065237s
  
  Comparison:
               bluejay:   383633.7 i/s
               graphql:    55520.6 i/s - 6.91x  (± 0.00) slower
  
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
               bluejay   202.000  i/100ms
               graphql    29.000  i/100ms
  Calculating -------------------------------------
               bluejay      2.035k (± 1.0%) i/s -     10.302k in   5.063196s
               graphql    290.481  (± 3.1%) i/s -      1.479k in   5.097784s
  
  Comparison:
               bluejay:     2034.9 i/s
               graphql:      290.5 i/s - 7.01x  (± 0.00) slower
  
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
               bluejay   204.000  i/100ms
               graphql    25.000  i/100ms
  Calculating -------------------------------------
               bluejay      2.023k (± 1.0%) i/s -     10.200k in   5.043670s
               graphql    251.365  (± 4.4%) i/s -      1.275k in   5.084231s
  
  Comparison:
               bluejay:     2022.5 i/s
               graphql:      251.4 i/s - 8.05x  (± 0.00) slower
  
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
               graphql   465.000  i/100ms
               bluejay     5.625k i/100ms
  Calculating -------------------------------------
               graphql      4.669k (± 1.9%) i/s -     23.715k in   5.080598s
               bluejay     55.302k (± 1.4%) i/s -    281.250k in   5.086730s
  
  Comparison:
               bluejay:    55302.4 i/s
               graphql:     4669.5 i/s - 11.84x  (± 0.00) slower
  
  Profiling Ruby memory allocations:
  Calculating -------------------------------------
               graphql    33.392k memsize (    12.160k retained)
                         383.000  objects (   151.000  retained)
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
               graphql   294.000  i/100ms
               bluejay     5.608k i/100ms
  Calculating -------------------------------------
               graphql      3.022k (± 1.3%) i/s -     15.288k in   5.059873s
               bluejay     55.294k (± 3.6%) i/s -    280.400k in   5.079344s
  
  Comparison:
               bluejay:    55294.2 i/s
               graphql:     3021.9 i/s - 18.30x  (± 0.00) slower
  
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
               graphql   263.000  i/100ms
               bluejay    15.639k i/100ms
  Calculating -------------------------------------
               graphql      2.607k (± 2.0%) i/s -     13.150k in   5.046483s
               bluejay    151.815k (± 3.1%) i/s -    766.311k in   5.053265s
  
  Comparison:
               bluejay:   151815.3 i/s
               graphql:     2606.9 i/s - 58.24x  (± 0.00) slower
  
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
               graphql   198.000  i/100ms
               bluejay    14.721k i/100ms
  Calculating -------------------------------------
               graphql      2.003k (± 1.3%) i/s -     10.098k in   5.042412s
               bluejay    155.296k (± 2.7%) i/s -    780.213k in   5.028210s
  
  Comparison:
               bluejay:   155295.8 i/s
               graphql:     2002.9 i/s - 77.53x  (± 0.00) slower
  
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
