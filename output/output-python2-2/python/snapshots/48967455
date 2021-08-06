# -*- coding: utf8 -*-

import sys
import pygame
from pygame.locals import *
import Tools

class Gui:
  '''
  Screen graphics and sounds are embedded in this class
  '''
  def __init__(self,appdir): 
    resolution=width,height=1024,768

    pygame.init()
#    pygame.mouse.set_visible(0)
    self.screen=pygame.display.set_mode(resolution,pygame.FULLSCREEN)
#    self.screen=pygame.display.set_mode(resolution)
  
    self.snd_startup=Tools.load_sound(appdir,"startup.ogg")
    self.snd_right=Tools.load_sound(appdir,"ok.ogg")
    self.snd_wrong=Tools.load_sound(appdir,"risa.ogg")
    self.snd_click=Tools.load_sound(appdir,"click.ogg")

    self.fnt_title = pygame.font.Font(None, 80)
    self.fnt_question = pygame.font.Font(None, 45)
    self.fnt_answer = pygame.font.Font(None, 35)
    self.fnt_score = pygame.font.Font(None, 32)

    self.color1=(50,50,140)
    self.color2=(255,192,22)
    self.color3=(250,250,255)
    self.color_right=(10,220,10)
    self.color_wrong=(220,10,10)

    self.background,bg_rect = Tools.load_image(appdir,"background.jpg")
    self.scoreboard_area=bg_rect
    self.scoreboard_area.top=700
      
    self.rect_a1=Rect(30,200,600,125)
    self.rect_a2=Rect(30,350,600,125)
    self.rect_a3=Rect(30,500,600,125)



# SHOW INTRO
  def show_intro(self):

    self.snd_startup.play()
    self.screen.blit(self.background, (0, 0))
    pygame.display.flip()
    pygame.time.delay(3500)
    


# SHOW QUESTION
  def show_question(self,q,a1,a2,a3): 

    self.screen.blit(self.background,(0,0),(0,0,1024,700))
    pygame.display.flip()

    text = self.fnt_question.render(q, 1, self.color1)
    textpos = text.get_rect(left=50,top=100)
    self.screen.blit(text, textpos)

    text = self.fnt_answer.render(a1, 1, self.color2)
    self.screen.fill(self.color3, self.rect_a1)
    self.screen.blit(text, self.rect_a1.move(10,50))

    text = self.fnt_answer.render(a2, 1, self.color2)
    self.screen.fill(self.color3, self.rect_a2)
    self.screen.blit(text, self.rect_a2.move(10,50))

    text = self.fnt_answer.render(a3, 1, self.color2)
    self.screen.fill(self.color3, self.rect_a3)
    self.screen.blit(text, self.rect_a3.move(10,50))

    pygame.display.flip()
    
 
# WAIT FOR ANSWERS 
  def wait_for_answers(self):
    choice=""
    pygame.event.clear()
    while 1:
      event=pygame.event.poll()
      buttons=pygame.mouse.get_pressed()
      if buttons[0]:
         self.snd_click.play()
         pos=pygame.mouse.get_pos()
         if self.rect_a1.collidepoint(pos):
            self.screen.blit(self.background, self.rect_a2,self.rect_a2)
            self.screen.blit(self.background, self.rect_a3,self.rect_a3)
            choice="A"
         if self.rect_a2.collidepoint(pos):
            self.screen.blit(self.background, self.rect_a1,self.rect_a1)
            self.screen.blit(self.background, self.rect_a3,self.rect_a3)
            choice="B"
         if self.rect_a3.collidepoint(pos):
            self.screen.blit(self.background, self.rect_a2,self.rect_a2)
            self.screen.blit(self.background, self.rect_a1,self.rect_a1)
            choice="C"
         if choice != "":
            pygame.display.flip()
            return choice

      if buttons[1]:
         sys.exit("Bye!")

    
   
# SHOW RESULT   
  def show_result(self,show,answer,ok):

    pygame.time.delay(2000)

    self.screen.blit(self.background, self.rect_a1,self.rect_a1)
    self.screen.blit(self.background, self.rect_a2,self.rect_a2)
    self.screen.blit(self.background, self.rect_a3,self.rect_a3)

    if answer == ok:
       color=self.color_right
       self.snd_right.play()
    else:
       color=self.color_wrong
       self.snd_wrong.play()

    text = self.fnt_answer.render(show, 1, self.color2)
    if answer == "A":
       self.screen.fill(color, self.rect_a1)
       self.screen.blit(text, self.rect_a1.move(10,50))
    if answer == "B":
       self.screen.fill(color, self.rect_a2)
       self.screen.blit(text, self.rect_a2.move(10,50))
    if answer == "C":
       self.screen.fill(color, self.rect_a3)
       self.screen.blit(text, self.rect_a3.move(10,50))

    pygame.display.flip()
       
    pygame.time.delay(3500)
        
