from PIL import Image, ImageFont
import pygame

pygame.init()

FPS = 20

FONTS = {
    'GameOver': ImageFont.truetype("./fonts/GameCube.ttf", 16),
    'Tiny': ImageFont.load('fonts/4x6.pil'),
    'Small': ImageFont.load('fonts/5x7.pil'),
    'Medium': ImageFont.load('fonts/6x10.pil'),
    'Digital14': ImageFont.load('fonts/digital-14.pil'),
    'Digital16': ImageFont.load('fonts/digital-16.pil')
}

IMAGES = {
    'MainLogo' : Image.open('imgs/combined-logo.png')
}

BUTTON = { 
    'B1000L': 0x0100,
    'B1000R': 0x0200,
    'B500': 0x0400,
    'B400': 0x0800,
    'B300': 0x1000,
    'B200': 0x2000,
    'B100': 0x4000,
    'BRET': 0x0080, 
    'SELECT': 0x0004,
    'START': 0x0002,
    'ANYBUTTON': 0x0006,
    'CONFIG': 0x0008,
    'SCORED': 0x7F00,
    'ANY': 0xFFFF
}

SOUNDS = {
    'START': pygame.mixer.Sound("sounds/great_balls_of_fire.ogg"),
    'JINGLE': pygame.mixer.Sound("sounds/skeeball_jingle.ogg"),
    'OVER9000': pygame.mixer.Sound("sounds/its_over_9000.ogg"),
    'PLACE1': pygame.mixer.Sound("sounds/place_1.ogg"),
    'PLACE2': pygame.mixer.Sound("sounds/place_2.ogg"),
    'PLACE3': pygame.mixer.Sound("sounds/place_3.ogg"),
    'PLACE4': pygame.mixer.Sound("sounds/place_4.ogg"),
    'PLACE5': pygame.mixer.Sound("sounds/place_5.ogg"),
}


BALL_COLORS = [
    (255,0,0),
    (255,0,0),
    (255,255,0),
    (255,255,0),
    (0,255,0),
    (0,255,0),
    (0,255,0),
    (0,255,0),
    (0,255,0),
    (0,255,0),
]