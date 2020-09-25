#!/usr/bin/env python3

from pynput.keyboard import Key, Controller
from sys import stderr

KEYMAP_COMBINATIONS = list()

KEYMAP_COMBINATIONS.append({
    'w1': ['1'],
    'b1': [Key.shift, '1'],
    'w2': ['2'],
    'b2': [Key.shift, '2'],
    'w3': ['3'],
    'w4': ['4'],
    'b3': [Key.shift, '4'],
    'w5': ['5'],
    'b4': [Key.shift, '5'],
    'w6': ['6'],
    'b5': [Key.shift, '6'],
    'w7': ['7'],
    'w8': ['8'],
})

KEYMAP_COMBINATIONS.append({
    'w1': ['8'],
    'b1': [Key.shift, '8'],
    'w2': ['9'],
    'b2': [Key.shift, '9'],
    'w3': ['0'],
    'w4': ['q'],
    'b3': [Key.shift, 'q'],
    'w5': ['w'],
    'b4': [Key.shift, 'w'],
    'w6': ['e'],
    'b5': [Key.shift, 'e'],
    'w7': ['r'],
    'w8': ['t'],
})

KEYMAP_COMBINATIONS.append({
    'w1': ['t'],
    'b1': [Key.shift, 't'],
    'w2': ['y'],
    'b2': [Key.shift, 'y'],
    'w3': ['u'],
    'w4': ['i'],
    'b3': [Key.shift, 'i'],
    'w5': ['o'],
    'b4': [Key.shift, 'o'],
    'w6': ['p'],
    'b5': [Key.shift, 'p'],
    'w7': ['a'],
    'w8': ['s'],
})

KEYMAP_COMBINATIONS.append({
    'w1': ['s'],
    'b1': [Key.shift, 's'],
    'w2': ['d'],
    'b2': [Key.shift, 'd'],
    'w3': ['f'],
    'w4': ['g'],
    'b3': [Key.shift, 'g'],
    'w5': ['h'],
    'b4': [Key.shift, 'h'],
    'w6': ['j'],
    'b5': [Key.shift, 'j'],
    'w7': ['k'],
    'w8': ['l'],
})

KEYMAP_COMBINATIONS.append({
    'w1': ['l'],
    'b1': [Key.shift, 'l'],
    'w2': ['z'],
    'b2': [Key.shift, 'z'],
    'w3': ['x'],
    'w4': ['c'],
    'b3': [Key.shift, 'c'],
    'w5': ['v'],
    'b4': [Key.shift, 'v'],
    'w6': ['b'],
    'b5': [Key.shift, 'b'],
    'w7': ['n'],
    'w8': ['m'],
})

# Current
KEYMAP_COMBINATION_INDICES = {2}
KEYMAP_COMBINATION_OVERWRITE_NEXT = True

keyboard = Controller()


def handleKeyName(key_name: str, pressed: bool):
    global KEYMAP_COMBINATION_INDICES
    global KEYMAP_COMBINATION_OVERWRITE_NEXT
    # Mode check
    if key_name.startswith('m'):
        index = int(key_name[1:]) - 1
        if pressed:
            if KEYMAP_COMBINATION_OVERWRITE_NEXT:
                KEYMAP_COMBINATION_INDICES = {index}
                KEYMAP_COMBINATION_OVERWRITE_NEXT = False
            else:
                KEYMAP_COMBINATION_INDICES.add(index)
        elif len(KEYMAP_COMBINATION_INDICES) > 1:
            KEYMAP_COMBINATION_INDICES.remove(index)
        elif len(KEYMAP_COMBINATION_INDICES) == 1:
            KEYMAP_COMBINATION_OVERWRITE_NEXT = True
        return


    for index in KEYMAP_COMBINATION_INDICES:
        if index >= len(KEYMAP_COMBINATIONS):
            print('Mode %d is not supported in this script' % (index+1), file=stderr)
            continue

        if key_name not in KEYMAP_COMBINATIONS[index]:
            print('Unknown key_name (%s)!' % key_name, file=stderr)
            continue

        keys = KEYMAP_COMBINATIONS[index][key_name]

        if pressed:
            for key in keys:
                keyboard.press(key)
        else:
            for key in keys[::-1]:
                keyboard.release(key)

while True:
    line = input()
    if line.startswith('PRESS'):
      handleKeyName(line.split()[1], True)
    elif line.startswith('RELEASE'):
      handleKeyName(line.split()[1], False)
    else:
      print('Invalid input', file=stderr)
