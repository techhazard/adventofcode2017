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
        IO.puts head
        IO.puts tail
        if Enum.member?(tail,head) do
          true
        else
            find_matches(tail)
        end
      end
  end


  @spec find_matches_for_line(String) :: Bool
  def find_matches_for_line(line) do
    a = find_matches(String.split(line, " ", [trim: true]))
    IO.puts "line:"
    IO.puts a
    a
  end

end

lines = Main.open_file
valid_lines = Enum.reject(lines, fn(x) -> Main.find_matches_for_line(x) end)
IO.puts length(Enum.filter(valid_lines, fn(x) -> x != "" end))
