import os
import formencode
from pylonsex.lib.base import *
from pylonsex.models import Data

class SimpleForm(formencode.Schema):
    allow_extra_fields = True
    filter_extra_fields = True
    name = formencode.validators.String(min=1, max=60)

class SimpleController(BaseController):
    def index(self):
        return render_response('/index.html')
    
    def forms(self):
        if request.method == 'POST':
            schema = SimpleForm()
            try:
                form_result = schema.to_python(request.params)
            except formencode.Invalid, error:
                c.form_errors = error.error_dict or {}
            else:
                c.name = form_result['name']
                c.form_errors = {}
        c.title = ' Simple form.'
        return render_response('/forms.html')
    
    def selfcode(self):
        import pylonsex
        code_path = os.path.dirname(pylonsex.__file__)
        pyfiles = []
        for dir_name, dir_list, file_list in os.walk(code_path):
            pyfiles += [{
                        'name': os.path.join(dir_name, f), 
                        'contents': open(os.path.join(dir_name, f)).read()
                        }
                        for f in file_list if f.endswith('.py')]
        c.file_list = pyfiles
        c.title = ' Selfcode.'
        return render_response('/selfcode.html')
    
    def dynamic(self):
        c.data = Data
        c.title = ' Dynamic.'
        return render_response('/dynamic.html')
