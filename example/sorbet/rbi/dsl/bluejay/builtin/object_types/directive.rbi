# typed: true

# DO NOT EDIT MANUALLY
# This is an autogenerated file for dynamic methods in `Bluejay::Builtin::ObjectTypes::Directive`.
# Please instead update this file by running `bin/tapioca dsl Bluejay::Builtin::ObjectTypes::Directive`.

module Bluejay::Builtin::ObjectTypes::Directive::Interface
  abstract!

  sig { abstract.returns(T::Array[Bluejay::Builtin::ObjectTypes::InputValue::Interface]) }
  def args; end

  sig { abstract.returns(T.nilable(String)) }
  def description; end

  sig { abstract.returns(T::Array[String]) }
  def locations; end

  sig { abstract.returns(String) }
  def name; end

  sig { abstract.returns(T::Boolean) }
  def repeatable?; end

  sig(:final) { returns(String) }
  def resolve_typename; end
end
