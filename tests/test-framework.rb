require_relative './helper'

def now = Process.clock_gettime(Process::CLOCK_MONOTONIC)

class TestCase
  def assert_eq(lhs, rhs, message = 'assertion failed')
    if lhs != rhs
      raise "#{message}: #{lhs} != #{rhs}"
    end
  end

  class << self
    def test_methods
      instance_methods.grep(/\Atest_/)
    end

    def inherited(subclass)
      subclasses << subclass
    end

    def subclasses
      @subclasses ||= []
    end

    def measure
      start = now
      yield
      now - start
    end

    def run(method_name, report)
      instance = new
      time = measure { instance.send(method_name) }
      print "."
      report.passed!(self, method_name, time)
    rescue => err
      print "F"
      report.failed!(self, method_name, err)
    end
  end
end

class Report
  attr_reader :passed, :failed

  def initialize
    @passed = []
    @failed = []
  end

  def passed!(klass, method_name, time)
    @passed << [klass, method_name, time]
  end

  def failed!(klass, method_name, err)
    @failed << [klass, method_name, err]
  end

  def merge!(other)
    @passed += other.passed
    @failed += other.failed
  end

  def print
    puts "Passed: #{passed.count}"
    passed.each do |klass, method_name, time|
      puts "  - #{klass}##{method_name} (in #{time}ms)"
    end
    puts "Failed: #{failed.count}"
    failed.each do |klass, method_name, err|
      puts "  - #{klass}##{method_name}: #{err}"
    end
  end
end

class TestClassOne < TestCase
  1.upto(20) do |i|
    class_eval <<~RUBY
      def test_#{i}
        # heavy_computation(rand(1000) + 2000)
        assert_eq(1, 1)
      end
    RUBY
  end
end

class TestClassTwo < TestCase
  def test_that_fails
    # heavy_computation(rand(1000) + 2000)
    assert_eq 1, 2
  end
end

def heavy_computation(ms)
  finish_at = now + ms / 1000.0
  counter = 0
  while now < finish_at
    1000.times { counter += 1 }
  end
end

QUEUE = SyncQueue.new

workers = 1.upto(CPU_COUNT).map do |i|
  Ractor.new(name: "worker-#{i}") do
    report = Report.new

    while (item = QUEUE.pop) do
      klass, method_name = item
      klass.run(method_name, report)
    end

    Ractor.yield report
  end
end

TestCase.subclasses.each do |klass|
  klass.test_methods.each do |method_name|
    QUEUE.push([klass, method_name])
  end
end
CPU_COUNT.times { QUEUE.push(nil) }

report = Report.new
workers.map(&:take).each do |subreport|
  report.merge!(subreport)
end
puts
report.print
