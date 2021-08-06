"""
Setup your Routes options here
"""
import sys, os
from routes import Mapper

def make_map(global_conf={}, app_conf={}):
    root_path = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))

    map = Mapper(directory=os.path.join(root_path, 'controllers'))
    
    # This route handles displaying the error page and graphics used in the 404/500
    # error pages. It should likely stay at the top to ensure that the error page is
    # displayed properly.
    map.connect('error/:action/:id', controller='error')
    
    # Make routes
    map.connect(':action', controller='simple')
    
    # Just render template
    map.connect('*url', controller='template', action='view')

    return map
