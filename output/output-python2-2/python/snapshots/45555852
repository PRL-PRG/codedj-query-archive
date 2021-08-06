"""
Template example: insert some data into template
"""
from nevow import rend, loaders

from datetime import datetime
from nevow._version import version as nevow_version
from twisted._version import version as twisted_version

template ="""
<html xmlns:nevow="http://nevow.com/ns/nevow/0.1">
<p>Hello, <em nevow:data="name" nevow:render="data">Dude</em></p>

<p>Current time is: <em nevow:data="time" nevow:render="string">2006-12-10 14:49:52</em></p>

<p nevow:render="fillSlots">
This page running by Twisted <nevow:slot name="twisted" /> and Nevow <nevow:slot name="nevow" />
</p>
</html>
"""

class Root(rend.Page):
    addSlash = True
    docFactory = loaders.xmlstr(template)
    
    data_name="Nevow newbie"
    
    def data_time(self, context, data):
        return datetime.now()
    
    def render_fillSlots(self, context, data):
        context.tag.fillSlots('twisted', twisted_version.short())
        context.tag.fillSlots('nevow', nevow_version.short())
        return context.tag
    
    