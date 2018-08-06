defmodule SnipsNluTest do
  use ExUnit.Case
  doctest SnipsNlu

  test "greets the world" do
    assert SnipsNlu.hello() == :world
  end
end
