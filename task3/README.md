Aufgabe 3: Mehr Rust Aufgaben
=================================

Bei vielen der folgenden Aufgaben gibt es wohl nicht nur eine "richtige" Lösung: Ziel ist es eher, dass ihr euch Gedanken darüber macht, welche Typen ihr an welcher Stelle verwendet. Diskutiert auch gerne mit euren Nachbarn über eure Entscheidungen und Ideen.

Außerdem: In dieser Aufgabe gibt es recht viele Unteraufgaben. Ihr könnt gerne schon nach Lösen der ersten Aufgabe den PullRequest eröffnen und regelmäßig eure Lösungen pushen. So kann ich gut verfolgen, wie weit ihr so seid.

### 3.1 Praktikumsgruppennamen

In `groups.rs`: Schreibt eine Funktion `group_letter`, die den Namen einer Praktikumsgruppe (*Plantex*, "AVZ-Run", "Space Game") als Parameter übergeben bekommt. Die Funktion gibt ein `char` mit dem Buchstaben, der zu der Gruppe gehört ('a', 'b' oder 'c'), zurück.


### 3.2 Tageszeiten

a) In `time.rs`: Schreibt eine Funktion, die abhängig von der Uhrzeit (nur die Stunde ist angegeben) einen Gruß-String zurück gibt. Zwischen 8 und 12 Uhr ist es "Guten Morgen", von 18 bis 22 "Guten Abend" und sonst "Hallo". Der Eingabeparameter der Funktion ist ein `u8`, die anderen Typen müsst ihr euch überlegen.

b) Fügt einen weiteren Fall hinzu: Zwischen 0 und 5 Uhr soll "Warum bist du denn um X Uhr noch wach?" zurückgegebene werden, wobei 'X' die gegebene Stunde ist. Das `format!` Macro (ebenfalls aus der Standardbibliothek) könnte hierbei helfen.


### 3.3 Sieb des Eratosthenes

Implementiert in `sieve.rs` das Sieb des Eratosthenes. Das funktioniert so:

```
Man hält sich eine Liste für alle Zahlen von 2 bis n vor.
Für t von 2 bis n:
  "Streiche" alle Vielfachen von t aus der Liste
Alle restlichen Zahlen in der Liste sind nun Primzahlen.
```

Überlegt euch, welche Datenstruktur ihr verwenden solltet (nehmt die Wörter "Liste" und "streichen" nicht zu ernst). Implementiert dann eine Funktion, die ein `n` gegeben bekommt und den Algorithmus ausführt. Zurückgeben soll die Funktion einen `Vec`, der alle Primzahlen enthällt.


### 3.4 Geometrien

In dieser Aufgabe geht es darum, ein paar simple Typen zu erstellen, die Methoden besitzen (in `geo.rs`).

a) Erstellt einen `Point` Typen, der über die zwei Float-Variablen `x` und `y` verfügt, die beide öffentlich sind. Implementiert:

- `new()` Konstruktor
- `origin()` Konstruktor, der den (0, 0) Punkt erstellt
- `is_origin()`
- `distance()`: errechnet den Abstand zwischen zwei Punkten, statische Methode

b) Erstellt den Typen `Rectangle`, der ein Rechteck darstellt, dessen Seitenkanten parrallel zu den Achsen verlaufen (axis aligned). Überlegt euch, welche Variablen es speichern muss und implementiert die Methoden:

- `new()` Konstruktor
- `area()`
- `contains()` gibt `true` zurück, falls der gegebene Punkt in dem Rechteck liegt
