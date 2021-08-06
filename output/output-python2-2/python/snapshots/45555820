"""
Controller in MVC model, and view in Django termins

anyway - it is an actions for urls.
"""
import os

from django.shortcuts import render_to_response
from django import forms

from djangoex.pseudomodels import Data

class SimpleManipulator(forms.Manipulator):
    """
    Manipulator for validating simple one-field form
    """
    def __init__(self):
        self.fields = (
            forms.TextField(field_name="name", 
                            is_required=True, 
                            length=30, 
                            maxlength=60,
                            ),
        )
        

def form(request):
    """
    Validate the form and show name
    """
    manipulator = SimpleManipulator()
    name = ''
    if request.method == 'POST':
        new_data = request.POST.copy()
        errors = manipulator.get_validation_errors(new_data)
        if not errors:
            manipulator.do_html2python(new_data)
            name = new_data['name']
            new_data = {}
    else:
        errors = new_data = {}
    context = {'form': forms.FormWrapper(manipulator, new_data, errors), 
               'name': name,
               'title': ' Simple form.'}
    return render_to_response('forms.html', context)


def dynamic(request):
    """
    Just place data in context and render template
    """
    context = {'title': ' Dynamic.', 'data': Data}
    return render_to_response('dynamic.html', context)

def selfcode(request):
    """
    Show code of this project, iterates over files
    """
    code_path = os.path.dirname(__file__)
    pyfiles = []
    for dir_name, dir_list, file_list in os.walk(code_path):
        pyfiles += [{
                    'name': os.path.join(dir_name, f), 
                    'contents': open(os.path.join(dir_name, f)).read()
                    }
                    for f in file_list if f.endswith('.py')]
    context = {'title': ' Selfcode.', 'file_list': pyfiles}
    return render_to_response('selfcode.html', context)