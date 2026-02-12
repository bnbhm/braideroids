# Goal

- Fun to code, not tiring
- Fun to play, not jarring

# TODO

- [ ] Levels
  - [ ] Level 0: Ghost ship to navigate levels
- [ ] Braid like time reversal.
- [ ] Drag limit should depend on the size (mass) heavier should rotate slow. [DragClamp]
      Beginning levels will be familiar to earth-brained and less violent. [LevelDifficulty]

- [x] Goal should be to avoid asteroids, so
      Collision between ship and asteroid
- [x] Bullets break asteroid while conserving momentum
- [x] Gradient background (possibly dynamic) DARKGRAY to LIGHTGRAY
- [x] Quit by pressing v
- [x] Asteroids are also rigid bodies!
- [x] Drag only for high speeds, to give the feel of space. [:DragClamp]
- [x] Warping with half triangle renderings
- [x] Turning should have inertia and drag.
- [x] Movement from input
- [x] At high speeds ship should go straight, so [:HighSpeedStraight]
      Ship and Asteroid will have different angular drag, so  
       Ship should have an automatic breaking system [AutoBreak]

# Ideas

- [ ] Multiple players
- [ ] Since it feels tougher when the limits are high. [:LevelDifficulty] [DragClamp]
      Objects will become more sensitive and drag will reduce on higher levels.
- [ ] Release light/radiation when dragging for high speeds [DragClamp]
- [ ] Inner fill of the triangle changes to something opaque as speed increases
- [ ] Exhaust smokes, both for boost and for turning
- [ ] Since there is always cheat to simplify confusion of [LevelDifficulty]
      Higher difficulty levels will be first person, the world will rotate.
- [ ] Drawing vertices looks nice! Maybe in later/earlier levels we could add that.

# Resources

- [x] [Pikuma - Collision with Separating Axis Theorem (SAT)](https://youtu.be/-EsWKT7Doww?si=rIOki83dMiZCQAwm)
- [ ] [javidx9 - Convex Polygon Collision](https://youtu.be/7Ik2vowGcU0?si=3JnGclYGdF5nZlUR)
