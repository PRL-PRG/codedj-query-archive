# code specific to use within Zope 3

import persistent.interfaces
import zope.component
import zope.security.interfaces
import zope.security.management
import zope.app.component.hooks
import zope.app.security.interfaces

import zc.async.job


class Participation(object):
    zope.interface.implements(zope.security.interfaces.IParticipation)
    interaction = principal = None

    def __init__(self, principal):
        self.principal = principal

class Job(zc.async.job.Job):
    # a job that examines the site and interaction participants when it is
    # created, and reestablishes them when run, tearing down as necessary.

    site = None
    participants = ()

    def __init__(self, *args, **kwargs):
        super(Job, self).__init__(*args, **kwargs)
        site = zope.app.component.hooks.getSite()
        self.site = site
        interaction = zope.security.management.queryInteraction()
        if interaction is not None:
            self.participants = tuple(
                participation.principal.id for participation in
                interaction.participations)

    def setUp(self):
        old_site = zope.app.component.hooks.getSite()
        zope.app.component.hooks.setSite(self.site)
        if self.participants:
            auth = zope.component.getUtility(
                zope.app.security.interfaces.IAuthentication)
            zope.security.management.newInteraction(
                *(Participation(auth.getPrincipal(principal_id)) for
                  principal_id in self.participants))
        return old_site

    def tearDown(self, setup_info):
        zope.app.component.hooks.setSite(setup_info)
        zope.security.management.endInteraction()
