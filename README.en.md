# The Bus Stream Deck Plugin

[Deutsche Version](README.md)

This plugin uses the telemetry interface of the TML Studios game "The Bus" to control in-game functions from the Stream
Deck and display information on buttons.

### The following features are available and have been tested with The Bus version 3.3:

* Status display: Player is in the driver's seat
* Parking brake
* Bus stop brake
* Ignition control
* Gear selektor
* Indicator control including warning lights
* Door buttons including door clearance
* Cash change buttons

### Known Bugs

* Solaris Urbino (aka Galaxis Urban) reports incorrect gear state to the telemetry endpoint. The gear can be changed,
  but only the "N" button lights up

### Installation and Update instructions

1. Download the ZIP-File `de.thatzok.thebus.sdPlugin.zip` (you only need this one) from the latest release on
   the [releases page](https://github.com/thatzok/TheBus-StreamDeck-Plugin/releases)
2. Double-click the downloaded file.
   Stream Deck will ask if you want to install the plugin. Click Install.

### Setup & Requirements

1. The Bus: Make sure the game is running.
2. Stream Deck Software: Version 6.6 or higher is required.
3. Plugin Config: Once installed, you will find a new category called "`The Bus`" in your Stream Deck action
   list. Drag and drop any action (e.g., "Ignition" or "Door Button") onto a key.
4. Enjoy!

### Acknowledgements

The property inspectors and some icons were taken from the TML plugin.


### License

This program is free software: you can redistribute it and/or modify it under the terms of the GNU General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version.

This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the [GNU General Public License](LICENSE) for more details.

You should have received a copy of the GNU General Public License along with this program. If not, see <https://www.gnu.org/licenses/>.
