require 'benchmark'
require 'bundler/setup'
require 'atomic_counter'

# trigger warnings once at the beginning to not pollute benchmark's output
Ractor.new {}

def assert_eq(lhs, rhs, message)
    raise "#{message}: #{lhs} != #{rhs}" if lhs != rhs
end

CNT_SEQ = Ractor.make_shareable(AtomicCounter.new)
def do_seq
    CNT_SEQ.write(0)
    8_000_000.times { CNT_SEQ.increment }
    assert_eq(CNT_SEQ.read, 8_000_000, 'buggy counter')
end

CNT_PAR = Ractor.make_shareable(AtomicCounter.new)
def do_par
    CNT_PAR.write(0)
    ractors = 1.upto(8).map do |i|
        Ractor.new do
            1_000_000.times { CNT_PAR.increment }
            Ractor.yield :done
        end
    end
    assert_eq(ractors.map(&:take), [:done] * 8, 'not all workers have finished successfully')
    assert_eq(CNT_PAR.read, 8_000_000, 'race condition')
end

def do_benchmark
    Benchmark.bmbm do |x|
        x.report("8x seq") { do_seq }
        x.report("8x par") { do_par }
    end
end

case ARGV.first
when 'seq' then do_seq
when 'par' then do_par
when 'benchmark' then do_benchmark
else
    warn "arg can be seq, par, benchmark"
end
