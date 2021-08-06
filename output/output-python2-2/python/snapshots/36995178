## trees from distances

import string
from os import popen,system,getcwd
import sys
from whrandom import random
from math import floor,log,exp
from popen2 import popen2
import fig_devel
from operator import add
from os.path import exists

def Mini_plot(contact_file,upper_left,lower_right):

    decoy_ss = ''
    points = []
    lines = map(string.split,open(contact_file,'r').readlines())
    for line in lines:
        if line[0] == 'DS':
            l = []
            for i in range(3):
                l.append ( [float(line[3+2*i]),line[2+2*i][0]])
            l.sort()
            decoy_ss = decoy_ss + l[-1][1]
        elif line[0] == 'DC':
            fraction = float(line[3])
            fill_power = int(floor(fraction*20))
            if fill_power:
                points.append([fill_power,int(line[1]),int(line[2])])
    points.sort()

    ## rescale
    L = len(decoy_ss)
    w1 = float( lower_right[0] - upper_left[0])/L ## floats
    h1 = float( lower_right[1] - upper_left[1])/L

    w2 = max( 3, int(floor(1.0+w1)))
    h2 = max( 3, int(floor(1.0+h1)))

##     sys.stderr.write('widths: w1=%4.1f h1=%4.1f w2=%d h2=%d\n'\
##                      %(w1,h1,w2,h2))

    fig_text = ''

    ## show ss
    for i in range(L):
        x = int(floor(0.5+upper_left[0] + i*w1))
        y = int(floor(0.5+lower_right[1] - (i+1)*h1))

        b = fig_devel.BOX([x,y],[x+w2,y+h2])

        if decoy_ss[i] == 'E':
            b.fill_power = 20
        elif decoy_ss[i] == 'H':
            b.fill_power = 10
        else:
            continue
        fig_text = fig_text + fig_devel.New_box(b)


    ## show contacts
    for p in points: ## sorted by increasing fill power
        x = int(floor(0.5+upper_left[0] + p[2]*w1))
        y = int(floor(0.5+lower_right[1] - (p[1]+1)*h1))

        b = fig_devel.BOX([x,y],[x+w2,y+h2])
        b.fill_power = p[0]
        fig_text = fig_text + fig_devel.New_box(b)

    ## add a black box
    b = fig_devel.BOX(upper_left,lower_right)
    b.fill_power = -1
    b.line_width = 1
    fig_text = fig_text + fig_devel.New_box(b)

    return fig_text

def IsALeaf(node): return (node[0] == node[1])


def Average_score (leaf_list, leaf_scores, percentile):
    ls = []
    for leaf in leaf_list:
        ls = ls + leaf_scores[leaf]
    ls.sort()

    pos = (percentile * len(ls) ) / 100
    if pos==len(ls): pos = len(ls)-1

    return ls[pos]


def Make_tree(distance,num_leaves,Update_distance_matrix,leaf_scores,percentile):
    N = num_leaves

    nodes = []
    for i in range(N):
        nodes.append( (i,i,0.0,Average_score([i],leaf_scores,percentile)) )

    for i in range(N): ## initialize distance matrix
        for j in range(N):
            distance[(nodes[i],nodes[j])] = distance[(i,j)]


    while N>1:

        ## find two closest nodes and join them
        min_d = 100000

        for i in nodes:
            for j in nodes:
                if i<=j:continue
                if distance[(i,j)] < min_d:
                    min_d = distance[(i,j)]
                    n1 = i
                    n2 = j

##         print "num_nodes: %d   Joining: %s and %s   Distance: %7.3f\n"\
##               %(N,Show_small(n1),Show_small(n2),min_d)

        new_node = (n1,n2,min_d,Average_score( Node_members(n1)+Node_members(n2),
                                               leaf_scores,percentile))


        ## update the distances
        Update_distance_matrix (new_node,nodes,distance)

        ## update the node_list
        nodes.append(new_node)
        del nodes[ nodes.index(n1)]
        del nodes[ nodes.index(n2)]

        N = N-1

    return nodes[0]

def Show_tree(tree,names):
    if IsALeaf(tree):
        return names [tree[0]]
    else:
        return '('+Show_tree(tree[0],names)+':'+str(float(tree[2])/2)+','+\
               Show_tree(tree[1],names)+':'+str(float(tree[2])/2)+')'

def Show_small(tree):
    if IsALeaf(tree):
        return `tree[0]`
    else:
        return '('+Show_small(tree[0])+','+Show_small(tree[1])+')'

def Node_members(node):
    if IsALeaf(node):
        return [node[0]]
    else:
        l1 = Node_members( node[0] )
        l2 = Node_members( node[1] )
        if min(l1)<min(l2):
            return l1+l2
        else:
            return l2+l1


def Update_distance_matrix_AL(new_node,old_nodes,distances): ## single linkage
    n1 = new_node[0]
    n2 = new_node[1]

    l1 = Node_members(new_node)
    distances [ (new_node,new_node)] = 0.0

    for n in old_nodes:
        if n==n1 or n==n2:continue
        l2 = Node_members(n)

        avg = 0.0
        count = 0
        for i in l1:
            for j in l2:
                assert i!=j
                avg = avg+ distances[(i,j)]
                count = count + 1

        distances[(n,new_node)] = avg/count
        distances[(new_node,n)] = avg/count

    return

def Update_distance_matrix_AL_GEOM(new_node,old_nodes,distances):
    dl = distances.values()
    dl.sort()
    for i in dl:
        if i!=0:
            min_log = log(i) - 3 ## closer than the closest non-id pair
            break

    n1 = new_node[0]
    n2 = new_node[1]

    l1 = Node_members(new_node)
    distances [ (new_node,new_node)] = 0.0

    for n in old_nodes:
        if n==n1 or n==n2:continue
        l2 = Node_members(n)

        avg = 0.0
        count = 0
        for i in l1:
            for j in l2:
                d = distances[(i,j)]
                assert i!=j
                count = count + 1
                if d == 0.0:
                    avg = avg + min_log
                else:
                    avg = avg + log( d )

        distances[(n,new_node)] = exp( avg / count )
        distances[(new_node,n)] = exp( avg / count )

    return

def Update_distance_matrix_SL(new_node,old_nodes,distances): ## single linkage
    n1 = new_node[0]
    n2 = new_node[1]

    l1 = Node_members(new_node)
    distances [ (new_node,new_node)] = 0.0

    for n in old_nodes:
        if n==n1 or n==n2:continue
        l2 = Node_members(n)

        min_d = 1000
        count = 0
        for i in l1:
            for j in l2:
                assert i!=j
                min_d = min(min_d, distances[(i,j)])

        distances[(n,new_node)] = min_d
        distances[(new_node,n)] = min_d

    return

def Center(tree,node_position):
    l = Node_members(tree)
    pos = 0.0
    for i in l:
        pos = pos + node_position[i]
    pos = pos/len(l)
    return pos

def Size(tree,sizes):
    if IsALeaf(tree):
        return sizes[tree[0]]
    else:
        return Size(tree[0],sizes)+Size(tree[1],sizes)

def Fig_tree(tree,node_position,sizes): ## edge = [ [x0,y0], [x1,y1], score, size]
    if IsALeaf(tree):
        return []
    else:

        rmsd = tree[2]
        center = Center(tree,node_position)

        c0 = Center(tree[0],node_position)
        r0 = tree[0][2]
        score0 = tree[0][3]
        size0 = Size(tree[0],sizes)
        if IsALeaf(tree[0]):
            cluster0 = tree[0][0]
        else:
            cluster0 = -1
        e0_horizontal = [ [rmsd, c0], [r0,c0], score0, size0, cluster0]
        e0_vertical   = [ [rmsd, c0], [rmsd,center], score0, 1, cluster0]

        c1 = Center(tree[1],node_position)
        r1 = tree[1][2]
        score1 = tree[1][3]
        size1 = Size(tree[1],sizes)
        if IsALeaf(tree[1]):
            cluster1 = tree[1][0]
        else:
            cluster1 = -1
        e1_horizontal = [ [rmsd, c1], [r1, c1], score1, size1 , cluster1]
        e1_vertical   = [ [rmsd, c1], [rmsd,center], score1, 1, cluster1]

        return [ e0_vertical,e0_horizontal,e1_vertical,e1_horizontal] + \
               Fig_tree(tree[0],node_position,sizes) + \
               Fig_tree(tree[1],node_position,sizes)

def Node_labels(tree,sizes,node_position):
    if IsALeaf(tree):return []
    else:
        pos = [tree[2],Center(tree,node_position)]
        size = 0
        for leaf in Node_members(tree):
            size = size+sizes[leaf]
        return [ [ `size`, pos] ] + \
               Node_labels(tree[0],sizes,node_position) + \
               Node_labels(tree[1],sizes,node_position)

def Plot_tree(tree, names, sizes, ps_file, GREY_SCALE = 0, num_cartoons = 0, prefix = ''):
    LEFT_MARGIN = 1500
    RIGHT_MARGIN = 1000
    TOP_MARGIN = 1000
    BOTTOM_MARGIN = 1500

    MAX_CARTOON_WIDTH = 4000
    cartooned = []

    ## allocate widths for branches; widths measure cluster sizes
    total = reduce(add,sizes)
    w_factor = 100.0/total ## allow 15*100 units for branch widths
    total = 0
    for s in sizes:
        width = max(1,int(floor(0.5+ s*w_factor)))
        total = total+width
    remainder = fig_devel.LONG_DIMENSION - total*15 - TOP_MARGIN -BOTTOM_MARGIN
    cluster_width = float(remainder)/len(names)

    ## position nodes vertically on tree
    nodes = Node_members(tree)
    node_position = {}
    mark = fig_devel.LONG_DIMENSION - BOTTOM_MARGIN
    for i in range(len(nodes)):
        node_position[nodes[i]] = mark
        width = max(1,int(floor(0.5+ s*w_factor)))
        mark = mark - cluster_width - width * 15

    edges = Fig_tree(tree,node_position,sizes) ## each edge = [[x0,y0],[x1,y1],score,size,cluster]

    ## create fig file

    fig_file = '/tmp/junk_phil_'+str(random())+'.fig'
    out = open(fig_file,'w')
    if GREY_SCALE:
        out.write(fig_devel.Header('p')+fig_devel.Greyscale_colors())
    else:
        out.write(fig_devel.Header('p')+fig_devel.Rainbow_colors())




    ## set fontsize
    font = min(18, max (5, int(floor( 0.5 + (cluster_width+15)/20))))

    if num_cartoons: ######### cartoon pictures of plots

        CARTOON_MARGIN = 50

        ## how many cartoons should we position next to one another?
        best_width = [0]
        for W in range(1,10):
            H = num_cartoons/W + ( (num_cartoons%W>0))
            cartoon_height = (fig_devel.LONG_DIMENSION - 2*CARTOON_MARGIN - \
                              TOP_MARGIN - BOTTOM_MARGIN)/ H

            cartoon_width = min( MAX_CARTOON_WIDTH / W, cartoon_height)
            if cartoon_width> best_width[0]:
                best_width = [cartoon_width,W,H]

        W = best_width[1]
        H = best_width[2]

        cartoon_height = (fig_devel.LONG_DIMENSION - 2*CARTOON_MARGIN - \
                          TOP_MARGIN - BOTTOM_MARGIN)/ H
        cartoon_width = min( MAX_CARTOON_WIDTH / W, cartoon_height)

        #cartoon_height = (fig_devel.LONG_DIMENSION - 100 - TOP_MARGIN - BOTTOM_MARGIN)/num_cartoons
        #nodes_per_cartoon = max(1,len(nodes)/num_cartoons)
        nodes_per_cartoon = float(len(nodes))/num_cartoons

        for cartoon in range(num_cartoons):
            if cartoon == num_cartoons-1:
                members = nodes [int(floor(cartoon * nodes_per_cartoon)): ]
            else:
                members = nodes [int(floor(cartoon * nodes_per_cartoon)):\
                                 int(floor((cartoon+1)*nodes_per_cartoon))]

            rep = min(members) ## choose the lowest cluster number, should be largest size
            for n in members: ## sanity check
                assert sizes[rep] >= sizes[n]
            cartooned.append(rep)

            contact_file = prefix+'.cluster%02d.%03d.contacts'%(rep,sizes[rep])

            if not exists(contact_file):
                sys.stderr.write('missing contact_file: %s\n'%contact_file)
                continue

            x = fig_devel.SHORT_DIMENSION - RIGHT_MARGIN - cartoon_width * (cartoon%W)
            y = fig_devel.LONG_DIMENSION - CARTOON_MARGIN - BOTTOM_MARGIN - \
                cartoon_height * (cartoon/W)
            lower_right = [x,y]

            #lower_right = [ fig_devel.SHORT_DIMENSION-RIGHT_MARGIN,
            #                fig_devel.LONG_DIMENSION-50-BOTTOM_MARGIN-cartoon * cartoon_height]


            upper_left = [lower_right[0] - cartoon_width, lower_right[1]-cartoon_width]
            sys.stderr.write('cartoon %d represents cluster: %d, size: %d\n'\
                             %(cartoon,rep,sizes[rep]))


            out.write(Mini_plot(contact_file,upper_left,lower_right))
            out.write(fig_devel.Text(names[rep],[upper_left[0]+50,upper_left[1]+font*20],font,'l'))
        RIGHT_MARGIN = RIGHT_MARGIN + cartoon_width*W + CARTOON_MARGIN


    ## rescale the x-positions
    max_rmsd = tree[2]
    min_rmsd = tree[2]
    for e in edges:
        if e[0][0]>0: min_rmsd = min(min_rmsd,e[0][0])
        if e[1][0]>0: min_rmsd = min(min_rmsd,e[1][0])
    min_rmsd = max(0,min_rmsd-0.5)

    def Transform(rmsd,min_rmsd = min_rmsd, max_rmsd = max_rmsd,
                  LEFT_MARGIN=LEFT_MARGIN, RIGHT_MARGIN=RIGHT_MARGIN):
        return LEFT_MARGIN + int (floor ( 0.5 + float (fig_devel.SHORT_DIMENSION - LEFT_MARGIN -\
                                                       RIGHT_MARGIN ) * \
                                          (rmsd - min_rmsd) / (max_rmsd - min_rmsd)))


    ## rescale colors
    scores = []
    for e in edges:
        scores.append(e[2])
    min_score = min(scores)
    max_score = max(scores)
    if max_score == min_score:
        max_score = max_score + 1
    if GREY_SCALE:
        min_score = min_score - (0.1 * (max_score-min_score))





    ## write the edges
    for e in edges:
        start = [ Transform (max(e[0][0],min_rmsd)), e[0][1]] ## rescale x-position
        stop = [ Transform (max(e[1][0],min_rmsd)), e[1][1]]
        l = fig_devel.LINE (start,stop)
        l.color = fig_devel.Color ( float( e[2] - min_score)/(max_score-min_score) ,0)
        l.width = max(1,int(floor(0.5+ e[3]*w_factor)))
        out.write(fig_devel.Line(l))

    ## show scale
    out.write(fig_devel.Line(fig_devel.LINE( [Transform(min_rmsd),TOP_MARGIN],
                                             [Transform(max_rmsd),TOP_MARGIN])))

    for i in range(int(floor(min_rmsd+1)),1+int(floor(tree[2]))):
        out.write(fig_devel.Text( str(i), [Transform(i),TOP_MARGIN], 18))


    ## show a color scale
    if GREY_SCALE:
        out.write(fig_devel.Text( 'colors: from white (%7.2f) to black (%7.2f)'%(min_score,max_score),
                                  [LEFT_MARGIN,
                                   fig_devel.LONG_DIMENSION-BOTTOM_MARGIN+300],
                                  10,'l'))
    else:
        out.write(fig_devel.Text( 'Colors: from blue (%7.2f) to red (%7.2f)'%(min_score,max_score),
                                  [LEFT_MARGIN,
                                   fig_devel.LONG_DIMENSION-BOTTOM_MARGIN+300],
                                  10,'l'))

    ## show a label
    out.write(fig_devel.Text('directory: %s'%getcwd(),
                             [LEFT_MARGIN,
                              fig_devel.LONG_DIMENSION-BOTTOM_MARGIN+450],
                             10,'l'))
    out.write(fig_devel.Text('command: %s'%string.join(sys.argv),
                             [LEFT_MARGIN,
                              fig_devel.LONG_DIMENSION-BOTTOM_MARGIN+600],
                             10,'l'))
    #print getcwd()
    #print string.join(sys.argv)



    ## label leaves
    for i in range(len(names)):
        out.write(fig_devel.Text('*'*(5*(i in cartooned))+names[i],
                                 [Transform(min_rmsd),node_position[i]], font, 'r'))

    ## label internal vertices with sizes
    for l in Node_labels (tree,sizes,node_position):
        out.write(fig_devel.Text(l[0], [Transform(l[1][0]),l[1][1]], font, 'r'))

    out.close()

    #system('fig2dev -L png -b 50 '+fig_file+' '+ps_file)
    system('fig2dev -L ps  '+fig_file+' '+ps_file)
    system('rm '+fig_file)
    return



def Canvas_tree(tree, names, sizes, plotter, plot_width, plot_height):
    ## plot_width and plot_height in pixels

    ## plotter has methods:
    ## .make_line ( [x0,y0], [x1,y1], line_width, normalized_score)
    ## .make_text (text,  [x,y], font)

    branch_width_pixels = min(100,plot_height/5)

    ## allocate widths for branches; widths measure cluster sizes
    total = reduce(add,sizes)
    w_factor = float( branch_width_pixels) / total
    total = 0
    for s in sizes:
        width = max(1,int(floor(0.5+ s*w_factor))) ## in pixels
        total = total+width
    remainder = plot_height - total
    cluster_width = float(remainder)/len(names) ## padding alotted to each cluster

    ## position nodes vertically on tree
    nodes = Node_members(tree)
    node_position = {}
    mark = plot_height
    for i in range(len(nodes)):
        node_position[nodes[i]] = mark
        width = max(1,int(floor(0.5+ s*w_factor)))
        mark = mark - cluster_width - width

    edges = Fig_tree(tree,node_position,sizes) ## each edge = [[x0,y0],[x1,y1],score,size,cluster]


    ## set fontsize: is this still right??

    font = min(18, max (5, int(floor( 0.5 + (cluster_width+7.5)/10))))

    ## rescale the x-positions
    max_rmsd = tree[2]
    min_rmsd = tree[2]
    for e in edges:
        if e[0][0]>0: min_rmsd = min(min_rmsd,e[0][0])
        if e[1][0]>0: min_rmsd = min(min_rmsd,e[1][0])
    min_rmsd = max(0,min_rmsd-0.5)

    def Transform(rmsd,min_rmsd = min_rmsd, max_rmsd = max_rmsd,plot_width = plot_width):
        return int (floor ( 0.5 + plot_width * (rmsd - min_rmsd) / (max_rmsd - min_rmsd)))


    ## rescale colors
    scores = []
    for e in edges:
        scores.append(e[2])
    min_score = min(scores)
    max_score = max(scores)
    if max_score == min_score:
        max_score = max_score + 1

    ## write the edges
    for e in edges:
        start = [ Transform (max(e[0][0],min_rmsd)), e[0][1]] ## rescale x-position
        stop = [ Transform (max(e[1][0],min_rmsd)), e[1][1]]

        normalized_score = float( e[2] - min_score)/(max_score-min_score)
        line_width = max(1,int(floor(0.5+ e[3]*w_factor)))
        if e[4]>=0: ## it's a real cluster edge
            cluster = e[4]
            extra_tag = 'cluster%02d.%03d'%(cluster,sizes[cluster])
        else:
            extra_tag = 'dummy'

        plotter.make_line(start,stop,line_width,normalized_score,extra_tag)


    ## show scale
    plotter.make_line([Transform(min_rmsd),5], [Transform(max_rmsd),5],3,1.0)

    for i in range(int(floor(min_rmsd+1)),1+int(floor(tree[2]))):
        plotter.make_text( str(i), [Transform(i),0], 18)


    plotter.make_text( 'Colors: from blue (%7.2f) to red (%7.2f)'%(min_score,max_score),
                       [0,25],10)


    ## label leaves
    for i in range(len(names)):
        extra_tag = 'cluster%02d.%03d'%(i,sizes[i])
        plotter.make_text(names[i],
                          [Transform(min_rmsd),node_position[i]],
                          font,extra_tag)

    ## label internal vertices with sizes
    for l in Node_labels (tree,sizes,node_position):
        plotter.make_text(l[0], [Transform(l[1][0]),l[1][1]], font)

    return



if 0: ## testing
    ## read distances
##     lines = map(string.split,popen('grep "CLUSTER_RMSD" ../C/t148_target.info').readlines())
    lines = map(string.split,popen('grep "CLUSTER_RMSD" ../C/junk1ubi.info').readlines())
##     lines = map(string.split,popen('grep "CLUSTER_RMSD" ../C/junk1c8cA.info').readlines())
    distance = {}
    N = len(lines)
    sizes = []
    for i in range(N):
        assert i == int(lines[i][1])
        sizes.append ( int(lines[i][2]) )
        assert len(lines[i]) == 10+i
        for j in range(i+1):
            distance[(j,i)] = float(lines[i][9+j])
            distance[(i,j)] = float(lines[i][9+j])
    m = 0
    for i in range(N):
        m = max(m,len(`sizes[i]`))
    names = []
    for i in range(N):
        names.append( `i`+'_'+`sizes[i]`)
    tree = Make_tree(distance, N, Update_distance_matrix_AL)

    Plot_tree( tree, names, sizes, 'junk1.ps')
    tree = Make_tree(distance, N, Update_distance_matrix_SL)

    Plot_tree( tree, names, sizes, 'junk2.ps')

