require "benchmark/ips"
require "benchmark/memory"
require "bluejay"
require "graphql"

module Bench
  extend(self)

  def ips(&blk)
    Benchmark.ips do |x|
      blk.call(x)

      x.config(time: 0, warmup: 0) if ENV["CI"]
    end
  end

  def memory(&blk)
    Benchmark.memory do |x|
      blk.call(x)

      x.config(time: 0, warmup: 0) if ENV["CI"]
    end
  end

  def all(&blk)
    ips(&blk)
    memory(&blk)
  end
end
