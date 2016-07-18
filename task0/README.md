Aufgabe 0: Aller Anfang mit Git
===============================

In dieser Aufgabe sollt ihr erstmal dieses Repository klonen und die ersten Schritte mit Git wagen. Es ist auch eher ein Tutorial, als eine Aufgabe, also keine Sorge ;-)

Bevor ihr hier loslegt, solltet ihr auf jeden Fall die Anleitung [Git einrichten](https://github.com/OsnaCS/cgp-2016/wiki/Git-einrichten) gelesen und ausgeführt haben. Fertig? Dann passt jetzt gut auf, denn das selbe müsst ihr später nochmal für das richtige Projektrepository machen (ich werde natürlich diese Datei nicht löschen, also ihr braucht nichts auswendig zu lernen :P). Los gehts:


## 1. Repository forken
Als erstes erstellt ihr euch eine private Kopie dieses Repositories. Dafür klickt ihr oben rechts auf den "Fork"-Button. Nach wenigen Momenten solltet ihr zu eurem Fork weitergeleitet werden.

Unterscheidet immer zwischen dem Original-Repo (`github.com/LukasKalbertodt/projektname` bzw `github.com/OsnaCS/projektname`)  und eurem Fork (`github.com/WilliWacker/projektname`)!

## 2. Klonen und remotes anpassen
Jetzt ist es an der Zeit euren Fork zu `clone`n, also auf euren PC zu laden. Um das zu tun, braucht ihr die Clone-URL eures Forks, welche ihr auf der Seite eures Repositories auf der rechten Seite findet (Button "Clone or download"). Dort sollte "Clone with SSH" mit einer URL stehen. Falls dort "Clone with HTTPS" steht, fragt einen Tutor oder Sitznachbarn. Die SSH-URL kopiert ihr jetzt in die Zwischenablage.

Arbeitet bitte nur im `/local` Verzeichnis und nicht in eurem Heimatsverzeichnis. Führt dazu folgendes aus (ersetzt das RZ-Kürzel und die URL durch eure gültigen Daten):
``` bash
$ cd /local
$ mkdir wwacker
$ cd wwacker
$ git clone git@github.com:WilliWacker/cgp-learn.git
$ cd cgp-learn
```
Nach kurzer Zeit ist jetzt ein Ordner `cgp-learn` entstanden, in dem sich schon die Dateien aus diesem Repository befinden sollten.

Nun müssen wir noch das Original Repository als `remote` hinzufügen. Ein Remote besteht vereinfacht gesagt aus einem Namen und einer URL. Wir brauchen zwei Remotes:
- `origin`: Euer Fork auf GitHub
- `upstream`: Das originale Repository auf Github (in diesem Fall mein Repository)

`origin` wurde automatisch beim clonen angelegt. Wir müssen also nur noch `upstream` hinzufügen. Dazu müsst ihr die clone-URL eures originalen Repositories wie oben beschrieben herausfinden. Diese URL kann diesmal aber auch eine HTTPS clone url sein. Mit dieser URL führt ihr folgendes aus:
``` bash
$ git remote add upstream git@github.com:LukasKalbertodt/cgp-learn.git
```
Danach sollte der Befehl `git remote` die beiden korrekten Remotes anzeigen und auch der Befehl `git fetch upstream` sollte funktionieren. Vergesst im Folgenden nicht, oft `$ git status` auszuführen!


## 3. Die "Aufgabe" lösen
Eure Aufgabe ist nun:

> Erstellt eine Datei `movie.txt` und befüllt sie mit dem Namen eines Films, der euch gut gefällt.

Beachtet, dass ihr für alle Aufgaben *erstmal* einen Unterordner mit eurem Namen erstellt. Ich würde in dieser Aufgabe also `task0/lukas_kalbertodt/movie.txt` erstellen. Tatsächlich existiert dieser Ordner (also meine Lösung) schon, sodass ihr dort mal gucken könnt.

Und wie in allen weiteren Aufgaben gehört zu der Aufgabe dazu, dass ihr eure Lösung committed und einen PullRequest (PR) stellt. Der grobe Ablauf (diesmal müsst ihr die exakten Befehle selbst herausfinden) ist:

- Fügt die Datei zu git hinzu (Quiz: Welchen Status hatte die Datei vorher? Welchen Status hat sie jetzt?)
- Erstellt einen Commit (Quiz: Welchen Status hat die Datei jetzt?)

Aber die Aufgabe geht noch weiter:

> Editiert die Datei und fügt eine Aktivität hinzu, der ihr gerne nachgeht. (Jap, diese Aufgabe ist das Kennenlernspiel für Informatiker :P)

Mit Git geht es ungefähr so weiter:

- Erstellt einen Commit mit den neuen Änderungen (Quiz: In welchem Status ist die Datei?)
- Push't diesen Commit auf euren privaten Fork
- Erstellt auf Github einen Pull Request (PR):
  - Von eurem branch `master` ...
  - ... zu dem originalen branch `master`

> Euch fällt auf, dass die Datei immer noch `movie.txt` heißt... benennt diese Datei jetzt in `stuff.txt` um.

*Wichtig*: Hier nicht `$ mv` nutzen, sondern `$git mv` (das selbe gilt für `$ rm`).

- Erstellt einen Commit mit der Änderung
- Push't eure Änderung wieder zu eurem privaten Fork
- Auf der Seite eures PR's sollte jetzt ein weiterer Commit angezeigt werden
- Wartet darauf (oder sagt einem Tutor Bescheid), bis euer PR gemerged wurde

-----------------

Wartet nun, bis alle mit der Aufgabe fertig sind. Ihr könnt Däumchen drehen oder viel besser den Menschen in eurer Umgebung helfen, falls sie irgendwo nicht weiterkommen.

Sobald ihr dann von mir (Lukas) das geheime Signal hört, tut folgendes, um euch die Dateien der anderen Gruppenmitglieder zu holen:

```bash
$ git pull upstream master
```

Ihr solltet in der Ausgabe irgendwo "fast forward" sehen: Das sollte dort immer stehen, wenn ihr vom `upstream` pullt! Ihr müsst genau den oben genannten Befehl auch immer ausführen, um euch die neu freigeschalteten Aufgaben runterzuladen.

Wartet noch kurz, bis ich die nächste Aufgabe freischalte und dann legt damit los!
