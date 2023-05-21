# frozen_string_literal: true

require_relative "lib/bluejay/version"

Gem::Specification.new do |spec|
  spec.name = "bluejay"
  spec.version = Bluejay::VERSION
  spec.authors = ["Adam Petro"]
  spec.email = ["adamapetro@gmail.com"]

  spec.summary = "A fast GraphQL engine."
  spec.description = "A fast GraphQL engine."
  spec.homepage = "https://github.com/adampetro/bluejay-rb"
  spec.required_ruby_version = ">= 3.2.0"
  spec.license = "MIT"

  spec.metadata["homepage_uri"] = spec.homepage
  spec.metadata["source_code_uri"] = "https://github.com/adampetro/bluejay-rb"
  spec.metadata["changelog_uri"] = "https://github.com/adampetro/bluejay-rb/blob/main/CHANGELOG"
  spec.metadata["cargo_crate_name"] = "ext"

  spec.files = Dir["{lib,ext}/**/*", "LICENSE", "README.md", "Cargo.*", "rust-toolchain.toml"]
  spec.files.reject! { |f| File.directory?(f) || File.extname(f) == ".bundle" }
  spec.bindir = "exe"
  spec.executables = spec.files.grep(%r{\Aexe/}) { |f| File.basename(f) }
  spec.require_paths = ["lib"]
  spec.extensions = ["ext/extconf.rb"]

  spec.add_dependency("sorbet-runtime")
end
