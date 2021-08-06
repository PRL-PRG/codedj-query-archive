import Acquisition, time
from plone.memoize import ram
from Products.Five.browser import BrowserView
from zope.interface import implements, alsoProvides

from Products.Five.browser.pagetemplatefile import ViewPageTemplateFile
from Products.CMFCore.utils import getToolByName
from osha.whoswho.interfaces import IWhosWhoView
from plone.app.layout.globals.interfaces import IViewView

class WhoswhoView(BrowserView):
    """View for displaying WhosWho Items by alphabet
    """
    implements(IWhosWhoView, IViewView)
    template = ViewPageTemplateFile('templates/whoswho_view.pt')


    def __call__(self):
        context = Acquisition.aq_inner(self.context)
        return self.template() 