##

from phil import *


#scop_file = '/users/divyab/scop_1.69/dir.cla.scop.txt_1.69.txt'
#vall_scop_file = '/users/divyab/scop_1.69/vallscop.tmp'
scop_file = '/work/pbradley/scop/dir.cla.scop.txt'
vall_scop_file = '/work/pbradley/scop/vall.scop.1.65.2001-02-02'


base_dir = argv[1]
id_list = map(lambda x:string.split(x,'/')[-2],
              glob('%s/?????/?????.fasta'%base_dir))
id_list.sort()

scop = {}

print id_list
for id in id_list:

    chain = id[4];
    if chain == '_':
        chain = '-'

    scop[id] = []
    lines = popen('grep "%s" %s'%(id[:4],scop_file)).readlines()
    for line in lines:
        scop_id = string.split(line)[3]
        if scop_id not in scop[id]:
            if string.split(line)[2][0] == chain :
                scop[id].append(scop_id)

    if argv.count('-switch21') and len(scop[id]) == 0 and \
       id[0] == '2':
        lines = popen('grep "1%s" %s'%(id[1:4],scop_file)).readlines()
        for line in lines:
            scop_id = string.split(line)[3]
            if scop_id not in scop[id]:
                scop[id].append(scop_id)

    print id, scop[id]
    if len( scop[id]) < 1:
    ## if no scop match
        scop_id = string.split(lines[0])[3]
        scop[id].append(scop_id)

    sfam = string.join(string.split( scop[id][0], '.' )[:3], '.')
    print 'excluding sfam= %s for id= %s'%(sfam,id)
    grepper = string.join(string.split(sfam,'.'),'\\.')+'\\.'

    cmd = 'grep "%s" %s'%(grepper,vall_scop_file)
    print cmd
    lines = map(string.split,popen(cmd).readlines())
    bad_ids = []
    for line in lines:
        bad_ids.append(line[0])
        match = 0
        for sf in line[1:]:
            match = match or \
                    string.split(sf,'.')[:3] == string.split(sfam,'.')[:3]
        assert match
    print 'found %d vall ids with this sfam'%(len(bad_ids))

    ## get all hits from homolog valls
    files = glob('%s/%s/h???_/h???_.homolog_vall'%(base_dir,id))

    start_len = len(bad_ids)
    for file in files:
        hom_id = string.split(file,'/')[-2]
        lines = map(string.split,open(file,'r').readlines())
        for line in lines:
            assert len(line) == 2
            for bad_id in line:
                if bad_id == hom_id: continue
                if bad_id in bad_ids: continue
                bad_ids.append( bad_id )
    print 'found %d new ids from the .homolog_vall files'\
          %(len(bad_ids) - start_len )

    hom_file = '%s/%s/%s.homolog_ids'%(base_dir,id,id)
    out = open(hom_file,'w')
    out.write(string.join(bad_ids,'\n')+'\n')
    out.close()

    whrandom.seed()
    for file in files:
        hom_id = string.split(file,'/')[-2]
        old_file = '%s.%f.old'%(file,random())
        assert not exists(old_file)
        system('cp %s %s'%(file,old_file))

        new_file = '%s.big'%file
        out = open(new_file,'w')
        for bad_id in bad_ids:
            out.write('%s %s\n'%(hom_id,bad_id))
        out.close()

        system('cp %s %s'%(new_file,file))




