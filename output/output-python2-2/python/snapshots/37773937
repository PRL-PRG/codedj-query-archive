from VisualObject import CVisualObject
from lib.transform_matrix import TransformMatrix, PointMatrix

class CSvgXML:
    def __init__(self, element):
        self.root = []
        self.__load(self.root, element)
    
    def __popsur(self, arr):
        tmp = arr.pop(0).split(',')
        return PointMatrix.mk_xy((float(tmp[0]), float(tmp[1])))
    
    def __parsepath(self, path):
        ret = []
        path = path.split()
        while len(path) > 0:
            cmd = path.pop(0)
            if cmd == 'M':
                ret.append(('move', self.__popsur(path)))
            elif cmd == 'C':
                ret.append(('bezier', self.__popsur(path), self.__popsur(path), self.__popsur(path)))
            elif cmd == 'L':
                ret.append(('line', self.__popsur(path)))
        if ret[0][0] != 'move':
            raise Exception, "First command of path in svg must be move (M)"
        return ret
    
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
    
    def __load(self, root, element):
        for chld in element.childNodes:
            if chld.nodeType not in (chld.ELEMENT_NODE, chld.DOCUMENT_NODE):
                continue
            elem = {'name': chld.tagName, 'childs': [], 'attrs': {}}
            root.append(elem)
            for attr in xrange(chld.attributes.length):
                attr = chld.attributes.item(attr)
                if attr.name == 'd':
                    elem['attrs']['d'] = self.__parsepath(chld.getAttribute(attr.name))
                elif attr.name == 'style':
                    elem['attrs']['style'] = self.__parsestyle(chld.getAttribute(attr.name))
                elif attr.name == 'transform':
                    elem['attrs']['transform'] = self.__parsetransform(chld.getAttribute(attr.name))
                else:
                    elem['attrs'][attr.name] = chld.getAttribute(attr.name)
            self.__load(elem['childs'], chld)
    
    def Paint(self, canvas, pos = (0, 0), scale = 1):
        self.PaintChilds(canvas, self.root, TransformMatrix.mk_translation(pos) * TransformMatrix.mk_scale(scale))
    
    def PaintChilds(self, canvas, node, transform):
        for i in node:
            fnc = "PaintSvg%s"%i['name'].capitalize()
            if not hasattr(self, fnc):
                raise Exception, "I don't know how to draw %s element from svg."%i['name']
            fnc = getattr(self, fnc)
            fnc(canvas, i, transform)
    
    def PaintSvgPath(self, canvas, node, transform):
        st = node['attrs'].get('style', {})
        if 'transform' in node['attrs']:
            transform = node['attrs']['transform'] * transform
        for cmd in node['attrs']['d']:
            if cmd[0] == 'move':
                old = (transform * cmd[1]).GetIntPos()
            elif cmd[0] == 'bezier':
                new = (transform * cmd[3]).GetIntPos()
                canvas.DrawBezier(old, (transform * cmd[1]).GetIntPos(),
                                       (transform * cmd[2]).GetIntPos(),
                                       new, fg = st.get('stroke', None))
                old = new
            elif cmd[0] == 'line':
                new = (transform * cmd[1]).GetIntPos()
                canvas.DrawLine(old, new, fg = st.get('stroke', None))
                old = new
    
    def PaintSvgG(self, canvas, node, transform):
        if 'transform' in node['attrs']:
            transform = node['attrs']['transform'] * transform
        self.PaintChilds(canvas, node['childs'], transform)

class CSvg(CVisualObject):
    def __init__(self, width, height, scale="1"):
        if scale[-1] == '%':
            self.scale = float(scale[:-1])/100
        else:
            self.scale = float(scale)
        self.width = int(width)
        self.height = int(height)
    
    def LoadXml(self, element):
        self.svg = CSvgXML(element)
    
    def GetWidth(self, canvas, element):
        return self.width * self.scale
    
    def GetHeight(self, canvas, element):
        return self.height * self.scale
    
    def Paint(self, canvas, pos, element, size = (None, None)):
        self.svg.Paint(canvas, pos, self.scale)
