from django.shortcuts import render_to_response, get_object_or_404
from django.template import RequestContext
from voting.models import Vote
from models import DjangoApp

def index(request, num=10):
    context = {
#        'app_list': Vote.objects.get_top(num),
    }
    return render_to_response('djangoapps/index.html', 
        context, 
        context_instance=RequestContext(request)
    )

def popular_list(request, num=10):
    """
    Lists all of the most popular applications on djangoapps.  This is 
    calculated by getting the sum of all of the votes on each app, and taking
    the top results.
    
    Arguments:
    
    ``num``
        The number of results to return.  Defaults to 10.
    """
    context = {
        'app_list' : Vote.objects.get_top(num),
    }
    return render_to_response('djangoapps/popular_list.html', 
        context, 
        context_instance=RequestContext(request)
    )

def hot_list(request, num=10):
    '''
    Lists the hottest apps.  *Hottness* is based on the rate at which it is 
    recieving up votes. it is calculated by weighting the votes assymptotically
    by time. 
    for example:
        obj A recieved 200 votes today, 100 votes this week(not including 
        today) and 3 votes this month(not including this week) and 0 other
        votes.
        obj A hottness rating = 200*10 + 100*5 + 3*1 = 2503
        obj A total votes = 200 + 100 + 3 = 303

        obj B recieved 9 votes today, 300 votes this week(not including today)
        and 600 votes this month(not including this week) and 0 other votes.
        obj B hottness rating = 9*10 + 300*5 = 600*1 = 2190
        obj B total votes = 9 + 300 + 600 = 909
    '''

    context = {}
    return render_to_response('djangoapps/hot_list.html', 
        context, 
        context_instance=RequestContext(request)
    )

def new_list(request, num=10):
    """
    Lists of the newest applications on djangoapps.
    
    Arguments:
    
    ``num``
        The number of results to return.  Defaults to 10.
    """
    context = {
        'app_list': DjangoApp.objects.order_by('-date_added')[:num],
    }
    return render_to_response('djangoapps/new_list.html', 
        context, 
        context_instance=RequestContext(request)
    )

def hotclub(request):
    """
    Lists of all applications on djangoapps which have reached "hotclub" 
    status.  All hotclub applications are guaranteed to work together and 
    exhibit best practices.
    """
    context = {
        'app_list': DjangoApp.objects.filter(is_hotclub=True).order_by('name')[:num],
    }
    return render_to_response('djangoapps/hotclub.html', 
        context, 
        context_instance=RequestContext(request)
    )

def detail(request, slug):
    """
    Shows the details about a particular application.
    """
    context = {
        'app': get_object_or_404(DjangoApp, slug=slug),
    }
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