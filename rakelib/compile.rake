# frozen_string_literal: true

require "rb_sys/extensiontask"

GEMSPEC = Bundler.load_gemspec("bluejay.gemspec")

RbSys::ExtensionTask.new("bluejay-rb", GEMSPEC) do |ext|
  ext.lib_dir = "lib/bluejay"
end
