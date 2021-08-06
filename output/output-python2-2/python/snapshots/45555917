"""
Formal example
"""

from nevow import rend, loaders, inevow
import formal

template ="""
<html xmlns:nevow="http://nevow.com/ns/nevow/0.1">
<link rel="stylesheet" type="text/css" href="formalcss" />
<p nevow:render="form simple" />
</html>
"""

class Root(formal.ResourceMixin, rend.Page):
    addSlash = True
    docFactory = loaders.xmlstr(template)
    
    child_formalcss = formal.defaultCSS
    
    counter = 0

    def form_simple(self, context):
        self.counter += 1
        form = formal.Form()
        form.addField("number", formal.Integer(required=True))
        form.addField("name", formal.String(missing="absence"))
        form.addField("id", formal.Integer(immutable=True))
        form.addAction(self.action)
        form.data = {'id': self.counter}
        print dir(form)
        return form
    
    def action(self, context, form, data):
        return "You've entered %r" % data
