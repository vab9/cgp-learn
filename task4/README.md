Aufgabe 4: Pfade, Traits und solche Sachen
=============================================

### 4.1 Tolle Funktion

Schreibt eine Funktion, welche zwei Parameter eines "beliebigen" Types nimmt und dann die Summe UND das Produkt zurück gibt.


### 4.2 Swagger

Erstellt einen Typen, `Swagger`, der einen anderen Typen, welcher `Display` implementiert, in sich speichert. Euer Typ soll selber `Display` implementieren: Wenn er ausgegeben wird, soll er erst "#swag", dann die Ausgabe des in sich gespeicherten Objektes und dann "#yolo" ausgeben.


### 4.3 Fibonacci

Erstellt einen Typen, der das Trait `Iterator` aus der Standardbibliothek imlpementiert. Euer Typ soll ein Iterator über die Zahlen der Fibonacci Reihe sein. Also bei diesem Code:

```rust
for i in YourType::new().take(20) {
    println!("{}", i);
}
```

...sollen die ersten 20 Fibonacci Zahlen ausgegeben werden.


### 4.4 Toller Taschenrechner (Zusatzaufgabe)

Implementiert einen Taschenrechner `calc.rs`, der eine einfache Rechnung über die Kommandozeile einließt und das Ergebnis ausgibt. Die Benutzung soll so aussehen:

```bash
$ ./calc 3 + 4
7
$ ./calc 9 - 8
1
$ ./calc peter
I can't calculate this :(
$ ./calc 3+4        # this is not valid, parameters need to be space separated
I can't calculate this :(
```

An die CLI-Parameter kommt man mit Hilfe von `std::env::args()`, hilfreich wird auch  Als Operation ist nur '+' und '-' erlaubt und es gibt immer nur zwei Operanden (d.h. "eine Rechnung").
