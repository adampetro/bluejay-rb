# typed: ignore
# frozen_string_literal: true

require_relative "bench"

class MyInputType < Bluejay::InputType
  class << self
    extend(T::Sig)

    sig { override.returns(T::Array[Bluejay::InputValueDefinition]) }
    def input_field_definitions
      [
        Bluejay::InputValueDefinition.new(name: "myArg", type: lit!(it!(Bluejay::Scalar::String)), description: "This is my arg"),
        Bluejay::InputValueDefinition.new(name: "myOtherArg", type: it!(Bluejay::Scalar::Int)),
      ]
    end
  end
end

Bench.all do |x|
  input = { "myArg" => ["a", "b", "c"], "myOtherArg" => 12 }
  expected = MyInputType.new(["a", "b", "c"], 12)
  x.report(:bluejay) do
    result = MyInputType.coerce_input(input)
    raise unless result.ok?
    raise unless expected == result.unwrap
  end
end
