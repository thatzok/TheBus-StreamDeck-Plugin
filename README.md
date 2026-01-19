# The Bus Stream Deck Plugin

This plugin uses the telemetry interface of the TML Studios game "The Bus" to control in-game functions from the Stream
Deck and display information on buttons.

### The following features are available and have been tested with The Bus version 3.2:

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
2. Close the Stream Deck software (or the unpack/copy process will maybe fail)
3. Copy the ZIP-File to the Stream Deck software plugins folder `%AppData%\Elgato\StreamDeck\Plugins`
3. Unpack the ZIP-File
4. Restart the Stream Deck software
5. Configure the plugin in the Stream Deck software
6. Enjoy!

### Acknowledgements

The property inspectors and some icons were taken from the TML plugin.


## License

This program is free software: you can redistribute it and/or modify it under the terms of the GNU General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version.

This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the [GNU General Public License](LICENSE) for more details.

You should have received a copy of the GNU General Public License along with this program. If not, see <https://www.gnu.org/licenses/>.
