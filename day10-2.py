import scipy.optimize as o
import numpy as np


def main():
    total = 0
    while True:
        try:
            line = input().split()
            desired = list(map(int, line[-1].strip("{}").split(",")))
            desired = np.array(desired)

            num_vars = desired.shape[0]
            num_constraints = len(line) - 2
            constraints = np.zeros((num_constraints, num_vars), dtype=int)
            for i in range(1, len(line) - 1):
                buttons = list(map(int, line[i].strip("()").split(",")))
                for button in buttons:
                    constraints[i - 1, button] = 1
            constraints = constraints.transpose()
            c = np.ones((num_constraints,))

            output = o.linprog(c, A_eq=constraints, b_eq=desired, integrality=1).x
            total += output.sum()
        except EOFError:
            break
    print(total)


main()
