from rpy import r
import rpy

def fitPoly(xarray, yarray, order):

    r.lm.local_mode(rpy.NO_CONVERSION)

    xl=list(xarray)
    yl=list(yarray)
    
    modelDef = "y ~ poly(x,%d)" % order
    model=r.lm(r(modelDef), data=r.data_frame(x=xl,y=yl))
    
    pred=r.predict(model)

# pred is now a dict with keys from '1' to 'N', where N is the size of xl

    predvals = []

    for i in range(len(xl)):
        predvals.append(pred[str(i+1)])
        
    return(xl, predvals)
