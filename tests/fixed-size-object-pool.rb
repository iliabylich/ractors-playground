require_relative './helper'

objects = [["obj-1"], ["obj-2"], ["obj-3"]]
POOL_SIZE = 3
POOL = CAtomics::FixedSizeObjectPool.new(POOL_SIZE, 5_000) { objects.shift }

ractors = 1.upto(6).map do |i|
  Ractor.new(i) do |i|
    30.times do |j|
      POOL.with do |v|
        v.push([i, j])
      end
    end

    Ractor.yield :done
  end
end

p ractors.map(&:take)
p POOL_SIZE.times.map { POOL.pop }
