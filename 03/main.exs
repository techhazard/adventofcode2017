IO.puts "Hello world from Elixir"


defmodule Advent do
  def min(a, b) do
    cond do
      a < b ->
        IO.puts "smaller"
        a
      a > b ->
        IO.puts "larger"
        b
      a == b ->
        IO.puts "same"
        a
    end
  end

  def create_grid(maxnum) do
    maximum = maxnum + 1;
    empty_grid = [[]];
    grid = create_grid(empty_grid, maxnum)
  end

  def create_grid(grid, lastnum, maxnum) do
    case lastnum do
      0 ->
        create_grid([[1]], 1, maxnum)
      maxnum ->
        grid
end


IO.puts Advent.min(1,2)
IO.puts Advent.min(4,2)
IO.puts Advent.min(2,2)
IO.puts Advent.create_grid(277678)
