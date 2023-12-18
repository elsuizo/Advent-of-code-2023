# Dia2: Enigma de cubo

## Parte1

Nos hemos lanzado alto en la atmosfera, el apex de nuestra trayectoria ha
llegado a una isla flotante en el cielo y un elfo corre a saludarnos. El elfo
nos explica que hemos arribado a la isla de nieve y se disculpa por la falta de
esta. El no tiene muchos visitantes alli nos dice si queremos jugar a un juego
mientras tanto, mientras caminamos el elfo nos muestra una bolsa vacia y algunos
cubos los cuales son rojo, verde, o azul. Cada vez que jugamos a este juego este
nos va a estar escondiendo un secreto del numero de cubos de cada color en la
bolsa y nuestro objetivo es encontrar la informacion sobre el numero de cubos

Para obtener la informacion, una vez que la bolsa ha sido cargada con los cubos
el elfo va hasta la bolsa y toma algunos cubos random, nos los muestra y los
pone de nuevo en la bolsa. El hace esto un par de veces en el juego. Nosotros
jugamos algunas partidas y guardamos la informacion de cada una de ellas(el
input). Cada partida se lista on un numero de ID seguido por una lista separada
por ; de cubos que fueron revelados en las partidas, por ejemplo algunas
partidas se ven asi:

```text
Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
```

El elfo lo primero que quiere saber es cual partida podria ser posible si en la
bolsa solo contiene 12 cubos red, 13 cubos green y 14 cubos blue.

En el ejemplo las partidas 1, 2 y 5 son posibles, pero 3 es imposible porque se
pasa de reds y asi con los otros. Si sumamos los IDs de las partidas que pueden
ser posibles obtenemos un 8

Determinar que partidas pueden ser posibles si la bolsa ha sido cargada con solo
12 red, 13 green y 14 blue. Cual es la suma de los IDs de esas partidas???

## Parte2

El elfo dice que han parado de producir nieve porque ellos no tienen agua!!! y
no estan seguros de porque el agua ha parado; sin embargo podemos mostrar como
obtener una fuente de agua para que chequeemos por nosotros mismos. Mientras
continuamos caminando el elfo nos hace una segunda pregunta: en cada juego que
hemos jugado en las bolsas cual es el menor numero de cubos de cada color que
pueden hacer un un juego realizable???. De nuevo considerando el ejemplo
anterior:

```text
Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
```

 - en el juego 1 el juego podria realizarse con 4 reds, 2 greens y 6 blues
 - el el juego 2 el juego podria realizarse con 1 red, 3 greens y 4 blues

y asi sucesivamente. La potencia de un conjunto de cubos es igual al numero de
reds, greens y blue multiplicados. La potencia de un conjunto minimo de cubos en
el juego 1 es 48 y en los otros juegos seria 12, 1560, 630 y 36 respectivamente
si sumamos todas esas potencias nos produciria la suma de 2286

Cual es la suma de las potencias de los conjuntos minimos???
