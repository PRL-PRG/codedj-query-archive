from django import template

register = template.Library()

def below(value, arg):
    "Checks whether value is below args"
    return value < arg

def above(value, arg):
    "Checks whether value is above args"
    return value > arg
    
register.filter('below', below)
register.filter('above', above)