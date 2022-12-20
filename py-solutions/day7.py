#!/usr/bin/env python3
import re
import itertools
from copy import deepcopy
import json

def traverse(path, root):
    final_location = root[path[0]]
    for dir in path[1:]:
        final_location = final_location[1][dir]

    return final_location

def build_file_system(input):
    root = {"/": ({}, {})}
    current_path = ["/"]
    current_dir = {}
    for line in input[1:]:
        if line.startswith("$ cd"):
            dir = line.split()[2]
            if dir == "..":
                current_path.pop()
            else:
                traverse(current_path, root)[1][dir] = ({}, {})
                current_path.append(dir)
        elif line.startswith("$ ls"):
            continue
        elif line.startswith("dir "):
            continue
        else:
            file = line.split(" ")
            traverse(current_path, root)[0][file[1]] = int(file[0])
    return root

def calc_sizes(dir, max_size=None, acceptable_sizes=[]):
    size = 0

    files = dir[0]
    dirs = dir[1]
    
    for d in dirs.values():
        new_size, acceptable_sizes = calc_sizes(d, max_size=max_size, acceptable_sizes=acceptable_sizes)
        size += new_size
    
    for file in files.values():
        size += file

    if not max_size or size < max_size:
        acceptable_sizes.append(size)
    
    return size, acceptable_sizes

def part_one(input):
    root = build_file_system(input)

    print(sum(calc_sizes(root['/'], max_size=100000)[1]))

def part_two(input):
    root = build_file_system(input)
    root_size, sizes = calc_sizes(root['/'])

    min_required = 30000000 - (70000000 - root_size)

    print(min(filter(lambda x: (x>min_required), sizes)))

def main():
    puzzleInputFile = open("../files/day7.txt", 'r')
    puzzleInput = puzzleInputFile.read().splitlines()

    part_one(puzzleInput)
    part_two(puzzleInput)

if __name__ == "__main__":
    main()