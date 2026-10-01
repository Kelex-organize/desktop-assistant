# Visión

## Qué es

**Kai** es un asistente de escritorio para Windows que me facilita tareas o las hace por mí cuando me da pereza hacerlas: abrir programas, poner una alarma, anotar algo, pausar la música, buscar los ítems de un campeón de TFT. Se lo pido escribiendo o hablando y lo hace.

## Qué NO es

**No es un chat con IA.** No hay ventana de conversación ni respuestas largas. Cuando le pido algo, lo hace y me confirma en una frase corta.

La IA, cuando se agregue, cumple dos funciones:
1. **Interpretar**: entender pedidos escritos de cualquier forma ("che, poneme una alarma para las siete") y traducirlos a comandos de la app.
2. **Responder con buena onda**: confirmaciones cortas y amigables, y contestar saludos ("Kai, buenos días" → "¡Buen día! ¿Arrancamos?").

## Cómo se usa: tres modos

1. **Personaje**: una figura animada siempre visible en una esquina (estilo coucou). Reacciona a lo que pasa: escucha, piensa, confirma. **Se puede ocultar.**
2. **Barra rápida** (tipo Spotlight): aparece con un atajo de teclado, escribo el pedido, Enter, y desaparece.
3. **Ventana completa**: para las cosas más serias: ver y editar alarmas, notas, configuración, los paneles de cada módulo.

Además: **comandos de voz** con un atajo (push-to-talk), algo prioritario porque es lo más cómodo, por ejemplo mientras juego.

## Primeros módulos

- **Alarmas y recordatorios**: suenan y avisan aunque la ventana esté cerrada
- **MetaTFT**: "items de Caitlyn" → abre `https://www.metatft.com/units/Caitlyn`
- **Notas**: crear, ver, buscar
- **Abrir aplicaciones**: cualquier app instalada
- **Spotify básico**: pausar, continuar, siguiente, anterior

## Principios

1. **App primero, IA después.** Todo funciona sin IA y sin internet. La IA mejora cómo se pide, no lo que se puede hacer.
2. **Modular.** Agregar una funcionalidad = agregar un módulo, sin tocar los demás.
3. **Siempre a mano, nunca molesta.** Liviana en segundo plano.
4. **Segura.** No ejecuta nada peligroso sin preguntar. Nunca ejecuta texto libre en una shell.
5. **Hecha con prácticas profesionales**: tests, ramas, PRs, CI.
6. **Es mía para aprender**: Rust, React, Tauri, e integración de IA en una app real.

## Fuera de alcance (por ahora)

- Sincronizar con otros dispositivos (todo queda en la PC)
- Mac, Linux, celular
- Funcionar con la PC apagada o suspendida
- Controlar otras apps haciendo clics como un usuario
- Recordar pedidos anteriores / tareas de varios pasos (el diseño lo deja preparado; se agrega después)
