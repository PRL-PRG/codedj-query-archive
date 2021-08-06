"""
Pages of nevowex app
"""
import os
import config
from nevow import rend, loaders, static, inevow
import formal

from pseudomodels import Data

def template(templatename, pattern=None):
    """
    Load XML template
    """
    return loaders.xmlfile(templatename, 
                           templateDir=config.TEMPLATE_PATH,
                           pattern=pattern)

class BasePage(rend.Page):
    """
    Base for all pages
    """
    addSlash = True
    docFactory = template('index.html')

class ContentPage(BasePage):
    """
    Base for all pages, which change only content
    """
    def __init__(self, title, contentFragment):
        """
        Make ContentPage with such title and such content fragment
        """
        self.title = title
        self.contentFragment = contentFragment
    
    def render_content(self, context, data):
        """
        Renderer of content
        """
        return context.tag.clear()[self.contentFragment]
    
    def data_title(self, context, data):
        """
        Title's data
        """
        return self.title

class DynamicFragment(rend.Fragment):
    """
    Fragment which renders template and fill it by some synamic data
    """
    docFactory = template('frg_dynamic.html')
    
    def data_dyndata(self, context, data):
        """
        Dyndata's data
        """
        return Data # Data from pseudomodels

class CodeFragment(rend.Fragment):
    """
    Fragment which show code of our application
    """
    docFactory = template('frg_code.html')
    
    def data_pyfiles(self, context, data):
        """
        Pyfiles' data - sequence, each item is a dict with name and contents
        """
        pyfiles = []
        for dir_name, dir_list, file_list in os.walk(config.CODE_PATH):
            pyfiles += [{
                        'name': os.path.join(dir_name, f), 
                        'contents': open(os.path.join(dir_name, f)).read()
                        }
                        for f in file_list if f.endswith('.py')]
        return pyfiles

class FormalHack(object):
    """
    Hack for formal.
    
    Formal's form renders in fragment, but processes and validates in page,
    so put common methods (form_ and it action) into separate class.
    """

    def form_showName(self, context):
        """
        Represent a form to render
        """
        form = formal.Form()
        form.addField('name', formal.String(required=True))
        form.addAction(self.action_showName)
        return form
    
    def action_showName(self, context, form, data):
        """
        Action for form
        """
        # just silly save name in session
        session = inevow.ISession(context)
        session.name = data['name']

class FormFragment(FormalHack, formal.ResourceMixin, rend.Fragment):
    """
    Fragment, which renders form.

    You may expect what this fragment also validate or process form,
    but it's not true - the page do it, not fragment.
    """
    docFactory = template('frg_forms.html')
    
    def render_greeting(self, context, data):
        """
        Render greeting
        """
        # retrieve name from session
        session = inevow.ISession(context)
        name = getattr(session, 'name', None)
        if name is not None:
            # also clears the name in session, because we don't want ot remember it
            session.name = None
            return context.tag[name]
        else:
            # if name is None (i.e. form not valid, or not submitted)
            # do nothing
            return ''

class FormPage(FormalHack, formal.ResourceMixin, BasePage):
    """
    Page include the FormFragment, validate and process form.
    
    You may expect what this form also rendered by page, but it's
    not true - the fragment do it, not page.
    """
    
    data_title = 'Forms.'  # data for title
    
    fragment = FormFragment()
    
    def render_content(self, context, data):
        """
        Renders content with FormFragment
        """
        return context.tag.clear()[self.fragment]

class Root(BasePage):
    """
    Root page
    """

    child_static = static.File(config.STATIC_PATH)              # static data
    child_forms = FormPage()                                    # form example
    child_selfcode = ContentPage('Selfcode.', CodeFragment())   # selfcode
    child_dynamic = ContentPage('Dynamic.', DynamicFragment())  # dynamic page
    
    data_title = ''
    
    def render_content(self, context, data):
        """
        Render content
        """
        # do nothing, because we at root
        return context.tag
