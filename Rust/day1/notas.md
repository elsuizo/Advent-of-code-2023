# Dia1

## Parte1

Algo esta mal con la produccion global de nieve y hemos sidos seleccionados para
mirar que pasa.

Nos envian al cielo en una catapulta para ver porque el mapa esta casi blanco.
Mientras hacen las ultimos ajustes ellos descubren que el documento de
calibracion(el input) ha sido modificado por un elfo joven que queria mostrar
sus capacidades artisticas. Conscuentemente los elfos tienen problemas para leer
los valores del documento. El nuevo documento de calibracion consiste en lineas
de texto; cada una originalmente contenia un valor de calibracion especifico que
los elfos ahora necesitan recuperar. En cada linea los valores de calibracion
pueden encontrarse combinando el primer digito y el ultimo digito(en ese orden)
para formar un simple numero de dos digitos, por ejemplo:

```text
1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet
```

En este ejemplo los valores de calibracion son 12, 38, 15, y 77 que si los
sumamos a todos juntos nos da: 142

Condiderando todo el documento de calibracion. Cual es la suma de todos los
valores de calibracion???

## Parte2

Los calculos que hicimos no estan bien. Porque parece que algunos digitos estan
escritos con letras: one, two, three, four, five, six, seven, eight, y nine que
tambien cuentan como digitos validos

Equipados con esta informacion, necesitamos hallar cuales son los reales primer
y segundo digito de cada linea, por ejemplo:

```text
two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen
```

En este ejemplo los valores de calibracion son: 29, 83, 13, 24, 42, 14, y 76 que
si los sumamos nos da: 281

Cual es la suma de todos los valores de calibracion???
