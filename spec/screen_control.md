# Screen control

## Turn off

The screen can be turned off by sending `0xEC 0x51` (remaining space, 65 bytes in total, filled by zeroes) to the `0x02` endpoint

## Text (Incomplete)

Sending `0xEC 0x53 0x00 <text as bytes in ascii>` (remaining space, 65 bytes in total, filled by zeroes) to the `0x02` endpoint will print the text on screen. However text styles not yet reverse engineered

## Default animation

To set default animation send `0xEC 0x5D 0x14 0x00 <mode>` (remaining space, 65 bytes in total, filled by zeroes) to the `0x02` endpoint where `<mode>` is range between `0x00` (0) and `0x04` (4)

## Time

Sending `0xEC 0x51 0x08 0x00 0x00` (remaining space, 65 bytes in total, filled by zeroes) to the `0x02` endpoint will set display to show time in 12h format where as sending `0xEC 0x51 0x08 0x00 0x01` will set display to show time in 24h

## Customized slideshow (Not yet implemented)

### Default animations

Send the following data to the `0x02` endpoint to set up a slideshow with 5 slides (all default animations in order).
Armoury Crate allows you to set up to 5 slides, I guess that's the limit of this package

```terminal
0000   ec 5d 00 05 14 00 00 05 14 00 01 05 14 00 02 05
0010   14 00 03 05 14 00 04 05 00 00 00 00 00 00 00 00
0020   00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00
0030   00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00
0040   00
```

#### Packet breakdown

- Command `0xEC 0x5D 0x00`
- Number of slides `0x01`..`0x05`
- Slide `0x14 0x00 0x01 0x05`
- - Delimiter `0x14 0x00` and `0x05`
- - Default animation number `0x00`..`0x04`
- Remaining space `0x00 0x00 0x00 0x00 0x00 0x00 0x00 0x00 0x00`
