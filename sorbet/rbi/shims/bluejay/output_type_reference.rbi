# typed: strict

module Bluejay
  class OutputTypeReference
    sig { params(type: BaseOutputTypeReference, required: T::Boolean).void }
    def initialize(type:, required:); end

    class << self
      sig { params(type: OutputTypeReference, required: T::Boolean).returns(OutputTypeReference) }
      def list(type:, required:); end
    end
  end
end
