require_relative './helper'

ITER_COUNT = 100_000
puts "Iterations: #{ITER_COUNT}"

def do_seq
  counter = CAtomics::PlainCounter.new
  (CPU_COUNT * ITER_COUNT).times { counter.increment }
  assert_eq(counter.read, (CPU_COUNT * ITER_COUNT), 'buggy counter')
end

def do_ractors
  counter = CAtomics::PlainCounter.new
  ractors = 1.upto(CPU_COUNT).map do |i|
    Ractor.new(counter) do |counter|
      ITER_COUNT.times { counter.increment }
      Ractor.yield :done
    end
  end
  assert_eq(ractors.map(&:take), [:done] * CPU_COUNT, 'not all workers have finished successfully')
  assert_ne(counter.read, CPU_COUNT * ITER_COUNT, 'race condition')
end

process_args
