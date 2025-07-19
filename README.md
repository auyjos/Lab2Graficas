# Conway's Game of Life 

Este es un proyecto que implementa el famoso [Juego de la Vida de Conway](https://en.wikipedia.org/wiki/Conway%27s_Game_of_Life) utilizando Rust y la biblioteca Raylib.

## 📝 Descripción

El Juego de la Vida no es realmente un "juego" en el sentido tradicional, sino un autómata celular desarrollado por el matemático británico John Horton Conway en 1970. Consiste en una cuadrícula bidimensional de células que evolucionan según un conjunto simple de reglas basadas en sus vecinos.

![Ejemplo del Juego de la Vida](https://upload.wikimedia.org/wikipedia/commons/e/e5/Gospers_glider_gun.gif)

## 🎮 Demostración

![Demostración del Juego](./assets/game.gif)

## 🧠 Reglas del juego

1. Una célula **viva** con menos de 2 vecinos vivos **muere** por subpoblación.
2. Una célula **viva** con 2 o 3 vecinos vivos **sobrevive** a la siguiente generación.
3. Una célula **viva** con más de 3 vecinos vivos **muere** por sobrepoblación.
4. Una célula **muerta** con exactamente 3 vecinos vivos **revive** por reproducción.

Estas simples reglas producen patrones sorprendentemente complejos y comportamientos emergentes.

## ✨ Características de esta implementación

- **Sistema de colores:** Los diferentes tipos de patrones se distinguen por colores:
  - 🟡 **Amarillo (GOLD):** Estructuras estáticas (Still Lifes)
  - 🟢 **Verde (LIME):** Osciladores (Oscillators)
  - 🔴 **Rojo (RED):** Naves espaciales (Spaceships)
  - 🔵 **Azul (SKYBLUE):** Patrones personalizados
  - 🟠 **Naranja (ORANGE):** Células generadas durante la simulación

- **Controles interactivos:**
  - **ESPACIO:** Pausa/reanuda la simulación
  - **S:** Avanza un paso (una generación) cuando está en modo paso a paso
  - **R:** Reinicia la simulación con el patrón inicial
  - **FLECHA ARRIBA:** Aumenta la velocidad de la simulación
  - **FLECHA ABAJO:** Disminuye la velocidad de la simulación

- **Patrones incluidos:**
  - **Estructuras estáticas:** Block, Beehive, Loaf, Boat, Tub
  - **Osciladores:** Blinker, Toad, Beacon, Pulsar, Pentadecathlon
  - **Naves espaciales:** Glider, LWSS (Lightweight spaceship)
  - **Patrones complejos:** Gosper Glider Gun, R-Pentomino

## 🚀 Cómo ejecutar

1. Asegúrate de tener [Rust](https://www.rust-lang.org/tools/install) instalado.

2. Clona este repositorio:
   ```bash
   git clone [URL_DEL_REPOSITORIO]
   cd [NOMBRE_DEL_DIRECTORIO]
   ```

3. Compila y ejecuta el programa:
   ```bash
   cargo run --release
   ```

## 🧩 Implementación técnica

- Implementado completamente en Rust.
- Utiliza la biblioteca Raylib para la representación gráfica.
- La lógica del juego se basa en dos grids:
  - Un grid booleano para el estado vivo/muerto de cada célula.
  - Un grid de tipos para rastrear el origen de cada célula (para los colores).
- Se utiliza el enfoque de "pantalla envolvente" donde los bordes se conectan entre sí.
- Resolución del grid: 100x100 con células de 6x6 píxeles para una mejor visualización.

## 📚 Contexto educativo

Este proyecto fue desarrollado como parte del laboratorio de Renderizado en Tiempo Real para practicar conceptos de:
- Renderizado en tiempo real
- Autómatas celulares
- Programación en Rust
- Manejo de framebuffers

## 🎯 Nota sobre la filosofía del juego

El Juego de la Vida de Conway es una demostración fascinante de cómo reglas simples pueden producir comportamientos complejos. Es un ejemplo perfecto de emergencia y complejidad a partir de la simplicidad, y ha sido objeto de estudio en campos que van desde la matemática hasta la biología computacional.

¡Disfruta experimentando con el Juego de la Vida!

---

**Nota:** Para capturar un GIF del juego en funcionamiento, puedes usar herramientas como [Peek](https://github.com/phw/peek) (Linux), [ScreenToGif](https://www.screentogif.com/) (Windows) o [Gifox](https://gifox.io/) (Mac).
