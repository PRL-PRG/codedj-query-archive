import persistent.interfaces
import zope.interface
import zope.component

import zc.async.interfaces


@zope.component.adapter(persistent.interfaces.IPersistent)
@zope.interface.implementer(zc.async.interfaces.IQueue)
def defaultQueueAdapter(obj):
    return obj._p_jar.root()[zc.async.interfaces.KEY]['']
