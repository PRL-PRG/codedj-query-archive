"""Definition of the whoswho content type
"""
from Products.Archetypes.atapi import *

try:
    from Products.LinguaPlone.public import *
except ImportError:
    HAS_LINGUAPLONE = False
else:
    HAS_LINGUAPLONE = True

from zope.interface import implements, directlyProvides

from Products.ATContentTypes.content import base
from Products.ATContentTypes.content import schemata
from Products.ATContentTypes.content.document import ATDocument

from osha.whoswho import whoswhoMessageFactory as _
from osha.whoswho.interfaces import Iwhoswho
from osha.whoswho.config import PROJECTNAME


schema = Schema((

    StringField(
        name='url',
        widget=StringField._properties['widget'](
            label=_(u'label_url', default=u'URL'),
            description=u'',
        ),
        required=False,
        schemata="default",
        searchable=True,
    ),
    StringField(
        name='email',
        widget=StringField._properties['widget'](
            label=_(u'label_email', default=u'E-mail'),
            description=u'',
        ),
        required=False,
        schemata="default",
        searchable=True,
    ),
    StringField(
        name='tel',
        widget=StringField._properties['widget'](
            label=_(u'label_tel', default=u'Telephone'),
            description=u'',
        ),
        required=False,
        schemata="default",
        searchable=False,
    ),
    StringField(
        name='fax',
        widget=StringField._properties['widget'](
            label=_(u'label_fax', default=u'Fax'),
            description=u'',
        ),
        required=False,
        schemata="default",
        searchable=False,
    ),
    TextField(
        name='address',
        widget=TextAreaWidget(
            label=_(u'label_address', default=u'Address'),
            description=u'',
        ),
        required=False,
        schemata="default",
        searchable=True,
    ),
    TextField(
        name='targets',
        widget=TextAreaWidget(
            label=_(u'label_targets', default=u'Targets'),
            description=u'',
        ),
        required=False,
        schemata="default",
        searchable=True,
    ),
    TextField(
        name='activities',
        widget=TextAreaWidget(
            label=_(u'label_activities', default=u'Activities'),
            description=u'',
        ),
        required=False,
        schemata="default",
        searchable=True,
    ),
    StringField(
        name='sponsorUrl',
        widget=StringField._properties['widget'](
            label=_(u'label_sponsorUrl', default=u'Sponsor URL'),
            description=u'',
        ),
        required=False,
        schemata="default",
        searchable=False,
    ),
    TextField(
        name='sponsorName',
        widget=TextAreaWidget(
            label=_(u'label_sponsorName', default=u'Sponsor name'),
            description=u'',
        ),
        required=False,
        schemata="default",
        searchable=True,
    ),
    StringField(
        name='relatedOrgUrl',
        widget=StringField._properties['widget'](
            label=_(u'label_relatedOrgUrl', default=u'Related organisation URL'),
            description=u'',
        ),
        required=False,
        schemata="default",
        searchable=False,
    ),
    TextField(
        name='relatedOrgName',
        widget=TextAreaWidget(
            label=_(u'label_relatedOrgName', default=u'Related organisation name'),
            description=u'',
        ),
        required=False,
        schemata="default",
        searchable=True,
    ),
    ))

whoswhoSchema = getattr(ATDocument, 'schema', Schema(())).copy() + \
    schema.copy()

# Set storage on fields copied from ATContentTypeSchema, making sure
# they work well with the python bridge properties.

whoswhoSchema['title'].storage = AnnotationStorage()
whoswhoSchema['description'].storage = AnnotationStorage()

schemata.finalizeATCTSchema(whoswhoSchema, moveDiscussion=False)

whoswhoSchema.moveField('text', after='relatedOrgName')

class whoswho(base.ATCTContent):
    """Description of the Example Type"""
    implements(Iwhoswho)

    portal_type = "whoswho"
    schema = whoswhoSchema

    title = ATFieldProperty('title')
    description = ATFieldProperty('description')

registerType(whoswho, PROJECTNAME)
