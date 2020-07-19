import sys
from pathlib import Path


def actions_to_str(actions):
    result = ""
    for action in actions:
        if action[0] == 0:
            assert(len(action) == 2)
            result += "thrust: {} ".format(action[1])
        elif action[0] == 2:
            result += "beam: {} {} ".format(action[1], action[2:])
        else:
            result += "action: {}".format(action)
    return result


def print_ship_stats(ship):
    assert(len(ship) == 2)
    actions = ship[1]
    ship = ship[0]
    ship_type, C, coords, velocity, [E, F, G, H], D, Dmax, m = ship
    print("{}: at {}, v {}, heat {}/{}, stats {}, other: {} {} {}".format(
        ["x", "o"][ship_type],
        coords,
        velocity,
        D, Dmax,
        [E, F, G, H],
        C,
        m,
        actions_to_str(actions)
    ))


def main():
    project_root = Path(__file__)/'..'/'..'/'..'

    if len(sys.argv) > 1:
        file_path = (project_root/'data'/'replays'/sys.argv[1]).resolve()
        f = open(file_path)
    else:
        f = sys.stdin

    state = eval(f.read())

    turns = state[1][11][1]

    for turn in turns:
        assert(len(turn) == 2)
        turn_number = turn[0]
        print("{}:".format(turn_number))
        assert(len(turn[1]) == 2)
        for ship in turn[1]:
            print_ship_stats(ship)
        print()


if __name__ == '__main__':
    main()