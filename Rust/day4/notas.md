# Dia 4

## Parte1

Juegos de raspaditas: Mientras abandonamos la gondola, la primer cosa que
notamos es que el aire aqui es mucho mas caliente que en la isla de la nieve. Es
tambien muy humedo, es de aqui de donde proviene el agua???

La proxima cosa que notamos es que un elfo se sienta sobre el piso a lo largo de
la estacion junto a una pila de cartas coloridas

"Oh hola como andas" el elfo viene hacia nosotros "en que te puedo ayudar???"
nosotoros le decimos sobre las fuentes de agua.

"Mmm no se yo solo opero el mecanismo de elevacion de la gondola quizas el
jardinero sepa, el esta en una isla diferente llamada er, que esta rodeada por
agua, sabes que si puedes ayudarme con un problema te prestare mi bote y podras
visitar esa isla. Tengo todas estas raspaditas como regalo, pero no puedo saber
cuales de ellas son las ganadoras"

El elfo nos lleva a la pila de cartas de colores. Alli descubrimos un moton de
raspaditas, todas con la pintura raspadas, tomamos una y vemos que cada carta
tiene dos listas de numeros separados por una linea vertical `|`: una lista de
numeros ganadores y entonces una lista de numeros que nosotros tenemos.
Organizamos la informacion en tablas (el input)

Tenemos que encontrar cual de los numeros que tenemos aparecen en la lista de
numeros ganadores. El primer match hace que la carta gane 1 punto y cada match
despues de ese duplica el valor del punto de la carta. Por ejemplo:

```text
Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
```


