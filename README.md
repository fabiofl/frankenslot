# FrankenSlot 🧟‍♂️

My "Frankenstein's monster" version of Slot, stitched together with body parts
from other forks and a few ideas of my own. My goal is to curate features and
tweak the UX/UI, visuals, and how those features work so Slot looks and feels
just right (to me, at least).

- Sometimes that means bringing over a badass feature.
- Sometimes it means overthinking what a button does.
- Eventually it means mixing in design language from other minimal UIs/frontends. Who knows!? (Foreshadowing? 🌚)

## Workflow

I loved seeing people make this awesome project their own, so I look through
other forks for features I'd enjoy using and small details that get me thinking
about what else I could give a shot.

1. Pick what I want to bring over.
2. Learn how it works.
3. Put my own twist on it if I feel it needs something.
4. Merge it through structured PRs linking back to the original work.

## With "body parts" sourced from:

- [Unslot](https://github.com/cloudwithax/unslot) by [clxud](https://github.com/cloudwithax) ([PRs](https://github.com/fabiofl/frankenslot/issues?q=label%3Afrom%3A%40cloudwithax%2Funslot))


### A few disclaimers:

1. I know just enough Rust and OpenGL/GLES to get by. I see this as a cool side project to learn more.
2. I'll use AI assistance to scale what I know. While I'll try my best not to let this turn into _FrankenSlop_ (ha, ha! 🫠), I CAN'T always guarantee grass-fed, organic, "production grade" code.
3. That said, if you want to use stuff from this fork, feel free to. You should be totally fine. (But do it at your own risk.)

> Thank you, [Brandon](https://github.com/BrandonKowalski) and [Markus](https://github.com/MarkusMaribu), for creating and shaping
Slot, as well as everyone in the community building they versions in the open. ✌🏻



---
## Slot README - [Upstream repository](https://github.com/slot-cfw/slot)


<div align="center">

<img align="top" src=".github/resources/hero.webp" width="100%" alt="slot. A bespoke, Game Boy-centric frontend for the Anbernic RG SP."><br><a href="https://github.com/slot-cfw/slot/releases/latest"><img align="top" src="https://badges.slot-cfw.fyi/release.svg" width="20.7142%" alt="Download"></a><a href="https://slot-cfw.fyi"><img align="top" src=".github/resources/badges/guide.svg" width="19.5238%" alt="Guide"></a><a href="https://discord.gg/NYRWsSuPUu"><img align="top" src=".github/resources/badges/discord.svg" width="19.5238%" alt="Discord"></a><a href="https://github.com/slot-cfw/slot/stargazers"><img align="top" src="https://badges.slot-cfw.fyi/stars.svg" width="19.5238%" alt="Stars"></a><a href="https://github.com/slot-cfw/slot/releases"><img align="top" src="https://badges.slot-cfw.fyi/downloads.svg" width="20.7142%" alt="Downloads"></a>

</div>

---

# What is slot?

A bespoke, Game Boy-centric frontend for the Anbernic RG SP.

Has support for GBA, GBC, and GB titles only.

---

# What does it look like?

<p align="center">
<img src=".github/resources/screenshots/carousel.gif" width="48%" alt="Scrolling the cart carousel across the Game Boy Advance, Game Boy and Game Boy Color shelves">
<img src=".github/resources/screenshots/core-picker.gif" width="48%" alt="Flipping a cart to its circuit board to choose mGBA or gpSP">
</p>

<p align="center">
<img src=".github/resources/screenshots/link.gif" width="96.5%" alt="Two SPs racing in Mario Kart over the emulated link">
</p>

---

# Supported Devices

| Device                | Since Version |
|-----------------------|---------------|
| Anbernic RG SP        | Always        |
| Anbernic RG 34XX      | 1.4.0         |
| Anbernic RG 34XXSP    | 1.4.0         |

## Devices I never wanted to support that slot works on

| Device                | Since Version |
|-----------------------|---------------|
| Anbernic RG 35XXSP    | 1.5.0         |
| Anbernic RG 35XX Plus | 1.5.0         |
| Anbernic RG 35XX-H    | 1.7.0         |
| Anbernic RG 35XX Pro  | 1.15.0        |
| Anbernic RG 28XX      | 1.6.0         |

---

# AI Disclosure

The Rust frontend, and the SVGs on [slot-cfw.fyi](https://slot-cfw.fyi), were put together by Claude.
I reviewed everything that was produced. All documentation is 100% free-range, meatbag prose.

My goal with this project was to recreate the feeling of playing my GBA SP in the back seat as a kid,
but with a few modern creature comforts.

Nothing about this project is exciting or groundbreaking from a technological perspective, and as a
result, I delegated the glue code. mGBA, gpSP and Gambatte have been around forever and are the true stars of
this show. The incredible skeuomorphic art is the work of [Markus](https://github.com/MarkusMaribu), an
artist who saw the bare-bones version and offered to dress it up.

I understand if my use of AI is a dealbreaker for you. Just know it's the least interesting part of
what's here.

Figured I should share the end result of all the wasted water. ✌🏻

---

# Spread Joy!

I've spent a lot of time building slot.

If you enjoy using it and feel inclined to pay it forward, go do something nice for someone! ❤️
