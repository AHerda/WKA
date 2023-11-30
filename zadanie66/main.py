import numpy as np
import random
import matplotlib.pyplot as plt

def fisher_yates_shuffle(arr):
    n = len(arr)
    for i in range(n - 1, 0, -1):
        j = random.randint(0, i)
        arr[i], arr[j] = arr[j], arr[i]

def count_fixed_points(perm):
    return sum(1 for i, v in enumerate(perm) if i == v)

def experimental_study(max_n, num_trials):
    arr = []
    for n in range(1, max_n + 1):
        total_fixed_points = 0
        for _ in range(num_trials):
            permutation = list(range(n))
            fisher_yates_shuffle(permutation)
            total_fixed_points += count_fixed_points(permutation)
        average_fixed_points = total_fixed_points / num_trials
        print(f"Avg fixed points for n = {n}:\t{average_fixed_points}")
        arr.append(average_fixed_points)
    
    return arr

# Ustawienia eksperymentu
max_n = 100
num_trials = 1000

# Przeprowadź eksperyment
x = experimental_study(max_n, num_trials)

plt.plot(range(max_n), x)
plt.show()