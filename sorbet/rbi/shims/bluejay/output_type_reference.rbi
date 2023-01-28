# typed: strict

module Bluejay
  class OutputTypeReference
    sig { params(type: BaseOutputTypeReference, required: T::Boolean).void }
    def initialize(type:, required:); end

    class << self
      sig { params(type: OutputTypeReference, required: T::Boolean).returns(OutputTypeReference) }
      def list(type:, required:); end
    end

    sig { returns(T::Boolean) }
    def list?; end

    sig { returns(T::Boolean) }
    def base?; end

    sig { returns(T::Boolean) }
    def required?; end

    sig { returns(String) }
    def sorbet_type; end

    sig { returns(T.self_type) }
    def unwrap_list; end
  end
end
