from __future__ import division

import random

_COLORS = ['#0099cc', '#006699']

def main():
    masks = []
    squares = []
    size = 128
    squarewidth = 10
    for x in range(0, size + 1, squarewidth):
        for y in range(0, size + 1, squarewidth):
            color = _COLORS[((x + y) // squarewidth) % 2]
            id = 990000 + (100 * x) + y
            d = (4 * random.randint(0, size - x) // size) ** 2
            dx = random.randint(-d, d)
            dy = random.randint(-d, d)

            masks.append('    <mask id="mask%i">'
                         '<circle cx="%i" cy="%i" r="%i" fill="#ffffff"/>'
                         '</mask>' % (id, size // 2 + dx, size // 2 + dy, size // (2 / 0.8)))

            squares.append('    <rect style="fill:%s" id="rect%i" width="%i" '
                           'height="%i" x="%i" y="%i" mask="url(#mask%s)" />'
                           % (color, id, squarewidth, squarewidth,
                              x + dx, y - 1 + dy, id))

    print('<?xml version="1.0" encoding="UTF-8" standalone="no"?>')
    print('<svg width="%ipx" height="%ipx">' % (size, size))
    print('  <defs>')
    print('\n'.join(masks))
    print('  </defs>')
    print('  <g>')
    print('\n'.join(squares))
    print('  </g>')
    print('</svg>')

if __name__ == '__main__':
    main()