from django import template

register = template.Library()

def below(value, arg):
    "Checks whether value is below args"
    return arg > value

def above(value, arg):
    "Checks whether value is above args"
    return arg < value
    
register.filter('below', below)
register.filter('above', above)