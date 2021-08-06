"""
Template example: sequence renderer
"""

from nevow import rend, loaders

template ="""
<html xmlns:nevow="http://nevow.com/ns/nevow/0.1">
<ul nevow:data="fruits" nevow:render="sequence">
<p nevow:pattern="empty">There is no fruits</p>
<p nevow:pattern="header">There is some fruits:</p>
<li nevow:pattern="item" nevow:render="data" class="odd">Some fruit here</li>
<li nevow:pattern="item" nevow:render="data" class="even">Another fruit here</li>
<p nevow:pattern="footer">...and nothing more</p>
</ul>
</html>
"""

class Root(rend.Page):
    addSlash = True
    docFactory = loaders.xmlstr(template)
    
    counter = 1
    
    def data_fruits(self, context, data):
        self.counter += 1
        if divmod(self.counter, 2)[1] == 0:
            return ('apple', 'orange', 'pear', 'apricot')
        else:
            return ()

