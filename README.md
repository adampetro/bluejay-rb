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
               bluejay     1.513k i/100ms
  Calculating -------------------------------------
               graphql      1.783k (± 0.9%) i/s -      9.048k in   5.075464s
               bluejay     15.176k (± 0.6%) i/s -     77.163k in   5.084700s

  Comparison:
               bluejay:    15176.1 i/s
               graphql:     1782.8 i/s - 8.51x  (± 0.00) slower

  Profiling Ruby memory allocations:
  Calculating -------------------------------------
               graphql    46.560k memsize (     2.568k retained)
                         421.000  objects (    40.000  retained)
                          12.000  strings (    11.000  retained)
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
               graphql    72.000  i/100ms
               bluejay     1.544k i/100ms
  Calculating -------------------------------------
               graphql    725.161  (± 2.2%) i/s -      3.672k in   5.066523s
               bluejay     15.399k (± 0.7%) i/s -     77.200k in   5.013638s

  Comparison:
               bluejay:    15398.8 i/s
               graphql:      725.2 i/s - 21.23x  (± 0.00) slower

  Profiling Ruby memory allocations:
  Calculating -------------------------------------
               graphql    46.560k memsize (    21.752k retained)
                         421.000  objects (   197.000  retained)
                          12.000  strings (    12.000  retained)
               bluejay     5.216k memsize (     1.088k retained)
                          39.000  objects (     7.000  retained)
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
               bluejay    36.583k i/100ms
               graphql     6.392k i/100ms
  Calculating -------------------------------------
               bluejay    373.852k (± 0.9%) i/s -      1.902M in   5.088886s
               graphql     63.906k (± 0.6%) i/s -    319.600k in   5.001306s

  Comparison:
               bluejay:   373851.7 i/s
               graphql:    63905.8 i/s - 5.85x  (± 0.00) slower

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
               bluejay    37.310k i/100ms
               graphql     5.508k i/100ms
  Calculating -------------------------------------
               bluejay    373.834k (± 2.3%) i/s -      1.903M in   5.093165s
               graphql     55.222k (± 0.6%) i/s -    280.908k in   5.087098s

  Comparison:
               bluejay:   373834.3 i/s
               graphql:    55221.6 i/s - 6.77x  (± 0.00) slower

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
               bluejay   194.000  i/100ms
               graphql    27.000  i/100ms
  Calculating -------------------------------------
               bluejay      1.967k (± 0.7%) i/s -      9.894k in   5.030400s
               graphql    272.205  (± 1.1%) i/s -      1.377k in   5.059300s

  Comparison:
               bluejay:     1966.9 i/s
               graphql:      272.2 i/s - 7.23x  (± 0.00) slower

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
               bluejay   195.000  i/100ms
               graphql    25.000  i/100ms
  Calculating -------------------------------------
               bluejay      1.959k (± 2.3%) i/s -      9.945k in   5.080629s
               graphql    250.819  (± 0.4%) i/s -      1.275k in   5.083483s

  Comparison:
               bluejay:     1958.7 i/s
               graphql:      250.8 i/s - 7.81x  (± 0.00) slower

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
               graphql   435.000  i/100ms
               bluejay     5.400k i/100ms
  Calculating -------------------------------------
               graphql      4.379k (± 0.8%) i/s -     22.185k in   5.066584s
               bluejay     54.198k (± 0.9%) i/s -    275.400k in   5.081749s

  Comparison:
               bluejay:    54198.1 i/s
               graphql:     4379.0 i/s - 12.38x  (± 0.00) slower

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
               graphql   291.000  i/100ms
               bluejay     5.485k i/100ms
  Calculating -------------------------------------
               graphql      2.901k (± 2.4%) i/s -     14.550k in   5.018700s
               bluejay     54.758k (± 0.5%) i/s -    274.250k in   5.008560s

  Comparison:
               bluejay:    54757.9 i/s
               graphql:     2901.1 i/s - 18.87x  (± 0.00) slower

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
               graphql   253.000  i/100ms
               bluejay    14.846k i/100ms
  Calculating -------------------------------------
               graphql      2.518k (± 0.7%) i/s -     12.650k in   5.024461s
               bluejay    148.053k (± 0.4%) i/s -    742.300k in   5.013849s

  Comparison:
               bluejay:   148052.8 i/s
               graphql:     2517.8 i/s - 58.80x  (± 0.00) slower

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
               graphql   189.000  i/100ms
               bluejay    15.120k i/100ms
  Calculating -------------------------------------
               graphql      1.860k (± 5.6%) i/s -      9.261k in   5.002411s
               bluejay    151.588k (± 0.6%) i/s -    771.120k in   5.087124s

  Comparison:
               bluejay:   151587.8 i/s
               graphql:     1859.5 i/s - 81.52x  (± 0.00) slower

  Profiling Ruby memory allocations:
  Calculating -------------------------------------
               graphql    56.872k memsize (     1.584k retained)
                         518.000  objects (    13.000  retained)
                          50.000  strings (     5.000  retained)
               bluejay   640.000  memsize (   640.000  retained)
                           1.000  objects (     1.000  retained)
                           1.000  strings (     1.000  retained)

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
