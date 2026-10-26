You can build and flash with arduino-cli.

Build:

```bash
arduino-cli compile --fqbn arduino:avr:uno
```

Flash (will probably need adjustment if not on Linux):

```bash
arduino-cli upload -p /dev/ttyACM0 --fqbn arduino:avr:uno
```
