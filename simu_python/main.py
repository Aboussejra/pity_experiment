import random
import matplotlib.pyplot as plt
from typing import List

def pity_experiment(proba: float, num_rounds_per_simu: int, pity_limit_auto_win: int) -> int:
    num_wins = 0
    pity_counter = 0
    for _ in range(num_rounds_per_simu):
        # AUTO WIN GG
        if pity_counter >= pity_limit_auto_win:
            num_wins += 1
            pity_counter = 0
        else:
            win = random.uniform(0,1) < proba
            if win:
                pity_counter = 0
                num_wins += 1
            else:
                pity_counter += 1
    return num_wins

# Parameters
proba_winning_per_round = 1/20
num_rounds_per_simu = 2000
no_pity_mean_win_theorteical = num_rounds_per_simu * proba_winning_per_round
pity_limit_auto_win = 20
num_simu = 1000

# Run the simulation multiple times and collect total wins per run
total_wins_per_run = []
for _ in range(num_simu):
    num_wins = pity_experiment(proba_winning_per_round, num_rounds_per_simu, pity_limit_auto_win)
    total_wins_per_run.append(num_wins)

# Plot the histogram
plt.figure(figsize=(10, 6))
plt.hist(total_wins_per_run, color='skyblue', edgecolor='black')
plt.title('Distribution of Total Wins in 1,000 Pity Experiments')
plt.xlabel('Total Wins per Run')
plt.ylabel('Frequency')
plt.grid(True, alpha=0.3)
plt.show()
