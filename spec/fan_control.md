# Fan Control

## Presets

As far as I can tell, the pump itself doesn't know anything about the temperatures, so the temperatures are monitored by the host, which then sends a command that sets the VRM speed of the fan and the pump in percentages

## Control

Sending `0xEC 0x1A 0x01 0x64 0x64` (remaining space, 65 bytes in total, filled by zeroes) to the `0x02` endpoint sets both the pump and VRM fan to full speed

Where first `0x64` sets the speed of the pump (100%) and second `0x64` sets the speed of the VRM fan (also 100%)
