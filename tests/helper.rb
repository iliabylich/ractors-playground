require 'benchmark'
require 'bundler/setup'
require 'c_atomics'
require 'stringio'

if ENV['GC_STRESS']
  GC.stress = true
end

def disable_stderr
  stderr, $stderr = $stderr, StringIO.new
  yield
ensure
  $stderr = stderr
end

Warning[:experimental] = false

CPU_COUNT = `cat /proc/cpuinfo | grep processor | wc -l`.to_i
puts "CPU count: #{CPU_COUNT}"

def assert_eq(lhs, rhs, message)
  raise "#{message}: #{lhs} != #{rhs}" if lhs != rhs
end

def assert_ne(lhs, rhs, message)
  raise "#{message}: #{lhs} == #{rhs}" if lhs == rhs
end

def do_benchmark
  Benchmark.bmbm do |x|
    x.report("#{CPU_COUNT}x seq") { do_seq }
    x.report("#{CPU_COUNT}x ractors") { do_ractors }
  end
end

def process_args
  case ARGV.first
  when 'seq' then do_seq
  when 'ractors' then do_ractors
  when 'benchmark' then do_benchmark
  else
    warn <<~USAGE
      Usage: #{$0} mode

      Options:
        mode: seq / ractors / benchmark
    USAGE
    exit 1
  end
end

def now = Process.clock_gettime(Process::CLOCK_MONOTONIC)

def heavy_computation(ms)
  finish_at = now + ms / 1000.0
  counter = 0
  while now < finish_at
    1000.times { counter += 1 }
  end
end
