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
               graphql   138.000  i/100ms
               bluejay     1.459k i/100ms
  Calculating -------------------------------------
               graphql      1.509k (±12.6%) i/s -      7.452k in   5.012654s
               bluejay     13.503k (± 9.9%) i/s -     67.114k in   5.018226s
  
  Comparison:
               bluejay:    13502.8 i/s
               graphql:     1509.4 i/s - 8.95x  (± 0.00) slower
  
  Profiling Ruby memory allocations:
  Calculating -------------------------------------
               graphql    46.560k memsize (   168.000  retained)
                         421.000  objects (     1.000  retained)
                          12.000  strings (     0.000  retained)
               bluejay     5.256k memsize (   208.000  retained)
                          40.000  objects (     2.000  retained)
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
               graphql    53.000  i/100ms
               bluejay     1.246k i/100ms
  Calculating -------------------------------------
               graphql    663.251  (±16.3%) i/s -      3.233k in   5.015141s
               bluejay     14.307k (±12.7%) i/s -     71.022k in   5.038985s
  
  Comparison:
               bluejay:    14306.9 i/s
               graphql:      663.3 i/s - 21.57x  (± 0.00) slower
  
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
               graphql   148.000  i/100ms
               bluejay     1.030k i/100ms
  Calculating -------------------------------------
               graphql      1.278k (±15.5%) i/s -      6.364k in   5.109074s
               bluejay      8.902k (±16.0%) i/s -     44.290k in   5.127437s
  
  Comparison:
               bluejay:     8902.5 i/s
               graphql:     1278.0 i/s - 6.97x  (± 0.00) slower
  
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
               graphql    72.000  i/100ms
               bluejay   975.000  i/100ms
  Calculating -------------------------------------
               graphql    692.483  (±14.7%) i/s -      3.456k in   5.105534s
               bluejay      8.055k (±11.1%) i/s -     39.975k in   5.024806s
  
  Comparison:
               bluejay:     8054.7 i/s
               graphql:      692.5 i/s - 11.63x  (± 0.00) slower
  
  Profiling Ruby memory allocations:
  Calculating -------------------------------------
               graphql    97.688k memsize (    30.736k retained)
                         884.000  objects (   319.000  retained)
                           6.000  strings (     5.000  retained)
               bluejay    12.600k memsize (   496.000  retained)
                         152.000  objects (     6.000  retained)
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
               bluejay    33.586k i/100ms
               graphql     6.166k i/100ms
  Calculating -------------------------------------
               bluejay    372.709k (± 2.4%) i/s -      1.881M in   5.049283s
               graphql     63.334k (± 5.4%) i/s -    320.632k in   5.078346s
  
  Comparison:
               bluejay:   372709.2 i/s
               graphql:    63334.0 i/s - 5.88x  (± 0.00) slower
  
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
               bluejay    37.375k i/100ms
               graphql     5.378k i/100ms
  Calculating -------------------------------------
               bluejay    363.767k (± 5.7%) i/s -      1.831M in   5.052379s
               graphql     53.200k (± 5.8%) i/s -    268.900k in   5.074764s
  
  Comparison:
               bluejay:   363766.8 i/s
               graphql:    53200.0 i/s - 6.84x  (± 0.00) slower
  
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
               bluejay   161.000  i/100ms
               graphql    24.000  i/100ms
  Calculating -------------------------------------
               bluejay      1.905k (± 2.7%) i/s -      9.660k in   5.075560s
               graphql    261.200  (±10.3%) i/s -      1.296k in   5.037454s
  
  Comparison:
               bluejay:     1904.7 i/s
               graphql:      261.2 i/s - 7.29x  (± 0.00) slower
  
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
               bluejay   188.000  i/100ms
               graphql    22.000  i/100ms
  Calculating -------------------------------------
               bluejay      1.985k (± 4.0%) i/s -      9.964k in   5.027172s
               graphql    259.536  (± 3.1%) i/s -      1.298k in   5.006371s
  
  Comparison:
               bluejay:     1985.3 i/s
               graphql:      259.5 i/s - 7.65x  (± 0.00) slower
  
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
               graphql   500.000  i/100ms
               bluejay     5.722k i/100ms
  Calculating -------------------------------------
               graphql      4.419k (±10.5%) i/s -     22.000k in   5.040646s
               bluejay     53.549k (± 8.0%) i/s -    268.934k in   5.058196s
  
  Comparison:
               bluejay:    53549.0 i/s
               graphql:     4418.7 i/s - 12.12x  (± 0.00) slower
  
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
               graphql   321.000  i/100ms
               bluejay     5.736k i/100ms
  Calculating -------------------------------------
               graphql      3.138k (± 3.2%) i/s -     15.729k in   5.016910s
               bluejay     57.553k (± 1.2%) i/s -    292.536k in   5.083639s
  
  Comparison:
               bluejay:    57552.7 i/s
               graphql:     3138.5 i/s - 18.34x  (± 0.00) slower
  
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
               graphql   193.000  i/100ms
               bluejay     4.737k i/100ms
  Calculating -------------------------------------
               graphql      2.081k (± 2.5%) i/s -     10.422k in   5.010179s
               bluejay     48.945k (± 2.6%) i/s -    246.324k in   5.036105s
  
  Comparison:
               bluejay:    48944.7 i/s
               graphql:     2081.4 i/s - 23.51x  (± 0.00) slower
  
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
               graphql   159.000  i/100ms
               bluejay     4.896k i/100ms
  Calculating -------------------------------------
               graphql      1.572k (± 3.3%) i/s -      7.950k in   5.061772s
               bluejay     48.191k (± 3.0%) i/s -    244.800k in   5.084702s
  
  Comparison:
               bluejay:    48190.8 i/s
               graphql:     1572.4 i/s - 30.65x  (± 0.00) slower
  
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
