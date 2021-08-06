"""
Object traversal with childFactory method (see Root)
"""

from nevow import rend, loaders

template_one ="""
<html xmlns:nevow="http://nevow.com/ns/nevow/0.1">
The page one.
</html>
"""

template_two ="""
<html xmlns:nevow="http://nevow.com/ns/nevow/0.1">
The page two.
</html>
"""

template_three ="""
<html xmlns:nevow="http://nevow.com/ns/nevow/0.1">
The page three.
</html>
"""

template_root ="""
<html xmlns:nevow="http://nevow.com/ns/nevow/0.1">
<ul>
<li><a href="/one/">The page one</a></li>
<li><a href="/two/">The page two</a></li>
<li><a href="/three/">The page three</a></li>
</ul>
</html>
"""


class PageOne(rend.Page):
    addSlash = True
    docFactory = loaders.xmlstr(template_one)

class PageTwo(rend.Page):
    addSlash = True
    docFactory = loaders.xmlstr(template_two)

class PageThree(rend.Page):
    addSlash = True
    docFactory = loaders.xmlstr(template_three)

class Root(rend.Page):
    addSlash = True
    docFactory = loaders.xmlstr(template_root)
    
    def childFactory(self, context, name):
        choices = {
            'one': PageOne(),
            'two': PageTwo(),
            'three': PageThree(),
        }
        
        return choices.get(name)