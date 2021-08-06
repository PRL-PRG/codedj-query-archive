from django.shortcuts import render_to_response
from django.template import RequestContext

def index_list(request):
    """
    This will eventually become the index page.  For now, however, it will
    be used as a testbed for the template design.
    """
    context = {}
    return render_to_response('djangoapps/index.html', 
        context, 
        context_instance=RequestContext(request)
    )

def popular_list(request):
    context = {}
    return render_to_response('djangoapps/popular_list.html', 
        context, 
        context_instance=RequestContext(request)
    )

def hot_list(request):
    context = {}
    return render_to_response('djangoapps/hot_list.html', 
        context, 
        context_instance=RequestContext(request)
    )

def new_list(request):
    context = {}
    return render_to_response('djangoapps/new_list.html', 
        context, 
        context_instance=RequestContext(request)
    )

def hotclub(request):
    context = {}
    return render_to_response('djangoapps/hotclub.html', 
        context, 
        context_instance=RequestContext(request)
    )

def detail(request, slug):
    context = {}
    return render_to_response('djangoapps/app_detail.html', 
        context, 
        context_instance=RequestContext(request)
    )

def submit(request):
    context = {}
    return render_to_response('djangoapps/submit_app.html', 
        context, 
        context_instance=RequestContext(request)
    )