# frozen_string_literal: true

require_relative "c_atomics/version"
require_relative "c_atomics/c_atomics"

module CAtomics
  class Error < StandardError; end

  class Undefined
    def inspect
      "#<Undefined>"
    end
  end
  UNDEFINED = Ractor.make_shareable(Undefined.new)

  class FixedSizeObjectPool
    def with
      obj_and_idx = checkout
      if obj_and_idx.nil?
        raise 'timeout error'
      else
        yield obj_and_idx[0]
      end
    ensure
      unless obj_and_idx.nil?
        checkin(obj_and_idx[1])
      end
    end
  end

  class ConcurrentHashMap
    def self.with_keys(known_keys)
      map = new
      known_keys.each { |key| map.set(key, 0) }
      map
    end

    def increment(key)
      fetch_and_modify(key) { |v| v + 1 }
    end

    def sum(known_keys)
      known_keys.map { |k| get(k) }.sum
    end
  end

  class QueueWithMutex
    def pop
      loop do
        value = try_pop(UNDEFINED)
        if value.nil?
          return nil
        elsif value.equal?(UNDEFINED)
          # continue
        else
          return value
        end
        sleep 0.001
      end
    end

    def push(value)
      loop do
        pushed = try_push(value)
        return if pushed
        sleep 0.001
      end
    end
  end

  class LogOnMark
    def initialize(inner)
      @inner = inner
    end

    def method_missing(method_name, *args, **kwargs, &block)
      @inner.__send__(method_name, *args, **kwargs, &block)
    end
  end
end
