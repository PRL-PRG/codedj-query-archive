import types
import datetime
import pytz
import persistent
import persistent.interfaces
import zope.interface
import zope.component

import zc.async.interfaces
import zc.async.subscribers
import zc.set
from zc.async import rwproperty


@zope.component.adapter(persistent.interfaces.IPersistent)
@zope.interface.implementer(zc.async.interfaces.IDataManager)
def defaultDataManagerAdapter(obj):
    return obj._p_jar.root()[zc.async.subscribers.NAME]


@zope.component.adapter(zc.async.interfaces.IPartial)
@zope.interface.implementer(zc.async.interfaces.IDataManager)
def partial_to_datamanager(partial):
    p = partial.__parent__
    while (p is not None and
           not zc.async.interfaces.IDataManager.providedBy(p)):
        p = getattr(p, '__parent__', None)
    return p


class TransparentDescriptor(object):
    def __init__(self, src_name, value_name, readonly=False):
        self.src_name = src_name
        self.value_name = value_name
        self.readonly = readonly

    def __get__(self, obj, klass=None):
        if obj is None:
            return self
        src = getattr(obj, self.src_name)
        return getattr(src, self.value_name)

    def __set__(self, obj, value):
        if self.readonly:
            raise AttributeError
        src = getattr(obj, self.src_name)
        setattr(src, self.value_name, value)


class DataManagerPartialData(persistent.Persistent):

    workerUUID = assignerUUID = thread = None
    _begin_by = _begin_after = None

    def __init__(self, partial):
        self.__parent__ = self.partial = partial
        self.selectedUUIDs = zc.set.Set()
        self.excludedUUIDs = zc.set.Set()

    @property
    def begin_after(self):
        return self._begin_after
    @rwproperty.setproperty
    def begin_after(self, value):
        if self.assignerUUID is not None:
            raise RuntimeError(
                'can only change begin_after before partial is assigned')
        if value is not None:
            if value.tzinfo is None:
                raise ValueError('cannot use timezone-naive values')
            else:
                value = value.astimezone(pytz.UTC)
        self._begin_after = value

    @property
    def begin_by(self):
        return self._begin_by
    @rwproperty.setproperty
    def begin_by(self, value):
        if self.partial.state != zc.async.interfaces.PENDING:
            raise RuntimeError(
                'can only change begin_by value of PENDING partial')
        if value is not None:
            if value < datetime.timedelta():
                raise ValueError('negative values are not allowed')
        self._begin_by = value

KEY = 'zc.async.datamanagerpartial'


class DataManagerPartial(persistent.Persistent):
    zope.interface.implements(zc.async.interfaces.IDataManagerPartial)
    zope.component.adapts(zc.async.interfaces.IPartial)

    def __init__(self, partial):
        self._data = partial
        if KEY not in partial.annotations:
            partial.annotations[KEY] = DataManagerPartialData(partial)
        self._extra = partial.annotations[KEY]

    for nm in zc.async.interfaces.IPartial.names(True):
        if nm == '__parent__':
            readonly = False
        else:
            readonly = True
        locals()[nm] = TransparentDescriptor('_data', nm, readonly)
    for nm in ('workerUUID', 'assignerUUID', 'thread', 'begin_after',
               'begin_by'):
        locals()[nm] = TransparentDescriptor('_extra', nm)
    for nm in ('selectedUUIDs', 'excludedUUIDs'):
        locals()[nm] = TransparentDescriptor('_extra', nm, True)

@zope.component.adapter(types.MethodType)
@zope.interface.implementer(zc.async.interfaces.IDataManagerPartial)
def method_to_datamanagerpartial(m):
    return DataManagerPartial(zc.async.partial.Partial(m))

@zope.component.adapter(types.FunctionType)
@zope.interface.implementer(zc.async.interfaces.IDataManagerPartial)
def function_to_datamanagerpartial(f):
    return DataManagerPartial(zc.async.partial.Partial(f))
