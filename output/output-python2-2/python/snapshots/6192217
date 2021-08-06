
def update_location(point_a, point_b, time_delta):
    # use start_loc and end_loc in range of (0,100) for this experiment
    #
    # possible situations are
    #   (90,90), (10,10)	AKA (hi,hi), (lo,lo)
    #   (90,10), (10,90)	AKA (lo,hi), (lo,hi)
    #   (10,90), (90,10)	AKA (hi,lo), (hi,lo)
    #   (10,10), (90,90)	AKA (lo,lo), (hi,hi)
    #
    # It's not likely to happen very often, but there could be a tie in one or
    # both tuples.
    ax=point_a[0]
    ay=point_a[1]
    bx=point_b[0]
    by=point_b[1]

    xdiff=abs(ax-bx)
    ydiff=abs(ay-by)
    multiplier=(time_delta*1.0)/(xdiff+ydiff)
    print '[DEBUG] mult: %.4f' % multiplier

    xdelta=round(multiplier*xdiff)
    ydelta=round(multiplier*ydiff)
    if ax>bx:
        xdelta=-xdelta
    if ay>by:
        ydelta=-ydelta

    return (int(ax+xdelta),int(ay+ydelta))

if __name__=='__main__':
    print update_location((90,90),(10,10),40)	# (70,70)
    print update_location((80,90),(10,10),40)	# (60,70)
    print update_location((80,80),(10,20),60)	# (48,52)

    print update_location((10,90),(90,10),40)	# (30,70)
    print update_location((80,10),(10,70),40)	# (58,28)
    print update_location((30,80),(60,20),20)	# (37,67)
