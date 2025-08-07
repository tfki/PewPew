```launchpad.c``` &rarr; ```rfPacketRx.c``` <br>
```sensortag.c``` &rarr; ```rfPacketTx.c```

# PewPew (IOT LAB Final) 

##### Sebb's special place (for special people)
- ```cargo run --bin ur_sandbox```
- ```cargo run``` if src/main.rs exists (with main func (```pub fn main(){}```))

## DOD (Definition of Done)
### Modus 1
- [ ] auf Bildschirm schießen mit HitReg durch BinSearch
### Modus 2
- [ ] Multiplayer Mohrhuhn
  - [ ] Multiplayer auf gleichem Bildschirm
  - [ ] custom, präzise Hitboxen
  - [ ] statisches Bild (nicht rechts/links verschieben)
  - [ ] Figuren
    - [ ] fliegende Hühner
    - [ ] bienendude
    - [ ] rennende Hühner
    - [ ] Pfannendude
  - [ ] Jogger verschwinden hinter mind. einem Obj
  - [ ] Punktestand für Spieler und Timer
  - [ ] Scoreboard vom aktuellen Spiel
  - [ ] **Reload per Gyro Flick** nach x Schuss --> Munitionanzeige
  - [ ] Ton bei Schuss & Treffer
  - [ ] **Pre-game lobby**, wo bei schüssen spielername angezeigt wird um spieler den Sensortags (Waffen) zuzuordnen
- [ ] HitReg
  - [ ] ein ganz schwarzer Frame
  - [ ] dann **binärsuche** alle Objekte flashen
  - [ ] **<200ms** detection
  - [ ] Delay zwischen Schüssen von **1 sec** --> LED rot
  - [ ] während eines HitReg werden andere Schüsse durch selbe Frequenz registriert
  - [ ] wenn keine HitReg möglich dann einfach kein Treffer (halt das Teil still!)
  - [ ] wenn 2 dasselbe treffen bekommen beide Punkte (jeder die Hälfte)
- [ ] Waffe
  - [ ] **3D gedruckte** Halterung für sensortag (länge, dicke, etc testen)
  - [ ] Lauf vor lichtsensor
  - [ ] trigger durch **eigenen Knopf --> anlöten**
  - [ ] **LED** nach außen sichbar machen --> Glasfaser?
  - [ ] Halterung für board --> klicken, schrauben, pins?
- [ ] Sensorboard
  - [ ] GCU (Gun control Unit)
  - [ ] Nachlade-Gyro-Flick Erkennung
  - [ ] LED
    - rot leuchten bei 1sec schussdelay
    - blinken wenn magazin leer
  - [ ] schickt brightness bei änderung mit timestamp
  - [ ] ID
  - [ ] aperiodisch magazin (füllstand + max size) nachricht senden
- [ ] Pregame lobby
  - zeigt übrige schuss aller spieler an
  - spiel geht los wenn alle volles mag haben
  - einmal schießen um lobby zu  betreten
- [ ] launchpad
  - concentrator
  - empfängt alle GCU daten
  - schickt daten per UART an Moorhuhn Control Unit (MCU) (PC)
