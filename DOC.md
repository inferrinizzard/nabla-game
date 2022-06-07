# DOC

## Table of Contents

- [Basis](#Basis)
- [Math](#Math)
- [Render](#Render)
- [Events](#Events)
- [Game](#Game)

## Basis

The Basis module contains the basic Basis struct that represents a mathematical expression.
Bases are defined in two types: BasisLeaf and BasisNode.<br>

BasisLeaf has two components: BasisElement and a rational Fraction coefficient.<br>
BasisElement is the atomic unit of the math representation: 0, 1, x, or infinity. <br>
BasisNode is defined by an operator and its component Bases (either BasisLeaf or nested BasisNode) + rational Fraction coefficient.

The following operators are supported:

- Add - can have n operands
- Mult - can have n operands
- Div - has 2 operands, divisor and dividend
- Pow - rational exponent defined as part of operator & 1 operand, base
- E (e^x) - has 1 operand, exponent
- Log (Logarithm) - has 1 operand, base
- Cos (Cosine) - has 1 operand
- Sin (Sine) - has 1 operand
- Acos (Arccosine) - has 1 operand
- Asin (Arcsine) - has 1 operand
- Inv (Inverse) - has 1 operand
- Int (Integral) - has 1 operand

The Basis module also has builder functions and overloaded operators for easier math manipulation and Basis construction.

## Math

The Math engine handles all computations and calculations initiated by the user during their play.
It supports the following Functions:

- Derivative
- Integral
- Inverse
- Logarithm

The Math code also has a basic Fraction implementation for rational coefficients and exponents, avoiding the need for floating points.

## Render

The Render directory is responsible for drawing and animating the main play screen, displaying shapes and text for all of the cards on screen.

For each visible item on the screen, the Render engine draws a visible shape onto the main canvas and a hidden shape with random colour onto the hit canvas in order to allow for hit detection.

On click, the colour of the hit region is looked up from the global mapping to determine which item was clicked.

Additionally, when a player plays a card, the new card drawn from the deck is animated to lerp into the position of the player's hand. This is achieved by extracting the start and end positions for each used card and interpolating and rerendering every frame with requestAnimationFrame.

## Events

The Events module manages all of the event listeners and callbacks that allows the browser to communicate with the compiled WASM binary.

The main event listener that carries most of the game's interactivity is the mouse click event listener that transforms hit regions into card selections, passing the id of the displayed item to the render and game engines.

This id is then passed to the callback to connect the player's selection to the selected target, applying the appropriate function (delegated to the Math module) and updating the game state with the result.

Also, the menu and gameover callbacks are managed by the Events module - interfacing with the HTML elements instead of just the game canvas.

## Game

The Game engine handles the main game logic, player hands, turns, deck, and field.

The main Game state and Render state are stored as global static variables, allowing other components to borrow them from anywhere using an unsafe block.

Within the main state, there are 3 main components: the player's hand, the deck, and the field.

- The deck simply holds the remaining cards in the game, with 75 in total
- The field show the 6 currently active Bases, 3 for each player
- The player hands show the 7 cards available to the player to play on their turn
