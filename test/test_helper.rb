# frozen_string_literal: true

$LOAD_PATH.unshift(File.expand_path("../lib", __dir__))
require "bluejay"
require "pry"
require "pry-nav"
require "date"
require "json"
require "minitest/autorun"

module GCStressPlugin
  def setup
    super
    if ENV["GC_STRESS"]
      GC.stress = true
    end
  end

  def teardown
    if ENV["GC_STRESS"]
      GC.stress = false
    end
    super
  end
end

module Minitest
  class Test
    include(GCStressPlugin)
  end
end
