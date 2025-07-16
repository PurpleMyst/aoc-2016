# /// script
# requires-python = ">=3.13"
# dependencies = [
#     "pygame",
# ]
# ///
import pygame
import math
import random

# Initialize pygame
pygame.init()


def color(h: str) -> tuple[int, int, int]:
    """Convert hex color string to RGB tuple."""
    return tuple(int(h[i:i + 2], 16) for i in (1, 3, 5))  # type: ignore

# Constants
RADIUS = 300
WIDTH = HEIGHT = math.ceil(RADIUS * 2.2)
CENTER_X, CENTER_Y = WIDTH // 2, HEIGHT // 2

# Colors
WHITE = color("#FFF1E8")
BLACK = (0, 0, 0)
RED = color("#FF004D")
BLUE = color("#29ADFF")
GREEN = color("#008751")
ORANGE = color("#FFA300")
GREY = color("#C2C3C7")

def draw_points_on_circle(screen, center_x, center_y, radius, points):
    """Draw N evenly spaced points around a circle"""
    n_points = len(points)
    angle_offset = math.pi / 2
    angle_step = 2 * math.pi / n_points

    positions = [
        (int(center_x + (radius - 3/2) * math.cos(angle_offset + i * angle_step)),
        int(center_y + (radius - 3/2) * math.sin(angle_offset + i * angle_step)))
        for i in range(n_points)]

    p = max(map(len, map(str, points)))
    for i in range(n_points):
        font = pygame.font.Font(None, int(72 * math.exp(-0.05 * len(points))))
        text = font.render(str(points[i]), True, BLACK)
        w, h = text.get_size()
        x, y = positions[i]

        color = GREY

        pygame.draw.circle(screen, color, (x, y), max(w, h) // 2 + 3)
        screen.blit(text, (x - w // 2,y - h // 2))

def main():
    screen = pygame.display.set_mode((WIDTH, HEIGHT))
    pygame.display.set_caption("Josephus Problem")
    clock = pygame.time.Clock()
    removed = []
    points = list(range(1, int(random.normalvariate(10, 1))))

    running = True
    while running:
        for event in pygame.event.get():
            if event.type == pygame.QUIT:
                running = False
            elif event.type == pygame.KEYDOWN:
                if event.key == pygame.K_MINUS and len(points) > 1:
                    removed.append(points.pop(len(points) // 2))
                    points.append(points.pop(0))
                elif event.key == pygame.K_PLUS:
                    if removed:
                        points.insert(0, points.pop())
                        points.insert(math.ceil(len(points) / 2), removed.pop())
                    else:
                        points.append(len(points) + 1)
                elif event.key == pygame.K_ESCAPE:
                    running = False
        
        # Clear screen
        screen.fill(WHITE)
        
        # Draw circle outline for reference
        pygame.draw.circle(screen, BLUE, (CENTER_X, CENTER_Y), RADIUS, 5)
        
        # Draw center point
        pygame.draw.circle(screen, BLACK, (CENTER_X, CENTER_Y), 3)
        
        # Draw evenly spaced points
        draw_points_on_circle(screen, CENTER_X, CENTER_Y, RADIUS, points)

        # Show removed
        font = pygame.font.Font(None, 36)
        if removed:
            removed_text = font.render(f"Removed: {', '.join(map(str, removed))}", True, RED)
            screen.blit(removed_text, (10, 10))
        
        pygame.display.flip()
        clock.tick(60)
    
    pygame.quit()

if __name__ == "__main__":
    main()
