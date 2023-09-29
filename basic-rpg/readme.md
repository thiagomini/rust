# Basic RPG Exercises

This package aims to implement some basic rpg features. Here's the list of requirements:

## 1. Calculate Damage

Write a Rust program that calculates the damage dealt by a player to an enemy. The program should:

    Take the player's attack power and the enemy's defense as input.
    Calculate the damage using the formula: damage = attack - defense (minimum damage is 1).
    Print the damage dealt.

## 2. Experience Points (XP) Calculator

Create a program that calculates the experience points (XP) a player earns after defeating an enemy. The program should:

    Take the enemy's level and the player's level as input.
    Calculate XP using the formula: XP = (enemy_level + 10) / (player_level + 1) * 20.
    Print the XP earned, rounded down to the nearest integer.

## 3. Health Potion Effectiveness

Write a Rust program that calculates the health restored by a health potion. The program should:

    Take the player's maximum health and current health as input.
    Calculate the health restored using the formula: restored_health = (max_health - current_health) * 0.25.
    Print the health restored, rounded up to the nearest integer.

## 4. Level Up Calculator

Develop a program that determines if a player levels up after earning XP. The program should:

    Take the player's current XP and the XP earned from defeating an enemy as input.
    Calculate the new total XP.
    Determine if the player levels up (e.g., every 100 XP).
    Print the player's new level and remaining XP.

5. Loot Drop Simulator

Create a Rust program that simulates the dropping of loot from an enemy. The program should:

    Have a list of possible loot items with different drop probabilities.
    Randomly select and print a loot item based on its drop probability.

Instructions for Each Exercise:

    Prompt the user for input values.
    Perform the necessary calculations.
    Print the result to the console.