# typed: strict
# frozen_string_literal: true

module Bluejay
  module InputTypeShorthands
    extend(T::Sig)

    sig { params(t: BaseInputType).returns(InputType) }
    def it(t)
      InputType.new(type: t, required: false)
    end

    sig { params(t: BaseInputType).returns(InputType) }
    def it!(t)
      InputType.new(type: t, required: true)
    end

    sig { params(t: InputType).returns(InputType) }
    def lit(t)
      InputType.list(type: t, required: false)
    end

    sig { params(t: InputType).returns(InputType) }
    def lit!(t)
      InputType.list(type: t, required: true)
    end
  end
end
