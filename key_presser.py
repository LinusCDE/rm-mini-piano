#!/usr/bin/env python3

from pynput.keyboard import Key, Controller
from select import select
import socket
from sys import stderr
from time import sleep
from traceback import print_exc

KEYMAP_COMBINATIONS_1 = {
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
}

KEYMAP_COMBINATIONS_2 = {
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
}

KEYMAP_COMBINATIONS_3 = {
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
}

KEYMAP_COMBINATIONS_4 = {
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
}

KEYMAP_COMBINATIONS_5 = {
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
}

# Current
KEYMAP_COMBINATIONS = KEYMAP_COMBINATIONS_3

keyboard = Controller()


def handleKeyName(key_name: str, pressed: bool):
    global KEYMAP_COMBINATIONS
    # Mode check
    if pressed and key_name.startswith('m'):
        if key_name == 'm1':
            KEYMAP_COMBINATIONS = KEYMAP_COMBINATIONS_1
            print('Selected keymap 1')
        elif key_name == 'm2':
            KEYMAP_COMBINATIONS = KEYMAP_COMBINATIONS_2
            print('Selected keymap 2')
        elif key_name == 'm3':
            KEYMAP_COMBINATIONS = KEYMAP_COMBINATIONS_3
            print('Selected keymap 3')
        elif key_name == 'm4':
            KEYMAP_COMBINATIONS = KEYMAP_COMBINATIONS_4
            print('Selected keymap 4')
        elif key_name == 'm5':
            KEYMAP_COMBINATIONS = KEYMAP_COMBINATIONS_5
            print('Selected keymap 5')
        else:
            print('Unknown mode: %s' % key_name, file=stderr)
        return
    elif key_name.startswith('m'):
        return


    if key_name not in KEYMAP_COMBINATIONS:
        print('Unknown key_name (%s)!' % key_name, file=stderr)
        return

    keys = KEYMAP_COMBINATIONS[key_name]

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
