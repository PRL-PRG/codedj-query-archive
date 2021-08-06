"""
Template example: conditional data
"""
from nevow import rend, loaders, inevow

template ="""
<html xmlns:nevow="http://nevow.com/ns/nevow/0.1">
<p>Hello, <em nevow:data="name" nevow:render="data">Dude</em></p>
<div id="definition" nevow:render="definition">
<p nevow:pattern="twisted_definition"><strong>Twisted</strong> 
is an event-driven networking framework written in Python</p>

<p nevow:pattern="nevow_definition"><strong>Nevow</strong> 
is a web application construction kit written in Python.</p>
</div>
</html>
"""

class Root(rend.Page):
    addSlash = True
    docFactory = loaders.xmlstr(template)
    
    counter = 1
    
    def data_name(self, context, data):
        self.counter += 1
        if divmod(self.counter, 2)[1] == 0:
            return "Nevow newbie"
        else:
            return "Twisted hacker"
    
    def render_definition(self, context, data):
        query = inevow.IQ(context)
        twisted_pattern = query.onePattern('twisted_definition')
        nevow_pattern = query.onePattern('nevow_definition')
        
        if divmod(self.counter, 2)[1] == 0:
            return context.tag[nevow_pattern]
        else:
            return twisted_pattern
        
    