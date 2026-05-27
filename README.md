# arctic-spa-dc-rust

A Rust CLI tool to interact with your Arctic Spa brand hot tub. Query device information, get/set device properties, and manage application settings.

This is a Rust rewrite of the Python package [arctic-spa-dc](https://github.com/matt-blodgett/arctic-spa-dc).

## Installation

Build the project with Cargo:

```bash
cargo build --release
```

The build process will automatically:
- Generate Rust code from the protobuf schema files in `src/proto_schemas/`
- Compile the Rust source code

The binary will be available at `target/release/asdc` (or `target/release/asdc.exe` on Windows).

Throughout this documentation, `asdc` refers to the built binary. You can add it to your environment path or replace it with the full executable path if needed.

## Usage

Below is a brief description of each currently available command.

### Discover

Automatically identify and display any available and valid IP Addresses of locally connected Arctic Spa brand hot tubs.

**Examples:**
```bash
# display ip addresses for any connected hot tubs
asdc discover

# display discovered ip addresses and update the config file with the first result
asdc discover --update-config
```

### Query

Request protobuf messages from the device. Returns information about device status and configuration.

**Available Message Types:**
- `live` — Status of temperatures, pumps, blowers, lights, filters, ozone, etc
- `settings` — Settings for filtration, onzen, ozone, minimum and maximum values, etc
- `configuration` — Capabilities of the hot tub such as pump layouts and installed features
- `peak` — Settings for power draw management
- `clock` — Device system clock information
- `information` — Serial numbers, firmware and hardware versions, etc
- `error` — Error status indicators
- `router` — Router details
- `filter` — Filter maintenance information
- `peripheral` — Information about installed peripheral device
- `onzen-live` — Status of orp and ph levels, electrode details, etc
- `onzen-settings` — Definitions for minimum and maximum thresholds of OnzenLive statuses

**Examples:**
```bash
# query the OnzenLive protobuf message from the host ip with silent logging
asdc query onzen-live --ip-address "192.168.0.1" --verbosity 0
```

### Device

Manage hot tub device properties. Get current values, set new values, or list all properties at once.

**Available Properties**
| Property Name | Read | Write | Valid Values |
|---|---|---|---|
| temperature-current | ✅ | ❌ | Number: 59-104 |
| temperature-setpoint | ✅ | ✅ | Number: 59-104 |
| pump-1 | ✅ | ✅ | HIGH, LOW, OFF |
| pump-2 | ✅ | ✅ | HIGH, LOW, OFF |
| pump-3 | ✅ | ✅ | HIGH, LOW, OFF |
| pump-4 | ✅ | ✅ | HIGH, LOW, OFF |
| pump-5 | ✅ | ✅ | HIGH, LOW, OFF |
| blower-1 | ✅ | ✅ | HIGH, LOW, OFF |
| blower-2 | ✅ | ✅ | HIGH, LOW, OFF |
| lights | ✅ | ✅ | ON, OFF |
| stereo | ✅ | ✅ | ON, OFF |
| heater-1 | ✅ | ❌ | IDLE, WARMUP, HEATING, COOLDOWN |
| heater-2 | ✅ | ❌ | IDLE, WARMUP, HEATING, COOLDOWN |
| filter | ✅ | ✅ | ON, OFF |
| onzen | ✅ | ✅ | ON, OFF |
| ozone | ✅ | ✅ | ON, OFF |
| exhaust-fan | ✅ | ✅ | ON, OFF |
| sauna-state | ✅ | ✅ | IDLE, PRESET_A, PRESET_B, PRESET_C, TIMER |
| sauna-time-left | ✅ | ✅ | Number: 0-120 |
| all-on | ✅ | ✅ | ON, OFF |
| fogger | ✅ | ✅ | ON, OFF |
| sds | ✅ | ✅ | ON, OFF |
| yess | ✅ | ✅ | ON, OFF |
| orp | ✅ | ❌ | Number: 0-999 |
| ph-100 | ✅ | ❌ | Number: 0-999 |
| orp-color | ✅ | ❌ | LOW, CAUTION_LOW, OK, CAUTION_HIGH, HIGH |
| ph-color | ✅ | ❌ | LOW, CAUTION_LOW, OK, CAUTION_HIGH, HIGH |
| spaboy-boost | ❌ | ✅ | ON, OFF |
| pack-reset | ❌ | ✅ | ON, OFF |
| log-dump | ❌ | ✅ | ON, OFF |

**Examples:**
```bash
# get the status of pump #1
asdc device get pump-1

# if pump #1 was "HIGH", try turning it to "OFF"
asdc device set pump-1 off

# update the temperature setpoint
asdc device set temperature-setpoint 104

# turn all pumps and lights on at once
asdc device set all-on on

# list all device properties and current values
asdc device list
```

### Config

Manage application settings stored locally. Configure IP address and verbosity level to avoid repeating flags.

**Available Properties:**
- `ip-address` — The stored IP address for your hot tub
- `verbosity` — The default logging level

**Examples:**

```bash
# get the stored ip address
asdc config get ip-address

# set the IP address for your hot tub
asdc config set ip-address 192.168.1.100

# set the default logging verbosity to INFO (3)
asdc config set verbosity 3

# list all config properties and current values
asdc config list
```

## Example Workflow

```bash
# first time setup - find your hot tubs ip address
asdc discover

# save your hot tubs ip address to avoid repetitive command line flags
asdc config set ip-address 192.168.1.100

# set logging output to silent
asdc config set verbosity 0

# check current temperature and setpoint
asdc device get temperature-current
asdc device get temperature-setpoint

# set temperature setpoint to 100°F
asdc device set temperature-setpoint 100

# turn on all jets and lights
asdc device set all-on ON

# query full device status
asdc query live
asdc query onzen-live

# get device information
asdc query information

# get device configuration
asdc query configuration

# run commands with full logging
asdc device list -v 5
```

## Disclaimer

This project is not affiliated in any way with the Arctic Spas company or brand.

A hot tub is a significant financial investment and there is inherent risk in using unauthorized third party tools to interact with your device.

By choosing to use the software, you assume full responsibility for any damage, loss, or consequences that could potentially result.
