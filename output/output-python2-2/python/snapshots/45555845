"""
Template example: insert some fragment into template
"""
from nevow import rend, loaders


main_template ="""
<html xmlns:nevow="http://nevow.com/ns/nevow/0.1">
<p nevow:render="fillAllSlots">
Some text before first (inserted by nevow:slot tag) fragment. 
<em><nevow:slot name="content_fragment" /></em>
Some text after first fragment.
</p>
<p>
Some text before second (inserted by nevow:render attribute) fragment.
<em nevow:render="fragment" />
Some text after second fragment.
</p>
</html>
"""

fragment_template="""
<p xmlns:nevow="http://nevow.com/ns/nevow/0.1">
Fragment may use own data and renderers, for example this is the counter: 
<strong nevow:data="counter" nevow:render="data">counter here</strong>
</p>
"""

class Fragment(rend.Fragment):
    docFactory = loaders.xmlstr(fragment_template)
    
    def __init__(self, counter_step=1):
        self.counter = 0
        self.counter_step = counter_step
        super(Fragment, self).__init__()
    
    def data_counter(self, context, data):
        self.counter += self.counter_step
        return "%.2f" % self.counter

class Root(rend.Page):
    addSlash = True
    docFactory = loaders.xmlstr(main_template)
    fragment_one = Fragment(0.346)
    fragment_two = Fragment(0.724)
    
    def render_fillAllSlots(self, context, data):
        context.tag.fillSlots('content_fragment', self.fragment_one)
        return context.tag
    
    def render_fragment(self, context, data):
        return context.tag[self.fragment_two]
