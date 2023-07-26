# typed: strict
# frozen_string_literal: true

module Bluejay
  class CustomScalarTypeDefinition
    sig do
      params(
        name: String,
        description: T.nilable(String),
        directives: T::Array[Base::Directive::Instance],
        specified_by_url: T.nilable(String),
        ruby_class: Base::CustomScalarType,
        internal_representation_sorbet_type_name: String,
        input_coercion_method_signature: CoercionMethodSignature,
        result_coercion_method_signature: CoercionMethodSignature,
        visibility: T.nilable(Visibility),
      ).void
    end
    def initialize(name:, description:, directives:, specified_by_url:, ruby_class:,
      internal_representation_sorbet_type_name:, input_coercion_method_signature:, result_coercion_method_signature:, visibility:)
    end

    class CoercionMethodSignature
      Result = T.let(T.unsafe(nil), CoercionMethodSignature)

      class << self
        sig { params(exception_class: T.class_of(StandardError)).returns(T.attached_class) }
        def exception(exception_class); end
      end
    end
  end
end
