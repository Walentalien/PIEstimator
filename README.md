# PIEstimator

## Monte Carlo Simulation to Estimate Pi

This project demonstrates how to use a **Monte Carlo simulation** to estimate the value of π by simulating the process of dropping balls into a box with a hollow cylinder. This method provides a probabilistic approximation of π, inspired by concepts from the **Buffon's Needle problem**.

## Project Overview

The simulation involves randomly dropping balls from a fixed height into a box containing a hollow vertical cylinder. The ratio of balls that land inside the cylinder to the total number of balls dropped is used to approximate the value of π.

### Key Steps:
1. **Simulation Setup**:
   - A square box of side length **L** is defined.
   - A hollow cylinder with radius **R** is placed inside the box.
2. **Dropping Balls**:
   - Balls are dropped from a fixed height above the box, and their landing positions are randomly determined.
3. **Counting and Estimating π**:
   - The number of balls landing inside the cylinder (**N_inside**) is tracked.
   - The total number of balls dropped (**N_total**) is also recorded.
   - The ratio of balls inside the cylinder to the total number of balls is used to estimate π using the formula:
   \[
   \pi \approx \frac{N_{\text{inside}}}{N_{\text{total}}} \times \frac{L^2}{R^2}
   \]

## Requirements

- **Programming Language**: Rust or Python (Depending on your preference)
- **Dependencies**: Random number generator for simulating the random drop of balls.

## How It Works

1. **Box Setup**:
   - A square box of side length **L**.
   - A vertical hollow cylinder with radius **R** placed centrally inside the box.

2. **Dropping Balls**:
   - The balls are dropped randomly within the confines of the box.
   - For each ball, a random point within the box is selected.

3. **Ball Landing Logic**:
   - If the ball lands inside the cylinder, it is counted as a "hit."
   - The total number of hits is compared to the total number of balls dropped to approximate π.

4. **Calculation of π**:
   - The ratio of hits to total balls is used to estimate π based on the formula mentioned above.

## Steps to Run the Simulation

### 1. Clone the Repository
todo
