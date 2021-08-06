"""
Hello world in Nevow
"""

from nevow import rend, loaders

template ="""
<html xmlns:nevow="http://nevow.com/ns/nevow/0.1">
Hello, world!
</html>
"""

class HelloWorld(rend.Page):
    addSlash = True
    docFactory = loaders.xmlstr(template)
