# typed: ignore
# frozen_string_literal: true

require_relative "bench"

class MyInputObjectType < Bluejay::InputObjectType
  class << self
    extend(T::Sig)

    sig { override.returns(T::Array[Bluejay::InputValueDefinition]) }
    def input_field_definitions
      [
        Bluejay::InputValueDefinition.new(
          name: "myArg",
          type: lit!(it!(Bluejay::Scalar::String)),
          description: "This is my arg",
        ),
        Bluejay::InputValueDefinition.new(name: "myOtherArg", type: it!(Bluejay::Scalar::Int)),
      ]
    end
  end
end

Bench.all do |x|
  input = { "myArg" => ["a", "b", "c"], "myOtherArg" => 12 }
  expected = MyInputObjectType.new(["a", "b", "c"], 12)
  x.report(:bluejay) do
    result = MyInputObjectType.coerce_input(input)
    raise unless result.ok?
    raise unless expected == result.unwrap
  end
end
