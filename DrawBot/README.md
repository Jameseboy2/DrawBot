You can build and flash with arduino-cli.

Set up arduino-cli for the esp8266 core:
```bash
arduino-cli config set board_manager.additional_urls https://arduino.esp8266.com/stable/package_esp8266com_index.json
arduino-cli core update-index
arduino-cli core install esp8266:esp8266
```

Build:

```bash
arduino-cli compile --fqbn esp8266:esp8266:nodemcuv2
```

Flash (will probably need adjustment if not on Linux):

```bash
arduino-cli upload -p /dev/ttyACM0 --fqbn esp8266:esp8266:nodemcuv2
```
