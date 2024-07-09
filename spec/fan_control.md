<!-- markdownlint-disable MD033 -->

# Fan Control

## Presets

**Note**: All commands start from `0xEC 0x1A 0x01`

**Note**: Maybe `0x01` is part of the command. Look at the difference between `URB_SUBMIT` and `URB_COMPLETE`

### <span style="background-color:rgb(26, 95, 180); color: rgb(255, 255, 255); border-radius: 4px; padding: 0 2px 0 2px">Full speed</span>

- Command: `0x64 0x64` (`dd`)
- Endpoint: `0x02`
- Display filter: `usb.capdata[3-4] == 64:64`
- <details>
  <summary>usb.capdata</summary>

  **URB_SUBMIT**:

  ```terminal
  0000   ec 1a 01 64 64 00 00 00 00 00 00 00 00 00 00 00
  0010   00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00
  0020   00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00
  0030   00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00
  0040   00
  ```

  **URB_COMPLETE**:

  ```terminal
  0000   ec 1a 00 01 64 64 00 00 00 00 00 00 00 00 00 00
  0010   00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00
  0020   00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00
  0030   00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00
  0040   00
  ```

</details>

### <span style="background-color:rgb(53, 132, 228); color: rgb(250, 249, 248); border-radius: 4px; padding: 0 2px 0 2px">Turbo</span>

- Command: `0x37 0x46` (`7F`)
- Endpoint: `0x02`
- Display filter: `usb.capdata[3-4] == 37:46`
- <details>
  <summary>usb.capdata</summary>

  **URB_SUBMIT**:

  ```terminal
  0000   ec 1a 01 37 46 00 00 00 00 00 00 00 00 00 00 00
  0010   00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00
  0020   00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00
  0030   00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00
  0040   00
  ```

  **URB_COMPLETE**:

  ```terminal
  0000   ec 1a 00 01 37 46 00 00 00 00 00 00 00 00 00 00
  0010   00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00
  0020   00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00
  0030   00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00
  0040   00
  ```

</details>

### <span style="background-color:rgb(98, 160, 234); color: rgb(0, 0, 0); border-radius: 4px; padding: 0 2px 0 2px">Standard</span>

- Command: `0x28 0x32` (`(2`)
- Endpoint: `0x02`
- Display filter: `usb.capdata[3-4] == 28:32`
- <details>
  <summary>usb.capdata</summary>

  **URB_SUBMIT**:

  ```terminal
  0000   ec 1a 01 28 32 00 00 00 00 00 00 00 00 00 00 00
  0010   00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00
  0020   00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00
  0030   00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00
  0040   00
  ```

  **URB_COMPLETE**:

  ```terminal
  0000   ec 1a 00 01 28 32 00 00 00 00 00 00 00 00 00 00
  0010   00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00
  0020   00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00
  0030   00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00
  0040   00
  ```

</details>

### <span style="background-color:rgb(153, 193, 240); color: rgb(0, 0, 0); border-radius: 4px; padding: 0 2px 0 2px">Silent</span>

- Command: `0x14 0x1e` (`··`)
- Endpoint: `0x02`
- Display filter: `usb.capdata[3-4] == 14:1e`
- <details>
  <summary>usb.capdata</summary>

  **URB_SUBMIT**:

  ```terminal
  0000   ec 1a 01 14 1e 00 00 00 00 00 00 00 00 00 00 00
  0010   00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00
  0020   00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00
  0030   00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00
  0040   00
  ```

  **URB_COMPLETE**:

  ```terminal
  0000   ec 1a 00 01 14 1e 00 00 00 00 00 00 00 00 00 00
  0010   00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00
  0020   00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00
  0030   00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00
  0040   00
  ```

</details>
