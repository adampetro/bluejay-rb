# typed: strict
# frozen_string_literal: true

module Bluejay
  module OutputTypeReferenceShorthands
    extend(T::Sig)

    sig { params(t: BaseOutputTypeReference).returns(OutputTypeReference) }
    def ot(t)
      OutputTypeReference.new(type: t, required: false)
    end

    sig { params(t: BaseOutputTypeReference).returns(OutputTypeReference) }
    def ot!(t)
      OutputTypeReference.new(type: t, required: true)
    end

    sig { params(t: OutputTypeReference).returns(OutputTypeReference) }
    def lot(t)
      OutputTypeReference.list(type: t, required: false)
    end

    sig { params(t: OutputTypeReference).returns(OutputTypeReference) }
    def lot!(t)
      OutputTypeReference.list(type: t, required: true)
    end
  end
end
