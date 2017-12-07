"""
N.B. square refers to the square of the number, not to a shape
"""
def squares():
    index = 0
    number = 1
    while True:
        yield (index, number, number**2)
        index += 1
        number += 2

def get_manhattan_distance(target):
    for index, number, square in squares():
        if square > target:
            break

    print(index, number, square)


    horizontal_distance_to_origin_of_square = index
    vertical_distance_to_origin_of_square = index

    manhattan_distance_of_square = horizontal_distance_to_origin_of_square + vertical_distance_to_origin_of_square

    coordinates_of_square = {'x': horizontal_distance_to_origin_of_square, 'y': vertical_distance_to_origin_of_square}

    if square == target:
        return manhattan_distance_of_square

    else:
        if (square-number) < target < square:
            # target is in the same row as the square, left of it
            print("bottom")
            target_distance_to_square = square - target
            coordinates_of_target = {
                'x': coordinates_of_square['x'] - target_distance_to_square,
                'y': coordinates_of_square['y']
            }

            manhattan_distance_of_target = coordinates_of_target['x'] + coordinates_of_target['y']

        elif (square-(number*2)) < target < (square - number):
            # target is on the left side of the square
            print("left")
        elif (square-(number*3)) < target < (square - (number*2)):
            # target is on the top edge
            print("top")
        elif (square-(number*4)) < target < (square - (number*3)):
            # target is on the right edge
            print("right")
        else:
            assert False, "Unreachable"



        return manhattan_distance_of_target



if __name__ == "__main__":
    dist = get_manhattan_distance(277678)
    print(dist)
