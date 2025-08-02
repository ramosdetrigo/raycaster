#include "raylib.h"

int main() {
  InitWindow(600, 400, "Hello, World!");
  SetTargetFPS(60);

  while (!WindowShouldClose()) {
    BeginDrawing();

    ClearBackground(RAYWHITE);
    DrawText("Congrats! You created your first window!", 10, 10, 20, LIGHTGRAY);

    EndDrawing();
  }

  CloseWindow();
}