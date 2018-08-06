defmodule Snips.NLU.NativeTest do
  use ExUnit.Case

  alias Snips.NLU.Native

  test "adding works" do
    assert {:ok, 3} == Native.add(1, 2)
  end
end
