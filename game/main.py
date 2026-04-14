# Compliance Note
# This project is based entirely on publicly available academic information and general knowledge of orbital mechanics. It contains no restricted, proprietary, or export-controlled information of any kind. This is a personal learning project only.

import logging
import pygame
import sys

# Configure basic logging
logging.basicConfig(
    level=logging.INFO,
    format='%(asctime)s - %(name)s - %(levelname)s - %(message)s'
)
logger = logging.getLogger(__name__)

def main():
    logger.info("Initializing Pygame...")
    pygame.init()

    # 800x600 window setup
    screen_width, screen_height = 800, 600
    screen = pygame.display.set_mode((screen_width, screen_height))
    pygame.display.set_caption("Delta Wrecker")

    clock = pygame.time.Clock()
    running = True

    logger.info("Starting main game loop...")
    while running:
        # Basic event loop handling QUIT
        for event in pygame.event.get():
            if event.type == pygame.QUIT:
                logger.info("QUIT event received. Exiting game loop...")
                running = False

        # TODO-MADSHADE: Add orbital mechanics updates and logic here

        # Clear the screen with a flat color every frame (dark grey)
        screen.fill((30, 30, 30))

        # Basic orbital rendering for Issue 8
        center_x, center_y = screen_width // 2, screen_height // 2

        # Draw central planet (Earth-like)
        pygame.draw.circle(screen, (0, 100, 255), (center_x, center_y), 20)

        # Draw circular orbit
        orbit_radius = 150
        pygame.draw.circle(screen, (100, 100, 100), (center_x, center_y), orbit_radius, 1)

        # Draw spacecraft (simple dot for now)
        import math
        time = pygame.time.get_ticks() / 1000.0  # seconds
        angle = time * 0.5  # slow orbit
        ship_x = center_x + orbit_radius * math.cos(angle)
        ship_y = center_y + orbit_radius * math.sin(angle)
        pygame.draw.circle(screen, (255, 255, 0), (int(ship_x), int(ship_y)), 5)

        # Update the display
        pygame.display.flip()

        # Cap the frame rate
        clock.tick(60)

    # Clean exit
    pygame.quit()
    sys.exit()

if __name__ == "__main__":
    main()

# Compliance Note
# This project is based entirely on publicly available academic information and general knowledge of orbital mechanics. It contains no restricted, proprietary, or export-controlled information of any kind. This is a personal learning project only.
