import sys

def main():
    [n,r,g,b] = [ int(x) for x in sys.stdin.readline().split() ]
    graph=dict((i,[]) for i in range(1,n+1))
    for i in range(1,n):
        [a,b] = [ int(x) for x in sys.stdin.readline().split() ]
        graph[a].append(b)
        graph[b].append(a)
    bfs(graph, remove_parent, None)

def remove_parent(node, graph, acc):
    for child in graph[node]:
        graph[child].remove(node)
    return acc

def flatten(node, graph, flat):
    return flat + graph[node]

def bfs(graph, fun, acc):
    queue = [1] # 1 is the root of the tree by definition
    for node in queue:
        acc = fun(node, graph, acc)
        queue += graph[node]
    return acc

def combinations(flat, rgb):
    acc = 0
    for (color,value) in rgb.iteritems():
        if value>0:
            #print "node %s case %s" % (node, color)
            partialacc = 1
            for child in graph[node]:
                rgbchild = rgb.copy()
                rgbchild.update({color:value-1})
                partialacc *= combinations(child, graph, rgbchild)
            acc += partialacc
    #print "exit node %s" % node
    return acc


if __name__=="__main__":
    main()
