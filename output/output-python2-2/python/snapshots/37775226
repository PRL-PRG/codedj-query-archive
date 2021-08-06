from VisualObject import CVisualObject
import sys

class CIcon(CVisualObject):
    def __init__(self, filename):
        CVisualObject.__init__(self)
        self.filename = filename

    def ComputeSize(self, context):
        filename, = self.GetVariables(context, 'filename')
        return context.GetCanvas().GetIconSize(filename)

    def Paint(self, context, shadow = False):
        filename, = self.GetVariables(context, 'filename')
        
        context.GetCanvas().DrawIcon(context.GetPos(), filename)
