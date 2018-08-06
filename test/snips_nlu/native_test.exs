defmodule Snips.NLU.NativeTest do
  use ExUnit.Case

  alias Snips.NLU.Native

  test "engine_open, parse" do
    assert {:error, :enoent} == Native.engine_open("not_found")

    assert {:ok, ref} = Native.engine_open("#{__DIR__}/engine.zip")
    assert is_reference(ref)

    assert {:ok, json} = Native.parse(ref, "Turn the lights on in the kitchen")
    assert {:ok, decoded} = Jason.decode(json)

    assert %{
             "input" => "Turn the lights on in the kitchen",
             "intent" => %{
               "intentName" => "sampleTurnOnLight",
               "probability" => 0.81847286
             },
             "slots" => [
               %{
                 "entity" => "room",
                 "range" => %{"end" => 33, "start" => 26},
                 "rawValue" => "kitchen",
                 "slotName" => "room",
                 "value" => %{"kind" => "Custom", "value" => "kitchen"}
               }
             ]
           } == decoded
  end
end
