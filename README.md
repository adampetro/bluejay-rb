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
              graphql   106.000  i/100ms
              bluejay   822.000  i/100ms
  Calculating -------------------------------------
              graphql    970.223  (±11.2%) i/s -      4.876k in   5.093229s
              bluejay      7.860k (± 9.5%) i/s -     39.456k in   5.068449s

  Comparison:
              bluejay:     7860.4 i/s
              graphql:      970.2 i/s - 8.10x  (± 0.00) slower

  Profiling Ruby memory allocations:
  Calculating -------------------------------------
              graphql    61.520k memsize (   168.000  retained)
                        625.000  objects (     1.000  retained)
                          31.000  strings (     0.000  retained)
              bluejay     5.800k memsize (   168.000  retained)
                          44.000  objects (     1.000  retained)
                          0.000  strings (     0.000  retained)

  Comparison:
              bluejay:       5800 allocated
              graphql:      61520 allocated - 10.61x more
  ```
</details>

<details>
  <summary>Execute (Ruby 3.2, YJIT disabled)</summary>

  ```
  Profiling IPS:
  Warming up --------------------------------------
              graphql    46.000  i/100ms
              bluejay   867.000  i/100ms
  Calculating -------------------------------------
              graphql    442.012  (± 8.8%) i/s -      2.208k in   5.038696s
              bluejay      8.605k (± 2.2%) i/s -     43.350k in   5.040408s

  Comparison:
              bluejay:     8605.1 i/s
              graphql:      442.0 i/s - 19.47x  (± 0.00) slower

  Profiling Ruby memory allocations:
  Calculating -------------------------------------
              graphql    61.520k memsize (    26.496k retained)
                        625.000  objects (   202.000  retained)
                          31.000  strings (     1.000  retained)
              bluejay     5.800k memsize (     5.224k retained)
                          44.000  objects (    36.000  retained)
                          0.000  strings (     0.000  retained)

  Comparison:
              bluejay:       5800 allocated
              graphql:      61520 allocated - 10.61x more
  ```
</details>

<details>
  <summary>Parse small (Ruby 3.2, YJIT enabled)</summary>

  ```
  Profiling IPS:
  Warming up --------------------------------------
              bluejay    42.247k i/100ms
              graphql   742.000  i/100ms
  Calculating -------------------------------------
              bluejay    397.214k (± 7.0%) i/s -      1.986M in   5.024209s
              graphql      6.979k (±11.4%) i/s -     34.874k in   5.063675s

  Comparison:
              bluejay:   397214.2 i/s
              graphql:     6978.5 i/s - 56.92x  (± 0.00) slower

  Profiling Ruby memory allocations:
  Calculating -------------------------------------
              bluejay     0.000  memsize (     0.000  retained)
                          0.000  objects (     0.000  retained)
                          0.000  strings (     0.000  retained)
              graphql    11.936k memsize (     4.360k retained)
                        176.000  objects (    56.000  retained)
                          17.000  strings (     8.000  retained)

  Comparison:
              bluejay:          0 allocated
              graphql:      11936 allocated - Infx more
  ```
</details>

<details>
  <summary>Parse small (Ruby 3.2, YJIT disabled)</summary>

  ```
  Profiling IPS:
  Warming up --------------------------------------
              bluejay    41.719k i/100ms
              graphql   462.000  i/100ms
  Calculating -------------------------------------
              bluejay    386.029k (± 8.2%) i/s -      1.919M in   5.005650s
              graphql      5.840k (± 9.9%) i/s -     29.106k in   5.034106s

  Comparison:
              bluejay:   386028.7 i/s
              graphql:     5840.1 i/s - 66.10x  (± 0.00) slower

  Profiling Ruby memory allocations:
  Calculating -------------------------------------
              bluejay     0.000  memsize (     0.000  retained)
                          0.000  objects (     0.000  retained)
                          0.000  strings (     0.000  retained)
              graphql    11.936k memsize (     1.320k retained)
                        176.000  objects (    24.000  retained)
                          17.000  strings (     6.000  retained)

  Comparison:
              bluejay:          0 allocated
              graphql:      11936 allocated - Infx more
  ```
</details>

<details>
  <summary>Parse large (Ruby 3.2, YJIT enabled)</summary>

  ```
  Profiling IPS:
  Warming up --------------------------------------
              bluejay   187.000  i/100ms
              graphql     4.000  i/100ms
  Calculating -------------------------------------
              bluejay      2.093k (± 6.0%) i/s -     10.472k in   5.023643s
              graphql     46.307  (± 8.6%) i/s -    232.000  in   5.043557s

  Comparison:
              bluejay:     2092.8 i/s
              graphql:       46.3 i/s - 45.19x  (± 0.00) slower

  Profiling Ruby memory allocations:
  Calculating -------------------------------------
              bluejay     0.000  memsize (     0.000  retained)
                          0.000  objects (     0.000  retained)
                          0.000  strings (     0.000  retained)
              graphql     2.025M memsize (   701.976k retained)
                          31.999k objects (     8.971k retained)
                          50.000  strings (    50.000  retained)

  Comparison:
              bluejay:          0 allocated
              graphql:    2025352 allocated - Infx more
  ```
</details>

<details>
  <summary>Parse large (Ruby 3.2, YJIT disabled)</summary>

  ```
  Profiling IPS:
  Warming up --------------------------------------
              bluejay   207.000  i/100ms
              graphql     3.000  i/100ms
  Calculating -------------------------------------
              bluejay      2.047k (± 8.8%) i/s -     10.350k in   5.102000s
              graphql     41.996  (± 9.5%) i/s -    210.000  in   5.050012s

  Comparison:
              bluejay:     2047.0 i/s
              graphql:       42.0 i/s - 48.74x  (± 0.00) slower

  Profiling Ruby memory allocations:
  Calculating -------------------------------------
              bluejay     0.000  memsize (     0.000  retained)
                          0.000  objects (     0.000  retained)
                          0.000  strings (     0.000  retained)
              graphql     2.025M memsize (   316.016k retained)
                          31.999k objects (     4.151k retained)
                          50.000  strings (    50.000  retained)

  Comparison:
              bluejay:          0 allocated
              graphql:    2025352 allocated - Infx more
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
