# Conway's Game of Life – Laboratorio de Renderizado en Tiempo Real

## 🎯 Objetivo

Este laboratorio tiene como objetivo practicar conceptos de **renderizado en tiempo real**, implementando desde cero el famoso algoritmo de **Conway’s Game of Life** usando únicamente la función `point`.

---

## 🕹️ Misión

Tu tarea es implementar el algoritmo utilizando solo la función `point` que creaste previamente. Debes:

- Añadir una función `get_color` para leer el color de una celda.
- No borrar el framebuffer entre frames (la lógica del juego ya lo controla).
- Trabajar con un **framebuffer más pequeño que la ventana** para una mejor visualización.

---

## 📐 Reglas del Juego

Cada celda (píxel) tiene 8 vecinos. Puede estar:

- **Viva**: blanco (u otro color).
- **Muerta**: negro (u otro color).

En cada frame (turno):

1. 🔻 Una célula **viva** con menos de **2 vecinos vivos** muere. *(underpopulation)*
2. 🟰 Una célula **viva** con **2 o 3 vecinos vivos** sobrevive. *(survival)*
3. 🔺 Una célula **viva** con más de **3 vecinos vivos** muere. *(overpopulation)*
4. 🌱 Una célula **muerta** con exactamente **3 vecinos vivos** revive. *(reproduction)*

> 🕰️ Se recomienda agregar un `delay` entre frames para visualizar correctamente la evolución.

---

## 🧪 Tips Técnicos

- Puedes implementar toda la lógica en la función `render`.
- Trabaja inicialmente en una **resolución baja** como `100x100`.
- Aumenta la resolución para pruebas finales.

---

## 🧬 Patrón Inicial

Debes definir un patrón inicial de celdas vivas (lista de coordenadas).

### Se espera que:

- Sea **creativo**
- **Llene una buena parte de la pantalla**
- Implemente al menos **10 organismos**

Pueden ser patrones clásicos, o puedes inventar uno propio. Aquí un ejemplo de cómo debería verse tu pantalla:

![Ejecución esperada](./a380d2b4-1ef4-483a-b05b-58444b4867a7.png)

Consulta patrones aquí: [Wikipedia – Conway’s Game of Life](https://en.wikipedia.org/wiki/Conway%27s_Game_of_Life)

---

## 💡 Ejemplos de Patrones

A continuación algunos patrones clásicos que puedes implementar como parte del patrón inicial:

![Patrones Clásicos](./931cab73-2dbe-44a5-ab2f-99a724084c1d.png)

- **Still Lifes**: Block, Beehive, Loaf, Boat, Tub
- **Oscillators**: Blinker, Toad, Beacon, Pulsar, Pentadecathlon
- **Spaceships**: Glider, LWSS, MWSS, HWSS

> Implementa una función por cada organismo para modularizar tu código.

---

## 🧠 Consideraciones sobre los Bordes

Hay dos enfoques posibles:

1. **Bordes muertos**: toda celda fuera del framebuffer se considera muerta.
2. **Pantalla envolvente**: el borde derecho se conecta con el izquierdo, y el superior con el inferior.

El segundo método genera patrones más complejos e interesantes.

---

## 📝 Entrega

Debes entregar:

1. ✅ Un repositorio en Git con tu código funcional.
2. ✅ Un **GIF** mostrando el juego corriendo por unos segundos.

> El GIF debe:
> - Estar embebido en este `README.md`
> - Subirse también al canal de Discord de la clase

---

## 🎁 Extra

> 🧠 **Conway's Game of Life es Turing Complete**.  
> Puedes simular cualquier sistema computacional.  
> Si encuentras una manera de determinar si un patrón inicial siempre converge a otro, podrías reescribir las bases de la matemática moderna.

---

## 📌 Nota Final

Tendrás nota completa si:

- Tu patrón inicial es **creativo**
- Implementas **al menos 10 organismos**
- Tu implementación corre de forma fluida y clara

---

¡Dale vida a tu código! 🚀
