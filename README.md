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
               graphql   179.000  i/100ms
               bluejay     1.195k i/100ms
  Calculating -------------------------------------
               graphql      1.844k (± 1.5%) i/s -      9.308k in   5.049845s
               bluejay     11.932k (± 1.5%) i/s -     59.750k in   5.008661s
  
  Comparison:
               bluejay:    11932.0 i/s
               graphql:     1843.6 i/s - 6.47x  (± 0.00) slower
  
  Profiling Ruby memory allocations:
  Calculating -------------------------------------
               graphql    46.560k memsize (   168.000  retained)
                         421.000  objects (     1.000  retained)
                          12.000  strings (     0.000  retained)
               bluejay     5.216k memsize (   208.000  retained)
                          39.000  objects (     2.000  retained)
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
               graphql    78.000  i/100ms
               bluejay     1.205k i/100ms
  Calculating -------------------------------------
               graphql    780.783  (± 1.4%) i/s -      3.978k in   5.095848s
               bluejay     12.084k (± 1.7%) i/s -     61.455k in   5.087355s
  
  Comparison:
               bluejay:    12083.7 i/s
               graphql:      780.8 i/s - 15.48x  (± 0.00) slower
  
  Profiling Ruby memory allocations:
  Calculating -------------------------------------
               graphql    46.560k memsize (    21.752k retained)
                         421.000  objects (   197.000  retained)
                          12.000  strings (    12.000  retained)
               bluejay     5.216k memsize (   920.000  retained)
                          39.000  objects (     6.000  retained)
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
               bluejay    39.110k i/100ms
               graphql     6.624k i/100ms
  Calculating -------------------------------------
               bluejay    397.493k (± 2.8%) i/s -      1.995M in   5.022378s
               graphql     66.486k (± 1.1%) i/s -    337.824k in   5.081785s
  
  Comparison:
               bluejay:   397492.6 i/s
               graphql:    66485.9 i/s - 5.98x  (± 0.00) slower
  
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
               bluejay    37.989k i/100ms
               graphql     5.789k i/100ms
  Calculating -------------------------------------
               bluejay    398.556k (± 1.9%) i/s -      2.013M in   5.053601s
               graphql     53.100k (±11.1%) i/s -    266.294k in   5.082801s
  
  Comparison:
               bluejay:   398556.4 i/s
               graphql:    53100.0 i/s - 7.51x  (± 0.00) slower
  
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
               bluejay   198.000  i/100ms
               graphql    25.000  i/100ms
  Calculating -------------------------------------
               bluejay      1.784k (±11.5%) i/s -      8.910k in   5.072242s
               graphql    264.855  (±12.5%) i/s -      1.300k in   5.004887s
  
  Comparison:
               bluejay:     1783.6 i/s
               graphql:      264.9 i/s - 6.73x  (± 0.00) slower
  
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
               bluejay   181.000  i/100ms
               graphql    18.000  i/100ms
  Calculating -------------------------------------
               bluejay      1.681k (±11.2%) i/s -      8.326k in   5.026915s
               graphql    257.161  (± 4.7%) i/s -      1.296k in   5.051135s
  
  Comparison:
               bluejay:     1680.6 i/s
               graphql:      257.2 i/s - 6.54x  (± 0.00) slower
  
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
               graphql   373.000  i/100ms
               bluejay     5.226k i/100ms
  Calculating -------------------------------------
               graphql      4.186k (±12.7%) i/s -     20.515k in   5.003778s
               bluejay     53.997k (± 4.1%) i/s -    271.752k in   5.041918s
  
  Comparison:
               bluejay:    53996.8 i/s
               graphql:     4185.7 i/s - 12.90x  (± 0.00) slower
  
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
               graphql   282.000  i/100ms
               bluejay     5.359k i/100ms
  Calculating -------------------------------------
               graphql      2.952k (± 5.0%) i/s -     14.946k in   5.075507s
               bluejay     54.661k (± 4.8%) i/s -    273.309k in   5.012009s
  
  Comparison:
               bluejay:    54661.4 i/s
               graphql:     2952.1 i/s - 18.52x  (± 0.00) slower
  
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
               graphql   242.000  i/100ms
               bluejay    14.911k i/100ms
  Calculating -------------------------------------
               graphql      2.476k (± 8.0%) i/s -     12.342k in   5.024615s
               bluejay    153.374k (± 3.3%) i/s -    775.372k in   5.061101s
  
  Comparison:
               bluejay:   153373.7 i/s
               graphql:     2475.5 i/s - 61.96x  (± 0.00) slower
  
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
               graphql   113.000  i/100ms
               bluejay    12.558k i/100ms
  Calculating -------------------------------------
               graphql      1.703k (± 9.7%) i/s -      8.475k in   5.026214s
               bluejay    141.316k (± 7.4%) i/s -    703.248k in   5.010912s
  
  Comparison:
               bluejay:   141315.8 i/s
               graphql:     1703.4 i/s - 82.96x  (± 0.00) slower
  
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
