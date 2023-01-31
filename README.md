# `bluejay-rb`

> **Warning**
> This project is still very early in its development and should be considered highly unstable and experimental. It is incomplete and not ready for production use.

> **Note**
> In an effort to get a working proof-of-concept, documentation and test coverage has been sacrificied. This will be corrected prior to a stable release.

`bluejay-rb` is a GraphQL engine for Ruby written primarily in Rust through the use of [`magnus`](https://github.com/matsadler/magnus).

## Goal

`bluejay-rb`'s goal is to provide a lightning fast GraphQL engine with first-class integration with [Sorbet](https://sorbet.org/). If you do not care deeply about one or both of these goals, then you should seriously consider using the excellent [`graphql-ruby`](https://graphql-ruby.org) gem, which has many more features than `bluejay-rb` in addition to being production-ready and much more customizable.

## Benchmarks

Some benchmarks comparing the performance of `bluejay-rb` with `graphql-ruby` are located at [`/bench`](/bench). The results of some of these benchmarks are included below.

<details>
  <summary>Execute (Ruby 3.2, YJIT enabled)</summary>
  
  ```
  Profiling IPS:
  Warming up --------------------------------------
              graphql    80.000  i/100ms
              bluejay   814.000  i/100ms
  Calculating -------------------------------------
              graphql    793.000  (± 1.5%) i/s -      4.000k in   5.045323s
              bluejay      8.106k (± 1.7%) i/s -     40.700k in   5.022282s

  Comparison:
              bluejay:     8106.5 i/s
              graphql:      793.0 i/s - 10.22x  (± 0.00) slower

  Profiling Ruby memory allocations:
  Calculating -------------------------------------
              graphql   107.688k memsize (   168.000  retained)
                          1.080k objects (     1.000  retained)
                          26.000  strings (     0.000  retained)
              bluejay     6.400k memsize (   168.000  retained)
                          55.000  objects (     1.000  retained)
                          12.000  strings (     0.000  retained)

  Comparison:
              bluejay:       6400 allocated
              graphql:     107688 allocated - 16.83x more
  ```
</details>

<details>
  <summary>Execute (Ruby 3.2, YJIT disabled)</summary>
  
  ```
  Profiling IPS:
  Warming up --------------------------------------
              graphql    35.000  i/100ms
              bluejay   822.000  i/100ms
  Calculating -------------------------------------
              graphql    326.806  (±14.4%) i/s -      1.610k in   5.034056s
              bluejay      7.916k (± 7.0%) i/s -     39.456k in   5.013297s

  Comparison:
              bluejay:     7916.4 i/s
              graphql:      326.8 i/s - 24.22x  (± 0.00) slower

  Profiling Ruby memory allocations:
  Calculating -------------------------------------
              graphql   107.688k memsize (   168.000  retained)
                          1.080k objects (     1.000  retained)
                          26.000  strings (     0.000  retained)
              bluejay     6.400k memsize (   168.000  retained)
                          55.000  objects (     1.000  retained)
                          12.000  strings (     0.000  retained)

  Comparison:
              bluejay:       6400 allocated
              graphql:     107688 allocated - 16.83x more
  ```
</details>

<details>
  <summary>Parse small (Ruby 3.2, YJIT enabled)</summary>
  
  ```
  Profiling IPS:
  Warming up --------------------------------------
              bluejay    42.616k i/100ms
              graphql   751.000  i/100ms
  Calculating -------------------------------------
              bluejay    419.058k (± 1.2%) i/s -      2.131M in   5.085492s
              graphql      7.483k (± 1.7%) i/s -     37.550k in   5.019630s

  Comparison:
              bluejay:   419058.2 i/s
              graphql:     7482.8 i/s - 56.00x  (± 0.00) slower

  Profiling Ruby memory allocations:
  Calculating -------------------------------------
              bluejay     0.000  memsize (     0.000  retained)
                          0.000  objects (     0.000  retained)
                          0.000  strings (     0.000  retained)
              graphql    10.576k memsize (     4.320k retained)
                        139.000  objects (    55.000  retained)
                          10.000  strings (     7.000  retained)

  Comparison:
              bluejay:          0 allocated
              graphql:      10576 allocated - Infx more
  ```
</details>

<details>
  <summary>Parse small (Ruby 3.2, YJIT disabled)</summary>
  
  ```
  Profiling IPS:
  Warming up --------------------------------------
              bluejay    42.010k i/100ms
              graphql   610.000  i/100ms
  Calculating -------------------------------------
              bluejay    412.413k (± 4.9%) i/s -      2.058M in   5.005818s
              graphql      5.862k (± 5.5%) i/s -     29.280k in   5.010455s

  Comparison:
              bluejay:   412412.6 i/s
              graphql:     5862.1 i/s - 70.35x  (± 0.00) slower

  Profiling Ruby memory allocations:
  Calculating -------------------------------------
              bluejay     0.000  memsize (     0.000  retained)
                          0.000  objects (     0.000  retained)
                          0.000  strings (     0.000  retained)
              graphql    10.576k memsize (     1.320k retained)
                        139.000  objects (    24.000  retained)
                          10.000  strings (     6.000  retained)

  Comparison:
              bluejay:          0 allocated
              graphql:      10576 allocated - Infx more
  ```
</details>

<details>
  <summary>Parse large (Ruby 3.2, YJIT enabled)</summary>
  
  ```
  Profiling IPS:
  Warming up --------------------------------------
              bluejay   216.000  i/100ms
              graphql     2.000  i/100ms
  Calculating -------------------------------------
              bluejay      2.148k (± 1.4%) i/s -     10.800k in   5.029772s
              graphql     28.787  (± 0.0%) i/s -    144.000  in   5.003419s

  Comparison:
              bluejay:     2147.7 i/s
              graphql:       28.8 i/s - 74.61x  (± 0.00) slower

  Profiling Ruby memory allocations:
  Calculating -------------------------------------
              bluejay     0.000  memsize (     0.000  retained)
                          0.000  objects (     0.000  retained)
                          0.000  strings (     0.000  retained)
              graphql     1.984M memsize (   701.936k retained)
                          31.056k objects (     8.970k retained)
                          50.000  strings (    50.000  retained)

  Comparison:
              bluejay:          0 allocated
              graphql:    1984432 allocated - Infx more
  ```
</details>

<details>
  <summary>Parse large (Ruby 3.2, YJIT disabled)</summary>
  
  ```
  Profiling IPS:
  Warming up --------------------------------------
              bluejay   188.000  i/100ms
              graphql     2.000  i/100ms
  Calculating -------------------------------------
              bluejay      2.167k (± 1.8%) i/s -     10.904k in   5.034545s
              graphql     28.417  (± 3.5%) i/s -    144.000  in   5.069022s

  Comparison:
              bluejay:     2166.6 i/s
              graphql:       28.4 i/s - 76.24x  (± 0.00) slower

  Profiling Ruby memory allocations:
  Calculating -------------------------------------
              bluejay     0.000  memsize (     0.000  retained)
                          0.000  objects (     0.000  retained)
                          0.000  strings (     0.000  retained)
              graphql     1.984M memsize (   316.016k retained)
                          31.056k objects (     4.151k retained)
                          50.000  strings (    50.000  retained)

  Comparison:
              bluejay:          0 allocated
              graphql:    1984432 allocated - Infx more
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
