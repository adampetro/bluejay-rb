# typed: strict
# frozen_string_literal: true

module Bluejay
  module OutputTypeShorthands
    extend(T::Sig)

    sig { params(t: BaseOutputType).returns(OutputType) }
    def ot(t)
      OutputType.new(type: t, required: false)
    end

    sig { params(t: BaseOutputType).returns(OutputType) }
    def ot!(t)
      OutputType.new(type: t, required: true)
    end

    sig { params(t: OutputType).returns(OutputType) }
    def lot(t)
      OutputType.list(type: t, required: false)
    end

    sig { params(t: OutputType).returns(OutputType) }
    def lot!(t)
      OutputType.list(type: t, required: true)
    end
  end
end
