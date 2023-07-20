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
               graphql   158.000  i/100ms
               bluejay     1.764k i/100ms
  Calculating -------------------------------------
               graphql      1.928k (± 2.7%) i/s -      9.638k in   5.003328s
               bluejay     17.899k (± 2.7%) i/s -     89.964k in   5.029921s
  
  Comparison:
               bluejay:    17899.5 i/s
               graphql:     1927.9 i/s - 9.28x  slower
  
  Profiling Ruby memory allocations:
  Calculating -------------------------------------
               graphql    45.944k memsize (     2.736k retained)
                         421.000  objects (    41.000  retained)
                          12.000  strings (    11.000  retained)
               bluejay     5.256k memsize (   208.000  retained)
                          40.000  objects (     2.000  retained)
                           0.000  strings (     0.000  retained)
  
  Comparison:
               bluejay:       5256 allocated
               graphql:      45944 allocated - 8.74x more
  ```
</details>

<details>
  <summary>Parse + Execute (Ruby 3.2, YJIT disabled)</summary>

  ```
  Profiling IPS:
  Warming up --------------------------------------
               graphql    79.000  i/100ms
               bluejay     1.804k i/100ms
  Calculating -------------------------------------
               graphql    781.860  (± 2.2%) i/s -      3.950k in   5.054411s
               bluejay     18.017k (± 1.6%) i/s -     90.200k in   5.007842s
  
  Comparison:
               bluejay:    18016.7 i/s
               graphql:      781.9 i/s - 23.04x  slower
  
  Profiling Ruby memory allocations:
  Calculating -------------------------------------
               graphql    45.944k memsize (    21.752k retained)
                         421.000  objects (   197.000  retained)
                          12.000  strings (    12.000  retained)
               bluejay     5.256k memsize (     5.056k retained)
                          40.000  objects (    35.000  retained)
                           0.000  strings (     0.000  retained)
  
  Comparison:
               bluejay:       5256 allocated
               graphql:      45944 allocated - 8.74x more
  ```
</details>

<details>
  <summary>Parse + Execute with large variables (Ruby 3.2, YJIT enabled)</summary>

  ```
  Profiling IPS:
  Warming up --------------------------------------
               graphql   166.000  i/100ms
               bluejay     1.101k i/100ms
  Calculating -------------------------------------
               graphql      1.642k (± 2.1%) i/s -      8.300k in   5.055691s
               bluejay     10.876k (± 1.8%) i/s -     55.050k in   5.063054s
  
  Comparison:
               bluejay:    10876.3 i/s
               graphql:     1642.4 i/s - 6.62x  slower
  
  Profiling Ruby memory allocations:
  Calculating -------------------------------------
               graphql    97.728k memsize (    40.000  retained)
                         885.000  objects (     1.000  retained)
                           6.000  strings (     0.000  retained)
               bluejay    15.640k memsize (    80.000  retained)
                         228.000  objects (     2.000  retained)
                          15.000  strings (     1.000  retained)
  
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
               graphql    79.000  i/100ms
               bluejay     1.009k i/100ms
  Calculating -------------------------------------
               graphql    841.882  (± 1.5%) i/s -      4.266k in   5.068484s
               bluejay     10.163k (± 2.1%) i/s -     51.459k in   5.065688s
  
  Comparison:
               bluejay:    10163.2 i/s
               graphql:      841.9 i/s - 12.07x  slower
  
  Profiling Ruby memory allocations:
  Calculating -------------------------------------
               graphql    97.688k memsize (    30.736k retained)
                         884.000  objects (   319.000  retained)
                           6.000  strings (     5.000  retained)
               bluejay    12.600k memsize (   536.000  retained)
                         152.000  objects (     6.000  retained)
                           8.000  strings (     1.000  retained)
  
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
               bluejay    39.288k i/100ms
               graphql     6.906k i/100ms
  Calculating -------------------------------------
               bluejay    397.916k (± 1.4%) i/s -      2.004M in   5.036417s
               graphql     67.671k (± 4.9%) i/s -    338.394k in   5.015897s
  
  Comparison:
               bluejay:   397915.9 i/s
               graphql:    67671.2 i/s - 5.88x  slower
  
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
               bluejay    37.618k i/100ms
               graphql     6.034k i/100ms
  Calculating -------------------------------------
               bluejay    403.347k (± 2.1%) i/s -      2.031M in   5.038540s
               graphql     59.928k (± 2.0%) i/s -    301.700k in   5.036461s
  
  Comparison:
               bluejay:   403347.5 i/s
               graphql:    59928.3 i/s - 6.73x  slower
  
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
               bluejay   203.000  i/100ms
               graphql    30.000  i/100ms
  Calculating -------------------------------------
               bluejay      2.106k (± 1.4%) i/s -     10.556k in   5.014156s
               graphql    308.161  (± 1.9%) i/s -      1.560k in   5.064324s
  
  Comparison:
               bluejay:     2105.7 i/s
               graphql:      308.2 i/s - 6.83x  slower
  
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
               bluejay   208.000  i/100ms
               graphql    27.000  i/100ms
  Calculating -------------------------------------
               bluejay      2.098k (± 1.6%) i/s -     10.608k in   5.057106s
               graphql    274.835  (± 1.8%) i/s -      1.377k in   5.011880s
  
  Comparison:
               bluejay:     2098.2 i/s
               graphql:      274.8 i/s - 7.63x  slower
  
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
               graphql   504.000  i/100ms
               bluejay     5.370k i/100ms
  Calculating -------------------------------------
               graphql      4.991k (± 6.2%) i/s -     25.200k in   5.073307s
               bluejay     45.612k (±11.5%) i/s -    225.540k in   5.057627s
  
  Comparison:
               bluejay:    45612.5 i/s
               graphql:     4991.1 i/s - 9.14x  slower
  
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
               graphql   299.000  i/100ms
               bluejay     4.726k i/100ms
  Calculating -------------------------------------
               graphql      2.953k (± 5.0%) i/s -     14.950k in   5.075697s
               bluejay     51.904k (± 4.0%) i/s -    259.930k in   5.016499s
  
  Comparison:
               bluejay:    51904.0 i/s
               graphql:     2953.1 i/s - 17.58x  slower
  
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
               graphql   189.000  i/100ms
               bluejay     4.228k i/100ms
  Calculating -------------------------------------
               graphql      1.929k (± 7.3%) i/s -      9.639k in   5.026688s
               bluejay     43.632k (± 4.6%) i/s -    219.856k in   5.050598s
  
  Comparison:
               bluejay:    43631.9 i/s
               graphql:     1929.2 i/s - 22.62x  slower
  
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
               graphql   157.000  i/100ms
               bluejay     4.662k i/100ms
  Calculating -------------------------------------
               graphql      1.562k (± 1.7%) i/s -      7.850k in   5.027157s
               bluejay     45.335k (± 3.3%) i/s -    228.438k in   5.044407s
  
  Comparison:
               bluejay:    45335.1 i/s
               graphql:     1561.9 i/s - 29.02x  slower
  
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
