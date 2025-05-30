import random
import streamlit as st
import matplotlib.pyplot as plt
import numpy as np
import scipy.stats as stats

def pity_experiment(proba: float, num_rounds_per_simu: int, pity_limit_auto_win: int) -> int:
    num_wins = 0
    pity_counter = 0
    for _ in range(num_rounds_per_simu):
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

st.title("Pity System Simulation")

proba_winning_per_round = st.number_input("Probability of winning per round", min_value=0.0, max_value=1.0, value=1/20.0, step=0.001, format="%.3f")
num_rounds_per_simu = st.number_input("Number of rounds per simulation", min_value=1, max_value=10000, value=2000, step=1)
pity_limit_auto_win = st.number_input("Pity limit (auto win after this many losses)", min_value=1, max_value=100, value=20, step=1)
num_simu = st.number_input("Number of simulations", min_value=1, max_value=10000, value=1000, step=1)

total_wins_per_run = []
for _ in range(int(num_simu)):
    num_wins = pity_experiment(proba_winning_per_round, int(num_rounds_per_simu), int(pity_limit_auto_win))
    total_wins_per_run.append(num_wins)

# No pity distrib:

mean_no_pity = num_rounds_per_simu * proba_winning_per_round
std_no_pity = (num_rounds_per_simu * proba_winning_per_round * (1 - proba_winning_per_round)) ** 0.5
x = np.linspace(mean_no_pity - 3 * std_no_pity, mean_no_pity + 3 * std_no_pity, 100)
distribution_no_pity = stats.norm.pdf(x, mean_no_pity, std_no_pity)

# Fitting normal distrib to data:

mean_fit_pity, std_fit_pity = stats.norm.fit(total_wins_per_run)
x_fit = np.linspace(min(total_wins_per_run), max(total_wins_per_run), 100)
distribution_fit_pity = stats.norm.pdf(x_fit, mean_fit_pity, std_fit_pity)

fig, ax = plt.subplots(figsize=(10, 6))
ax.hist(total_wins_per_run, color='skyblue', edgecolor='black', alpha=0.7, bins=30, density=True, label='Pity Distribution')
ax.plot(x_fit, distribution_fit_pity, 'g--', linewidth=2, label='Normal Fit Pity distrib (from data)')

ax.plot(x, distribution_no_pity, color='red', linewidth=2, label='no_pity_distrib win')
ax.set_title(f'Distribution of Total Wins in {num_simu:,} Pity Experiments')
ax.set_xlabel('Total Wins per Run')
ax.set_ylabel('Density')
ax.grid(True, alpha=0.3)
ax.legend()
st.pyplot(fig)