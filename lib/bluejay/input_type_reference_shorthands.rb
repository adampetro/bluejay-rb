# typed: strict
# frozen_string_literal: true

module Bluejay
  module InputTypeReferenceShorthands
    extend(T::Sig)

    sig { params(t: BaseInputTypeReference).returns(InputTypeReference) }
    def it(t)
      InputTypeReference.new(type: t, required: false)
    end

    sig { params(t: BaseInputTypeReference).returns(InputTypeReference) }
    def it!(t)
      InputTypeReference.new(type: t, required: true)
    end

    sig { params(t: InputTypeReference).returns(InputTypeReference) }
    def lit(t)
      InputTypeReference.list(type: t, required: false)
    end

    sig { params(t: InputTypeReference).returns(InputTypeReference) }
    def lit!(t)
      InputTypeReference.list(type: t, required: true)
    end
  end
end
