#
# This software is licensed under the MIT License
#
# The MIT License
# 
# Copyright (c) 2007 Siddharta Govindaraj. All rights reserved.
# 
# Permission is hereby granted, free of charge, to any person obtaining a copy
# of this software and associated documentation files (the "Software"), to deal
# in the Software without restriction, including without limitation the rights
# to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
# copies of the Software, and to permit persons to whom the Software is
# furnished to do so, subject to the following conditions:
# 
# The above copyright notice and this permission notice shall be included in
# all copies or substantial portions of the Software.
# 
# THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
# IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
# FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
# AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
# LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
# OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN
# THE SOFTWARE.
# 

#!/usr/bin/python

import os
import Image, ImageDraw, ImageFont

class BadgeImage(object):
    def __init__(self, filename):
        self.img = Image.open(filename)
        self.draw = ImageDraw.Draw(self.img)
        self.width = int(self.img.size[0]*0.9)

    def drawAlignedText(self, pos, text, (font, color), xtransform, ytransform):
        width,height = font.getsize(text)
        xpos = xtransform(pos[0], width)
        ypos = ytransform(pos[1], height)
        self.draw.text((xpos, ypos), text, fill=color, font=font)

    def drawCenteredText(self, pos, text, font):
        self.drawAlignedText(pos, text, font, lambda x,w:x-w/2, lambda y,h:y-h/2)

    def getFitSize(self, startsize, text):
        size = startsize
        font = ImageFont.truetype("Trebucbd.ttf", size*300/72)
        textwidth, textheight = font.getsize(text)
        while textwidth > self.width:
            size -= 1
            font = ImageFont.truetype("Trebucbd.ttf", size*300/72)
            textwidth, textheight = font.getsize(text)
        return size

    def drawPerson(self, name):
        linepos = (self.img.size[0]/2, 240)
        line1pos = (self.img.size[0]/2, 150)
        line2pos = (self.img.size[0]/2, 320)
        size = self.getFitSize(45, name)
        if name.find(" ") >= 0:
            firstname, rest = name.split(" ", 1)
        else:
            firstname, rest = (name, "")
        if size < 45 and rest != "":
            personFont = ImageFont.truetype("Trebucbd.ttf", self.getFitSize(45, firstname)*300/72)
            self.drawCenteredText(line1pos, firstname, (personFont, "#ffffff"))
            personFont = ImageFont.truetype("Trebucbd.ttf", self.getFitSize(45, rest)*300/72)
            self.drawCenteredText(line2pos, rest, (personFont, "#ffffff"))
        else:
            personFont = ImageFont.truetype("Trebucbd.ttf", self.getFitSize(45, name)*300/72)
            self.drawCenteredText(linepos, name, (personFont, "#ffffff"))

    def drawCompany(self, name):
        pos = (self.img.size[0]/2, 500)
        font = ImageFont.truetype("Trebucbd.ttf", self.getFitSize(26, name)*300/72)
        self.drawCenteredText(pos, name, (font, "#0099ff"))

    def save(self, filename, doubleSided=True):
        if not doubleSided:
            self.img.save(filename)
            return

        newimg = Image.new("RGB", (self.img.size[0]*2+20, self.img.size[1]), "#000000")
        newimg.paste(self.img, (0,0))
        newimg.paste(self.img, (self.img.size[0]+20,0))
        newimg.save(filename)

class DataFileReader(object):
    def __init__(self, filename):
        fp = open(filename)
        self.lines = [line[:-1] for line in fp.readlines()]
        fp.close()

    def getData(self):
        for id, line in enumerate(self.lines):
            if len(line.strip()) != 0:
                name,company = line.split("\t")
                name = name.title()
		if not company.startswith("*"):
                    company = company.title()
                else:
                    company = company[1:]
                yield (id, name.title(), company)

import sys

if len(sys.argv) > 1:
    filenames = sys.argv[1:]
else:
    filenames = ["people"]

count = 0
for filename in filenames:
    reader = DataFileReader(filename + ".csv")
    if not os.path.exists(filename):
	    os.makedirs(filename)
    for id, name, company in reader.getData():
        print id, name, company
        badge = BadgeImage("badge_template.png")
        badge.drawPerson(name)
        badge.drawCompany(company)
        badge.save(filename + "\\" + filename + "_badge_" + str(id) + ".png")
        count += 1
print "\n%d badges created" % (count)

