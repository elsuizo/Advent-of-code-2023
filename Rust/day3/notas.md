# Dia3

## Parte 1

Tu y el elfo eventualmente alcanzan la estacino gondola lift; ellos dicen que
van a brindarnos agua, pero esta un poco lejos por lo que vamos a ella

No nos toma mucho tiempo encontrar las gondolas pero parece haber un problema:
ellas no se mueven

Nos damos vuelta y vemos a un elfo todo grasiento con una llave y una mirada de
sorpresa. "Los siento no esperaba a nadie!!!". La gondolo no esta funcionando
ahora mismo; y va a pasar un rato hasta que funcione" entonces nos ofrecemos
para ayudar

El ingeniero nos explica que hay una parte del motor que esta perdida, pero
nadie puede detectar cual es. Si podemos sumar todas los numeros de las partes
en el esquematico del motor, seria facil saber cual es el que falta

El esquematico del motor(el input) consite en representaciones visuales del
motor. Hay un monton de numeros y simbolos que no entendemos, pero aparentemente
cualquier numero adjacente a un simbolo aun en diagonal es un numero de parte y
deberia ser incluido en la suma(los puntos no cuentan como simbolo .)

Veamos un ejemplo de esquematico de motor:

```text
467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..
```

En este esquematico, dos numeros no son un numero de parte porque ellos no
tienen un simbolo adjacente: 114(arriba a la derecha) y 58(al medio a la
derecha) todos los demas numeros son adjacentes a un simbolo y entonces son un
numero de parte; su suma es: 4361

Cual es la suma de todos los numeros de parte en el esquematico del motor???
