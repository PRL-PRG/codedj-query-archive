from django.shortcuts import render_to_response, get_object_or_404
from django.template import RequestContext
from voting.models import Vote
from models import DjangoApp
#from favorites.models import Favorite
from threadedcomments.models import ThreadedComment
from django.contrib.models import User
from math import e
from datetime import datetime, timedelta
from operator import itemgetter
from django.contrib.auth import logout
from django.http import HttpResponseRedirect
from django.core.urlresolvers import reverse
from forms import DjangoAppForm

def get_at_least(app_list_input, num=10):
    app_list = [a for a in app_list_input]
    len_of_list = len(app_list)
    num_to_add = num - len_of_list
    if num_to_add <= 0:
        return app_list
    reduced_set = DjangoApp.objects.exclude(id__in=[v[0].id for v in app_list])
    reduced_set = reduced_set.order_by('?')[:num_to_add]
    for app in reduced_set:
        app_list.append((app, 0))
    return app_list

def index(request, num=10):
    context = {
        'app_list': get_at_least(Vote.objects.get_top(DjangoApp, limit=num)),
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
        'app_list' : get_at_least(Vote.objects.get_top(DjangoApp, limit=num)),
    }
    return render_to_response('djangoapps/popular_list.html', 
        context, 
        context_instance=RequestContext(request)
    )

def hot_list(request, num=10, decay_constant=.05, decay_window = 90):
    '''
    Lists the hottest apps.  *Hottness* is based on the rate at which it is 
    recieving votes. The votes will be decayed based on the 
    ``decay_costant`` (c) and the Euler's number (e) and the time from 
    today (t): e^(-ct).  To save some efficiency only the number of days from
    today dictated by the ``decay_window`` will be counted in the hottness 
    score.
    '''
    app_scores = {}
    for instance in DjangoApps.objects.all():
        votes = Vote.objects.filter(object = instance).filter(date_submitted__gte = datetime.now() - timedelta(days=90))
        score = 0
        for vote in votes:
            score = score + e ** ( ( datetime.now() - vote.date_submitted() ).days * decay_constant )
        app_scores[instance]=score
    app_list = [x for (x,y) in sorted(app_scores.item(), key=itemgetter(1), reverse=True)]
    context = {'app_list':app_list}
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
    
def logout_view(request):
    logout(request)
    request.session.set_test_cookie()
    return HttpResponseRedirect(reverse("da_index"))
    
def user_profile(request, username):
    user = User.objects.all(username = username)
#    favorites = Favorite.objects.filter(user=user)
    comments = ThreadedComments.objects.filter(user=user)
    votes = Vote.objects.filter(user=user)
    return render_to_response("djangoapps/userprofile.html", {'favorites':favorites,'comments':comments,'votes':votes})

#def djangoapp_create(request):
    #form = DjangoAppForm(request.POST or None)
    #if(form.is_valid()):
        #app = form.save(commit=False)
        #app.user = request.user
        #app.save()
        #return HttpResponseRedirect(reverse('da_detail', kwargs = {'slug':app.slug}))
    #return render_to_response('djangoapps/djangoapp_form', {'form':form}, )
