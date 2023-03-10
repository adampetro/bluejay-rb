# typed: strict
# frozen_string_literal: true

require "sorbet-runtime"
require "frozen_record"
require "zeitwerk"
require "bluejay"
require "date"

require_relative "graph"

loader = Zeitwerk::Loader.new
loader.push_dir("#{__dir__}/graph", namespace: Graph)
loader.push_dir("#{__dir__}/models")
loader.setup

FrozenRecord::Base.base_path = "#{__dir__}/../data"
