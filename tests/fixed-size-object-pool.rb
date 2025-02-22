require_relative './helper'

class FixedSizeObjectPool
  def with
    obj = pop
    raise 'timeout error' if obj.nil?
    yield obj
  ensure
    push(obj) unless obj.nil?
  end
end

objects = [1, 2, 3]

pool = FixedSizeObjectPool.new(3) { objects.shift }

pool.with do |obj1|
  p obj1
  pool.with do |obj2|
    p obj2
    pool.with do |obj3|
      p obj3

      begin
        pool.with { |obj4| p obj 4 }
      rescue => err
        p err
      end
    end
  end
end

pool.with { |o| p o }
