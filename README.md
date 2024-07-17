# Ryujin III Driver

An implementation of Ryujin III driver using Rust with `rusb`

## ⚠️ Not in active development

Recently I switched to Windows (so embarrassing), so I first of all, no longer need this library and secondly cannot continue developing it due to limits and complexities of Windows. I have no plans to continue working on the library anytime soon.

Although I want to say that I achieved what I wanted from Armoury Crate: silence (VRM fan is so noisy) and complete darkness (I don't have RGB case lighting and don't plan to, so if I cant control the LCD I want to turn it off). If you are looking for the same goals as I you can use this library as is

## Specification

I wrote all reverse engineered commands in [specs](spec/README.md) folder, if you want to create your own library you can use these specifications and it's implementation

## Supported features

For a little example you can refer to `example` folder

- Setting pump and VRM fan speed:

```rust
ryujin.set_duty(100, 0)?; // Turns off VRM fan and sets the pump to 100%
```

- Turn off LCD screen:

```rust
ryujin.turn_off_screen()?;
```

- Print text on the screen (incomplete, not reverse engineered text styling):

```rust
ryujin.print_text_on_screen("Hello, World!")?;
```

- Set one of five default animations:

```rust
use ryujin_iii_driver::DefaultAnimation;

ryujin.set_default_animation(DefaultAnimation::First)?;
```

## Roadmap

- [ ] Improve specs
- [ ] Reverse engineer text styling
- - [ ] Implement text styling
- [ ] Implement customized slideshow
- [ ] Implement showing time on the screen
