## some fig-drawing functions

import string
from math import floor,sqrt,exp,log
from sys import stderr

LONG_DIMENSION = 13200
SHORT_DIMENSION = 10200

class LINE:
    def __init__(self,start,stop):
        self.x0 = int(floor(start[0]+0.5))
        self.y0 = int(floor(start[1]+0.5))
        self.x1 = int(floor(stop[0]+0.5))
        self.y1 = int(floor(stop[1]+0.5))
        self.color = 0
        self.width = 1
        self.style = 0
        return

class ARROW:
    def __init__(self,start,stop):
        self.x0 = int(floor(start[0]+0.5))
        self.y0 = int(floor(start[1]+0.5))
        self.x1 = int(floor(stop[0]+0.5))
        self.y1 = int(floor(stop[1]+0.5))
        self.color = 0
        self.width = 1
        return

class BOX:
    def __init__(self,upper_left,lower_right):
        self.x0 = int(floor(upper_left[0]+0.5))
        self.y0 = int(floor(upper_left[1]+0.5))
        self.x1 = int(floor(lower_right[0]+0.5))
        self.y1 = int(floor(lower_right[1]+0.5))
        self.fill_color = 0
        self.line_color = 0
        self.line_width = 0
        self.fill_power = 20 ## runs from 0 to 20 (-1 is empty fill)
        return

def New_box(box):
    return '2 2 0 %d %d %d 50 0 %d 0.000 0 0 -1 0 0 5\n'\
           %(box.line_width, box.line_color, box.fill_color, box.fill_power)+\
           '\t%d %d %d %d %d %d %d %d %d %d\n'\
           %(box.x0,box.y0,
             box.x1,box.y0,
             box.x1,box.y1,
             box.x0,box.y1,
             box.x0,box.y0)


def Dist(v,w):
    return sqrt( (v[0]-w[0])**2 + (v[1]-w[1])**2)

def Header(orientation='L'):
    fig = '#FIG 3.2\n'
    if orientation in ['L','l','landscape','Landscape'] :
        fig = fig + 'Landscape\n'
    else: fig = fig + 'Portrait\n'
    return fig+'Center\nInches\nLetter  \n100.00\nSingle\n-2\n1200 2\n'


def Box(x_top_left,y_top_left,width,height,color):
    x = int(floor(0.5+x_top_left))
    y = int(floor(0.5+y_top_left))
    return string.join(map(str,['2 2 0 0',color,color,
                                '50 0 20 0.000 0 0 -1 0 0 5\n',
                                '\t',x,y,
                                width+x,y,
                                width+x,height+y,
                                x,height+y,
                                x,y]))+'\n'

def Marked_box(x_top_left,y_top_left,width,height,color,marked_color):
    x = int(floor(0.5+x_top_left))
    y = int(floor(0.5+y_top_left))
    return string.join(map(str,['2 2 0 1',marked_color,color,
                                '50 0 20 0.000 0 0 -1 0 0 5\n',
                                '\t',x,y,
                                width+x,y,
                                width+x,height+y,
                                x,height+y,
                                x,y]))+'\n'


def Line(l):
    return '2 1 %d %d %d 7 50 0 -1 0.000 0 0 -1 0 0 2\n\t%d %d %d %d\n' \
           %(l.style,int(floor(l.width)),l.color,l.x0,l.y0,l.x1,l.y1)


def Arrow(l):
    return '2 1 0 %d %d 7 50 0 -1 0.000 0 0 -1 1 0 2\n\t1 1 1.00 30.00 60.00\n%d %d %d %d\n'\
           %(l.color,l.width,l.x0,l.y0,l.x1,l.y1)

def Disk(center,radius):
    x = int(floor(0.5 + center[0]))
    y = int(floor(0.5 + center[1]))
    r = int(floor(0.5 + radius))

    return '1 4 0 1 0 0 50 0 20 0.000 1 0.0000 '+\
           string.join(map(str,[x,y,r,r,x-r,y,x+r,y]))+'\n'

def Text(text,point,font,justified='r'):

    x = int(floor(0.5+point[0]))
    y = int(floor(0.5+point[1]))
    return '4 %d 0 50 0 0 %d 0.0000 4 0 0 %d %d %s\\001\n' \
           %(2*(justified=='r'),font,x,y,text)

def FW_text( text, point, font, color ): ## fixed width
    x = int(floor(0.5+point[0]))
    y = int(floor(0.5+point[1]))
    return '4 0 %d 50 -1 5 %d 0.0000 0 120 525 %d %d %s\\001\n' \
           %(color,font,x,y,text)

def Color(fraction,LOG):
    if LOG:
        EXPONENT = log10(2)
    else:
        EXPONENT = 1.0

    if fraction < 0.00001:
        score = fraction
    else:
        score = exp( EXPONENT * log(fraction))

    color = 32 + int(floor(2*16*16*(score/1.00001)))
    return color

def Greyscale_colors():
    l = map(str,range(10))+['a','b','c','d','e','f']## hexadecimal

    fig = ''
    counter = 32
    for i in range(512):
        green = 255 - i/2
        blue = 255 - i/2
        red = 255 - i/2

        fig = fig + string.join(map(str,[0,counter,
                                         '#'+l[red/16]+l[red%16]+l[green/16]+\
                                         l[green%16]+l[blue/16]+l[blue%16]]))+'\n'
        counter = counter + 1

    return fig

def Rainbow_colors():
    l = map(str,range(10))+['a','b','c','d','e','f']## hexadecimal

    fig = ''
    counter = 32
    for i in range(256):
        #green = min(200,2*i)
        green = min(200,(200*i)/128)
        blue = min(255,510-2*i)
        red = 0

        fig = fig + string.join(map(str,[0,counter,
                                         '#'+l[red/16]+l[red%16]+l[green/16]+\
                                         l[green%16]+l[blue/16]+l[blue%16]]))+'\n'
        counter = counter + 1

    for i in range(256):
        red = min(255,2*i)
        green = max(0,min(200,400 - (200*i)/128))
        #green = min(200,510-2*i)
        blue = 0
        fig = fig + string.join(map(str,[0,counter,
                                         '#'+l[red/16]+l[red%16]+l[green/16]+\
                                         l[green%16]+l[blue/16]+l[blue%16]]))+'\n'
        counter = counter + 1
    return fig

def Greyscale_colors():
    l = map(str,range(10))+['a','b','c','d','e','f']## hexadecimal

    fig = ''
    counter = 32
    for i in range(512):
        red = 255 - i/2
        green = 255 - i/2
        blue = 255 - i/2
        fig = fig + string.join(map(str,[0,counter,
                       '#'+l[red/16]+l[red%16]+l[green/16]+\
                       l[green%16]+l[blue/16]+l[blue%16]]))+'\n'
        counter = counter + 1
    return fig


def Graph(vertices,edges,arrows,labels = [],BY_WIDTH=0):## arrows are directed from left to right

    X_DIMENSION = LONG_DIMENSION
    Y_DIMENSION = SHORT_DIMENSION

    ## do we have score information for the edges and arrows?
    scores = []
    for e in edges+arrows:
        if len(e) == 3:
            scores.append(float( e[2]) )
    if scores:
        mn = min(scores)
        mx = max(scores)
        if mx==mn: ## if they're all the same color, make them blue
            mx=mn+1
        stderr.write('edge scores: min=%f max=%f\n'%(mn,mx))

    points = vertices + map(lambda x:x[0],edges) + map(lambda x:x[1],edges) + \
             map(lambda x:x[0],arrows) + map(lambda x:x[1],arrows)

    min_x = float(min(map(lambda x:x[0],points)))
    max_x = float(max(map(lambda x:x[0],points)))
    min_y = float(min(map(lambda x:x[1],points)))
    max_y = float(max(map(lambda x:x[1],points)))

    def Transform(point,min_x=min_x,max_x=max_x,min_y=min_y,max_y=max_y,
                  X_DIMENSION=X_DIMENSION,Y_DIMENSION=Y_DIMENSION):
        x_margin = 1000
        y_margin = 1000
        new_x = int ( floor ( 0.5 + (X_DIMENSION - 2 * x_margin) * \
                              (point[0] - min_x) / (max_x - min_x))) + x_margin
        new_y = int ( floor ( 0.5 + (Y_DIMENSION - 2 * y_margin) * \
                              (max_y - point[1]) / (max_y - min_y))) + y_margin
        return [new_x,new_y]

    fig = ''
    min_d = 100000
    for e in edges:
        l = LINE(Transform(e[0]),Transform(e[1]))
        if len(e) == 3:
            if BY_WIDTH:
                l.width = e[2]
            else:
                l.color = Color( (float(e[2])-mn)/(mx-mn) ,0)
        fig = fig + Line (l)
        min_d = min(min_d,Dist(Transform(e[0]),Transform(e[1])))

    for e in arrows:
        l = ARROW(Transform(e[0]),Transform(e[1]))
        if len(e) == 3:
            if BY_WIDTH:
                l.width = e[2]
            else:
                l.color = Color( (float(e[2])-mn)/(mx-mn) ,0)
        fig = fig + Arrow (l)
        min_d = min(min_d,Dist(Transform(e[0]),Transform(e[1])))

    radius = max(38, min(1,int(floor(0.25*min_d))))

    for v in vertices:
        fig = fig + Disk(Transform(v),radius)

    l = []
    for a in labels:
        if len(a)==2 or a[2] == 'r': ## only use right-justified labels
            l.append( [Transform(a[1])[1], Transform(a[1])[0] ]  )
    l.sort()
    min_sep = 1000
    nearby = 200
    for i in range(len(l)):
        for j in range(i+1,len(l)):
            if l[j][0] - l[i][0] > min_sep: break
            if abs ( l[j][1] - l[i][1] ) <nearby:
                min_sep = l[j][0]-l[i][0]

    font = min( 18, max( 5, min_sep/20))

    for l in labels:
        if len(l)==2:
            justified = 'r'
        else:
            justified = l[2]
        fig = fig + Text( l[0], Transform(l[1]), font, justified)
    return fig

def Read_graph(file):
    vertices = []
    edges = []
    arrows = []
    lines = map(string.split,open(file,'r').readlines())
    for line in lines:
        if line[0] != 'FIGPLOT':continue
        if line[1] == 'VERTEX':
            vertices.append([float(line[2]),float(line[3])])
        elif line[1] == 'ARROW':
            arrows.append( [ [float(line[2]),float(line[3])] , [float(line[4]),float(line[5])] ])
        elif line[1] == 'EDGE':
            edges.append( [ [float(line[2]),float(line[3])] , [float(line[4]),float(line[5])] ])
    return vertices,edges,arrows


if 0:
    print Header()
    vertices,edges,arrows = Read_graph('../146A.1.figplot')
    print Graph(vertices,edges,arrows)
