# Nabla Operator Game ナブラ演算子ゲーム

This project is a web version of the physical card game created by UTokyo students here: https://nablagame.com/.

The live version of the game is available here: https://nabla-game.netlify.app.

The source code is written is Rust + WASM, bootstrapped from here: https://github.com/rustwasm/rust-webpack-template.

Additionally, the math engine implements a custom Computer Algebra System (CAS) to calculate arbitrary Derivatives and Integrals, in additional to other algebraic functions such as Mult, Div, Sqrt, Log, etc.

### Future Plans

- Adding a Japanese language mode
- Adding WebAudio for browser sounds
- Improving responsiveness, currently only mostly supports landscape browsers
- Using WebGL + custom models for the game scene
- Polishing the Menu UI
- Fleshing out the tutorial section
- Eventually improving the custom CAS
- Adding an min-max AI that the user can play against

### Known Issues / Incomplete:

- Integration
  - Integrals don't have full support yet for Complexe Coefficients (log(n), n^(a/b), e^n)
  - Log, Limit operators don't have full support for Integrals
- Inverses
  - Inverses don't support complex integrals
  - Limit operators don't have full support for complex Inverse functions
- Distributive Property (FOIL) is not fully implemented for polynomial x polynomial calculations

### References

- Nabla Operator Game: https://nablagame.com/
- Play Guide: https://www.youtube.com/watch?v=kf0DAygsXAU
- English Rules: https://trans-nabla--itter2voxrtiyag.repl.co/
