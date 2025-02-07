require 'benchmark'
require 'bundler/setup'
require 'c_atomics'

# trigger warnings early to not pollute benchmark's output
Ractor.new {}

CPU_COUNT = `cat /proc/cpuinfo | grep processor | wc -l`.to_i
puts "CPU count: #{CPU_COUNT}"
ITER_COUNT = 1_000_000
puts "Iterations: #{ITER_COUNT}"

def assert_eq(lhs, rhs, message)
    raise "#{message}: #{lhs} != #{rhs}" if lhs != rhs
end

def do_seq
    cnt = AtomicCounter.new
    (CPU_COUNT * ITER_COUNT).times { cnt.increment }
    assert_eq(cnt.read, (CPU_COUNT * ITER_COUNT), 'buggy counter')
end

def do_ractors
    cnt = AtomicCounter.new
    ractors = 1.upto(CPU_COUNT).map do |i|
        Ractor.new(cnt) do |cnt|
            ITER_COUNT.times { cnt.increment }
            Ractor.yield :done
        end
    end
    assert_eq(ractors.map(&:take), [:done] * CPU_COUNT, 'not all workers have finished successfully')
    assert_eq(cnt.read, CPU_COUNT * ITER_COUNT, 'race condition')
end

def do_benchmark
    Benchmark.bmbm do |x|
        x.report("#{CPU_COUNT}x seq") { do_seq }
        x.report("#{CPU_COUNT}x ractors") { do_ractors }
    end
end

case ARGV.first
when 'seq' then do_seq
when 'ractors' then do_ractors
when 'benchmark' then do_benchmark
else
    warn "arg can be one of: seq / ractors / benchmark"
end
