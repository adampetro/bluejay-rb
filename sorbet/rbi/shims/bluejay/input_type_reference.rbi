# typed: strict

module Bluejay
  class InputTypeReference
    sig { params(type: BaseInputTypeReference, required: T::Boolean).void }
    def initialize(type:, required:); end

    class << self
      sig { params(type: InputTypeReference, required: T::Boolean).returns(InputTypeReference) }
      def list(type:, required:); end
    end
  end
end
