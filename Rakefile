# frozen_string_literal: true

require "bundler/gem_tasks"
require "rake/testtask"

test_config = lambda do |t|
  t.libs << "test"
  t.test_files = FileList["test/**/test_*.rb"]
end

Rake::TestTask.new(:test, &test_config)

namespace :mem do
  if RbConfig::CONFIG["host_os"] == "linux"
    begin
      require "ruby_memcheck"

      RubyMemcheck.config(binary_name: "ext")

      RubyMemcheck::TestTask.new(check: "compile:dev", &test_config)
    rescue LoadError
      task(:check) do
        abort('Please add `gem "ruby_memcheck"` to your Gemfile to use the "mem:check" task')
      end
    end
  else
    task(:check) do # rubocop:disable Rake/DuplicateTask
      abort('The "mem:check" task is only available on Linux')
    end
  end
end

require "rubocop/rake_task"

RuboCop::RakeTask.new

task default: [:compile, :test, :rubocop]
