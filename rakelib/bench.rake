# frozen_string_literal: true

namespace :bench do
  all_tasks = [:compile]

  Dir.glob("bench/*.rb").each do |path|
    task_name = File.basename(path, ".rb")
    next if task_name == "bench" # Bench helper

    desc "Run #{path} benchmark"
    task task_name do
      sh "ruby -Ilib #{path}"
      puts
    end

    all_tasks << task_name
  end

  desc "Run all benchmarks"
  task all: all_tasks
end

desc "Run all benchmarks"
task bench: "bench:all"
