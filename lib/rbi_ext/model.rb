# typed: strict
# frozen_string_literal: true

module RBI
  class Tree
    extend T::Sig

    # originally from:
    # https://github.com/Shopify/tapioca/blob/7ea79ff61ac418ee2f5f573eec8a1f005db13ebd/lib/tapioca/rbi_ext/model.rb#L79-L106
    sig do
      params(
        name: String,
        return_type: T.nilable(String),
        parameters: T::Array[TypedParam],
        class_method: T::Boolean,
        visibility: RBI::Visibility,
        comments: T::Array[RBI::Comment],
        is_final: T::Boolean,
        is_abstract: T::Boolean,
      ).void
    end
    def custom_create_method(name, return_type:, parameters: [], class_method: false, visibility: RBI::Public.new,
      comments: [], is_final: false, is_abstract: false)
      return unless Tapioca::RBIHelper.valid_method_name?(name)

      sig = RBI::Sig.new(return_type:, is_final:, is_abstract:)
      method = RBI::Method.new(
        name,
        sigs: [sig],
        is_singleton: class_method,
        visibility:,
        comments:,
      )
      parameters.each do |param|
        method << param.param
        sig << RBI::SigParam.new(param.param.name, param.type)
      end
      self << method
    end

    sig { void }
    def mark_abstract
      add_helper("abstract")
    end

    sig { void }
    def mark_interface
      add_helper("interface")
    end

    sig { params(name: String).void }
    def create_include(name)
      self << RBI::Include.new(name)
    end

    private

    sig { params(name: String).void }
    def add_helper(name)
      helper = RBI::Helper.new(name)
      self << helper
    end
  end
end
