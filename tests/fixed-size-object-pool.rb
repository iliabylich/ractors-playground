require_relative './helper'

POOL_SIZE = 5
objects = 1.upto(POOL_SIZE).map { |i| ["pool-object-#{i}"] }
POOL = CAtomics::FixedSizeObjectPool.new(POOL_SIZE, 1_000) { objects.shift }

ractors = 1.upto(POOL_SIZE).map do |i|
  Ractor.new(i) do |i|
    10.times do |j|
      POOL.with do |v|
        v.push([i, j])
      end
    end

    Ractor.yield :done
  end
end

p ractors.map(&:take)
POOL_SIZE.times do
  p POOL.checkout
end
POOL.with { |obj| }
