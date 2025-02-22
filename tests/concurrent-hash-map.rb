require_relative './helper'

ITER_COUNT = 100_000
puts "Iterations: #{ITER_COUNT}"

Key = Struct.new(:text)
KEYS = Ractor.make_shareable(1.upto(20).map { |n| Key.new("key#{n}") })

class ConcurrentHashMap
  def self.with_keys
    map = new
    KEYS.each { |key| map.set(key, 0) }
    map
  end

  def inc_random_value = fetch_and_modify(KEYS.sample) { |v| v + 1 }
  def sum = KEYS.map { |k| get(k) }.sum
end

def do_seq
  map = ConcurrentHashMap.with_keys
  (CPU_COUNT * ITER_COUNT).times { map.inc_random_value }
  assert_eq(map.sum, CPU_COUNT * ITER_COUNT, 'buggy counter')
end

def do_ractors
    map = ConcurrentHashMap.with_keys
    ractors = 1.upto(CPU_COUNT).map do |i|
      Ractor.new(map) do |map|
          ITER_COUNT.times { map.inc_random_value }
          Ractor.yield :done
      end
    end
    assert_eq(ractors.map(&:take), [:done] * CPU_COUNT, 'not all workers have finished successfully')
    assert_eq(map.sum, CPU_COUNT * ITER_COUNT, 'race condition')
end

process_args
