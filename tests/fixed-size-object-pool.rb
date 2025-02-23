require_relative './helper'

class FixedSizeObjectPool
  def with
    obj = pop
    raise 'timeout error' if obj.nil?
    yield obj
  ensure
    unless obj.nil?
      $stderr.puts "Returning #{obj}"
      push(obj)
    end
  end
end

POOL_SIZE = 3
POOL = FixedSizeObjectPool.new(POOL_SIZE, 3_000) { [] }

ractors = 1.upto(CPU_COUNT).map do |i|
  Ractor.new(i) do |i|
    POOL.with do |v|
      v.push(i)
    end

    Ractor.yield :done
  end
end

p ractors.map(&:take)
p POOL_SIZE.times.map { POOL.pop }
