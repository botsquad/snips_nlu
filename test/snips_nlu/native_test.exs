defmodule Snips.NLU.NativeTest do
  use ExUnit.Case

  alias Snips.NLU.Native

  test "adding works" do
    assert {:ok, 3} == Native.add(1, 2)
  end

  test "engine_open" do
    assert {:error, :enoent} == Native.engine_open("not_found")

    assert is_reference(Native.engine_open("#{__DIR__}/engine.zip"))
  end
end
