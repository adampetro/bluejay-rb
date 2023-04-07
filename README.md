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

<details>
  <summary>Execute (Ruby 3.2, YJIT enabled)</summary>

  ```
  Profiling IPS:
  Warming up --------------------------------------
              bluejay    10.186k i/100ms
  Calculating -------------------------------------
              bluejay     94.905k (±12.6%) i/s -    468.556k in   5.041452s
  Profiling Ruby memory allocations:
  Calculating -------------------------------------
              bluejay   680.000  memsize (     0.000  retained)
                          17.000  objects (     0.000  retained)
                          6.000  strings (     0.000  retained)

  ruby -Ilib bench/execute.rb
  Profiling IPS:
  Warming up --------------------------------------
              graphql   126.000  i/100ms
              bluejay   886.000  i/100ms
  Calculating -------------------------------------
              graphql      1.251k (± 7.8%) i/s -      6.300k in   5.079402s
              bluejay      8.116k (±11.0%) i/s -     40.756k in   5.089903s

  Comparison:
              bluejay:     8115.5 i/s
              graphql:     1251.4 i/s - 6.49x  (± 0.00) slower

  Profiling Ruby memory allocations:
  Calculating -------------------------------------
              graphql    54.136k memsize (   168.000  retained)
                        480.000  objects (     1.000  retained)
                          22.000  strings (     0.000  retained)
              bluejay     5.800k memsize (   168.000  retained)
                          44.000  objects (     1.000  retained)
                          0.000  strings (     0.000  retained)

  Comparison:
              bluejay:       5800 allocated
              graphql:      54136 allocated - 9.33x more
  ```
</details>

<details>
  <summary>Execute (Ruby 3.2, YJIT disabled)</summary>

  ```
  Profiling IPS:
  Warming up --------------------------------------
              graphql    50.000  i/100ms
              bluejay   846.000  i/100ms
  Calculating -------------------------------------
              graphql    491.825  (± 5.5%) i/s -      2.450k in   5.000317s
              bluejay      8.338k (± 4.2%) i/s -     42.300k in   5.083268s

  Comparison:
              bluejay:     8338.4 i/s
              graphql:      491.8 i/s - 16.95x  (± 0.00) slower

  Profiling Ruby memory allocations:
  Calculating -------------------------------------
              graphql    54.136k memsize (    27.536k retained)
                        480.000  objects (   227.000  retained)
                          22.000  strings (    12.000  retained)
              bluejay     5.800k memsize (     5.264k retained)
                          44.000  objects (    37.000  retained)
                          0.000  strings (     0.000  retained)

  Comparison:
              bluejay:       5800 allocated
              graphql:      54136 allocated - 9.33x more
  ```
</details>

<details>
  <summary>Parse small (Ruby 3.2, YJIT enabled)</summary>

  ```
  Profiling IPS:
  Warming up --------------------------------------
              bluejay    35.213k i/100ms
              graphql     5.122k i/100ms
  Calculating -------------------------------------
              bluejay    372.142k (±12.2%) i/s -      1.866M in   5.101777s
              graphql     51.270k (±13.6%) i/s -    256.100k in   5.107247s

  Comparison:
              bluejay:   372141.7 i/s
              graphql:    51269.7 i/s - 7.26x  (± 0.00) slower

  Profiling Ruby memory allocations:
  Calculating -------------------------------------
              bluejay     0.000  memsize (     0.000  retained)
                          0.000  objects (     0.000  retained)
                          0.000  strings (     0.000  retained)
              graphql     6.192k memsize (     4.176k retained)
                          70.000  objects (    58.000  retained)
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
              bluejay    23.697k i/100ms
              graphql     4.703k i/100ms
  Calculating -------------------------------------
              bluejay    345.072k (±12.1%) i/s -      1.706M in   5.030643s
              graphql     43.589k (±13.5%) i/s -    216.338k in   5.067260s

  Comparison:
              bluejay:   345072.0 i/s
              graphql:    43589.3 i/s - 7.92x  (± 0.00) slower

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
              bluejay   183.000  i/100ms
              graphql    20.000  i/100ms
  Calculating -------------------------------------
              bluejay      1.875k (±10.3%) i/s -      9.333k in   5.038755s
              graphql    221.568  (±11.7%) i/s -      1.100k in   5.050325s

  Comparison:
              bluejay:     1875.3 i/s
              graphql:      221.6 i/s - 8.46x  (± 0.00) slower

  Profiling Ruby memory allocations:
  Calculating -------------------------------------
              bluejay     0.000  memsize (     0.000  retained)
                          0.000  objects (     0.000  retained)
                          0.000  strings (     0.000  retained)
              graphql     1.425M memsize (   928.624k retained)
                          16.001k objects (    13.044k retained)
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
              bluejay   167.000  i/100ms
              graphql    22.000  i/100ms
  Calculating -------------------------------------
              bluejay      1.810k (±12.2%) i/s -      9.018k in   5.080901s
              graphql    195.133  (± 8.7%) i/s -    990.000  in   5.115266s

  Comparison:
              bluejay:     1810.1 i/s
              graphql:      195.1 i/s - 9.28x  (± 0.00) slower

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
