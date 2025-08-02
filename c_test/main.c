#include "raylib.h"
#include <stdio.h>

int main() {
  SetTraceLogLevel(LOG_NONE);
  InitWindow(600, 400, "Hello, World!");
  SetTargetFPS(60);

  
  Image x = LoadImage("mia.png");
  printf("width: %d, height: %d, format: %d\n", x.width, x.height, x.format);

  while (!WindowShouldClose()) {
    BeginDrawing();

    ClearBackground(RAYWHITE);
    DrawText("Congrats! You created your first window!", 10, 10, 20, LIGHTGRAY);

    EndDrawing();
  }

  CloseWindow();
}