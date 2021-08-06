# This file contains code that only exists for backwards compatibility with
# previous versions of zc.async.  Typically, these definitions are imported
# with ``from zc.async.legacy import EXAMPLE`` so that database references
# can find the code in the old locations.

import twisted.python.failure

def success_or_failure(success, failure, res):
    callable = None
    if isinstance(res, twisted.python.failure.Failure):
        if failure is not None:
            callable = failure
    elif success is not None:
        callable = success
    if callable is None:
        return res
    return callable(res)

def chooseFirst(agent):
    return agent.queue.claim()
