defmodule Snips.NLU.Native do
  use Rustler, otp_app: :snips_nlu, crate: :snips_nlu_native

  def engine_open(_path), do: error()
  def parse(_engine, _utterance), do: error()

  defp error, do: :erlang.nif_error(:nif_not_loaded)
end
