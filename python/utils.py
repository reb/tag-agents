import enum


# Duplicate the Action from enum from the Rust side for now to make life easier
class Action(enum.Enum):
    Stay = 0
    Up = 1
    Right = 2
    Down = 3
    Left = 4
    Random = 255
