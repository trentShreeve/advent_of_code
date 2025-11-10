#!/usr/bin/python
from itertools import combinations

# dictionary of found antenna
def found_antenna(lines) -> dict:
    antenna = {}
    for y, row in enumerate(lines):
        for x, char in enumerate(row):
            if char != '.':
                if char not in antenna:
                    antenna[char] = []
                antenna[char].append((y, x))
    return antenna

def check_antinode(antenna, grid) -> set:
    max_y = len(grid)
    max_x = len(grid[0])

    antinodes = set()

    for char, positions in antenna.items():
        for p1, p2 in combinations(positions, 2):
            y1, x1 = p1
            y2, x2 = p2

            dy = y2 - y1    
            dx = x2 - x1

            # forward antinode
            f_y = y2 + dy
            f_x = x2 + dx
            
            # backward antinode
            b_y = y1 - dy
            b_x = x1 - dx
            
            if 0 <= f_x < max_x and 0 <= f_y < max_y:
                antinodes.add((f_y, f_x))           

            if 0 <= b_x < max_x and 0 <= b_y < max_y:
                antinodes.add((b_y, b_x))

    return antinodes

def check_antinode_resonance(antenna, grid) -> set:
    max_y = len(grid)
    max_x = len(grid[0])

    antinodes = set()

    for char, positions in antenna.items():
        for p1, p2 in combinations(positions, 2):
            y1, x1 = p1
            y2, x2 = p2

            dy = y2 - y1    
            dx = x2 - x1
            
            # extend backwards from (x1,y1)
            x,y = x1, y1
            while 0 <= x < max_x and 0 <= y < max_y:
                antinodes.add((x,y))
                x -= dx
                y -= dy

            # extend forwards from (x2,y2)
            x,y = x2, y2
            while 0 <= x < max_x and 0 <= y < max_y:
                antinodes.add((x,y))
                x += dx
                y += dy

    return antinodes

def print_grid_with_antinodes(grid, antinodes, marker='#'):
    for y, row in enumerate(grid):
        row_str = ""
        for x, char in enumerate(row):
            if (y, x) in antinodes:
                row_str += marker
            else:
                row_str += char
        print(row_str)

if __name__ == '__main__':
    with open('puzzle.txt', 'r') as file:
        lines = [list(line.strip()) for line in file]
    
    antennas = found_antenna(lines)
 
    print(f"total antinodes: {len(check_antinode(antennas, lines))}")

    print(f"total antinode resonance: {len(check_antinode_resonance(antennas, lines))}")
        