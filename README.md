# The Bus Stream Deck Plugin

[English version](README.en.md)

Dieses Plugin nutzt die Telemetrie-Schnittstelle des TML-Studios-Spiels „The Bus“, um In-Game-Funktionen über das Stream Deck zu steuern und Informationen auf den Tasten anzuzeigen.

### Die folgenden Funktionen sind verfügbar und wurden mit der Version 3.3 von The Bus getestet:

* Statusanzeige: Spieler sitzt auf dem Fahrersitz
* Feststellbremse
* Haltestellenbremse
* Zündungssteuerung
* Gangwahlschalter
* Blinkersteuerung inklusive Warnblinkanlage
* Türsteuerung inklusive Türfreigabe
* Geldwechsel-Tasten

### Bekannte Probleme

* Der Solaris Urbino (auch bekannt als Galaxis Urban) meldet einen falschen Gangzustand an den Telemetrie-Endpunkt. Der Gang kann zwar gewechselt werden, aber nur die „N“-Taste leuchtet.

### Installations- und Update-Anweisungen

1. Lade die ZIP-Datei `de.thatzok.thebus.sdPlugin.zip` (du benötigst nur diese) von der neuesten Version auf der [Releases-Seite](https://github.com/thatzok/TheBus-StreamDeck-Plugin/releases) herunter.
2. Doppelklicke auf die heruntergeladene Datei.
   Das Stream Deck wird fragen, ob du das Plugin installieren möchtest. Klicke auf Installieren.

### Einrichtung & Voraussetzungen

1. The Bus: Stelle sicher, dass das Spiel läuft.
2. Stream Deck Software: Version 6.6 oder höher ist erforderlich.
3. Plugin-Konfiguration: Nach der Installation findest du in deiner Stream Deck Aktionsliste eine neue Kategorie namens „`The Bus`“. Ziehe eine beliebige Aktion (z. B. „Zündung“ oder „Türtaste“) auf eine Taste.
4. Viel Spaß!

### Danksagungen

Die Property Inspector und einige Icons wurden vom offiziellen TML-Plugin übernommen.

### Lizenz

Dieses Programm ist freie Software: Sie können es unter den Bedingungen der GNU General Public License, wie von der Free Software Foundation veröffentlicht, entweder in Version 3 der Lizenz oder (nach Ihrer Wahl) jeder späteren Version, weitergeben und/oder modifizieren.

Dieses Programm wird in der Hoffnung verbreitet, dass es nützlich sein wird, aber OHNE JEDE GEWÄHRLEISTUNG; sogar ohne die implizite Gewährleistung der MARKTFÄHIGKEIT oder EIGNUNG FÜR EINEN BESTIMMTEN ZWECK. Siehe die [GNU General Public License](LICENSE) für weitere Details.

Sie sollten eine Kopie der GNU General Public License zusammen mit diesem Programm erhalten haben. Wenn nicht, siehe <https://www.gnu.org/licenses/>.
