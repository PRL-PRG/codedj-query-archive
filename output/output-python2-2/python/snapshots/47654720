from django.shortcuts import render_to_response
from django.template import RequestContext

def index(request):
    """
    This will eventually become the index page.  For now, however, it will
    be used as a testbed for the template design.
    """
    context = {}
    return render_to_response('djangoapps/index.html', 
        context, 
        context_instance=RequestContext(request)
    )