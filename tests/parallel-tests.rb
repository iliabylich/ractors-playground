require_relative './microtest'

class TestClassOne < Microtest::TestCase
  1.upto(20) do |i|
    class_eval <<~RUBY
      def test_#{i}
        heavy_computation(2000)
        assert_eq(1, 1)
      end
    RUBY
  end
end

class TestClassTwo < Microtest::TestCase
  def test_that_fails
    heavy_computation(2000)
    assert_eq 1, 2
  end
end

Microtest.run!
