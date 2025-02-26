require_relative './helper'

ITER_COUNT = 10
puts "Iterations: #{ITER_COUNT}"

Key = Struct.new(:text)
KEYS = Ractor.make_shareable(1.upto(20).map { |n| Key.new("key#{n}") })

def do_seq
  map = CAtomics::ConcurrentHashMap.with_keys(KEYS)
  (CPU_COUNT * ITER_COUNT).times { map.inc_random_value(KEYS) }
  assert_eq(map.sum, CPU_COUNT * ITER_COUNT, 'buggy counter')
end

def do_ractors
    map = CAtomics::ConcurrentHashMap.with_keys(KEYS)
    ractors = 1.upto(CPU_COUNT).map do |i|
      Ractor.new(map) do |map|
          ITER_COUNT.times { map.inc_random_value(KEYS) }
          Ractor.yield :done
      end
    end
    assert_eq(ractors.map(&:take), [:done] * CPU_COUNT, 'not all workers have finished successfully')
    assert_eq(map.sum, CPU_COUNT * ITER_COUNT, 'race condition')
end

process_args
