defmodule Main do

  @spec open_file() :: list(String.t)
  def open_file() do
    text = File.read! "input.txt"
    String.split(text, "\n", [])
  end

  @spec find_matches(list(String.t)) :: Bool
  def find_matches(words) do

    case length(words) do
      0  ->
        false
      x ->
        [ head | tail ] = words
        Enum.member?(tail, head) || find_matches(tail)
      end
  end


  @spec find_matches_for_line(String) :: Bool
  def find_matches_for_line(line) do
    words = String.split(line, " ", [trim: true])
    sorted_words = Enum.map(words, fn(x) -> String.to_charlist(x) |> Enum.sort |> List.to_string end)
    find_matches(sorted_words)
  end

end

lines = Main.open_file
valid_lines = Enum.reject(lines, fn(x) -> Main.find_matches_for_line(x) end)
IO.puts length(Enum.filter(valid_lines, fn(x) -> x != "" end))
