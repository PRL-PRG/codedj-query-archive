import sys
import traceback

import gtk
import pango

class DebugAttribute(object):
    def __init__(self, name, val = None):
        self.val = val
        self.name = name
    
    def __set__(self, instance, value):
        self.val = value
        f = sys._getframe(1)
        print '========================'
        print 'Subor: "%s", riadok: %d'%(f.f_code.co_filename, f.f_lineno)
        print 'Set %s.%s to %r'%(instance.__class__.__name__, self.name, value)
        print '========================'
    
    def __get__(self, instance, owner):
        f = sys._getframe(1)
        print '========================'
        print 'Subor: "%s", riadok: %d'%(f.f_code.co_filename, f.f_lineno)
        print 'Get %s.%s (%r)'%(instance.__class__.__name__, self.name, self.val)
        print '========================'
        return self.val

def display_exc():
    win = gtk.Window()
    win.set_title('Exception')
    text = gtk.TextView()
    text.set_editable(False)
    buffer = text.get_buffer()
    text.show()
    win.add(text)
    win.show()
    
    buffer.create_tag("italic", style=pango.STYLE_ITALIC)
    buffer.create_tag("bold", weight=pango.WEIGHT_BOLD)
    buffer.create_tag("monospace", family="monospace")
    
    iter = buffer.get_iter_at_offset(0)
    
    buffer.insert_with_tags_by_name(iter, "Traceback (most recent call last):\n", "bold")
    
    exccls, exc, tb = sys.exc_info()
    for filename, line, function, text in traceback.extract_tb(tb)[1:]:
        buffer.insert(iter, "\tFile \"")
        buffer.insert_with_tags_by_name(iter, filename, "bold")
        buffer.insert(iter, "\", line ")
        buffer.insert_with_tags_by_name(iter, str(line), "bold")
        if function is not None and function != '?':
            buffer.insert(iter, ", in ")
            buffer.insert_with_tags_by_name(iter, str(function), "bold")
        buffer.insert(iter, "\n")
        if text is not None:
            buffer.insert(iter, "\t\t")
            buffer.insert_with_tags_by_name(iter, str(text), "monospace")
            buffer.insert(iter, "\n")
    
    buffer.insert(iter, "\n")
    buffer.insert_with_tags_by_name(iter, exccls.__name__, "bold")
    buffer.insert(iter, ": %s\n"%exc)
