from VisualObject import CVisualObject
from lib.Math2D import TransformMatrix, Path

class CSvg(CVisualObject):
    def __init__(self, width, height, scale="1"):
        if scale[-1] == '%':
            self.scale = float(scale[:-1])/100
        else:
            self.scale = float(scale)
        self.width = int(width)
        self.height = int(height)
        self.svg = []
    
    def __parsestyle(self, style):
        ret = {}
        style = style.split(';')
        for line in style:
            name, val = line.split(':')
            name = name.strip()
            val = val.strip()
            if val.lower() == 'none':
                val = None
            ret[name] = val
        return ret
    
    def __parsetransform(self, transform):
        name, attr = transform.split('(')
        attr = [float(i) for i in attr.split(')')[0].split(',')]
        if name == 'translate':
            return TransformMatrix.mk_translation(attr)
        if name == 'matrix':
            return TransformMatrix.mk_matrix(attr)
    
    def __getattrs(self, node):
        ret = {}
        for attr in xrange(node.attributes.length):
            attr = node.attributes.item(attr)
            ret[attr.name] = node.getAttribute(attr.name)
        return ret
    
    def LoadXml(self, element):
        def recursive(parent, transform):
            for node in parent.childNodes:
                if node.nodeType not in (node.ELEMENT_NODE, node.DOCUMENT_NODE):
                    continue
                attrs = self.__getattrs(node)
                if 'transform' in attrs:
                    transform = transform*self.__parsetransform(attrs['transform'])
                if node.tagName == 'path':
                    self.svg.append({'style': self.__parsestyle(attrs.get('style', {})),
                                     'path': transform*Path(attrs.get('d', ''))})
                elif node.tagName == 'g':
                    recursive(node, transform)
        
        recursive(element, TransformMatrix.mk_scale(self.scale))
    
    def GetSize(self, canvas, element):
        return self.width * self.scale, self.height * self.scale
    
    def PaintShadow(self, canvas, pos, element, color, size = (None, None)):
        trans = TransformMatrix.mk_translation(pos)
        for path in self.svg:
            if path['style'].get('fill', None) is None:
                bgcolor = None
            else:
                bgcolor = color
            canvas.DrawPath(trans*path['path'], color, bgcolor, int(float(path['style'].get('stroke-width', '1').rstrip('px'))+0.5))
    
    def Paint(self, canvas, pos, element, size = (None, None)):
        trans = TransformMatrix.mk_translation(pos)
        for path in self.svg:
            canvas.DrawPath(trans*path['path'], path['style'].get('stroke', 'black'), path['style'].get('fill', None), int(float(path['style'].get('stroke-width', '1').rstrip('px'))+0.5))
