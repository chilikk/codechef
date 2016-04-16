import sys, Queue

def main():
    n,k = [ int(x) for x in sys.stdin.readline().split() ]
    if n<1 or k<1:
        return
    b = int(sys.stdin.readline())-1
    graph = [ [] for x in range(0,n)]
    for link in range(1,n):
        one, two = [ int(x)-1 for x in sys.stdin.readline().split() ]
        graph[one].append(two)
        graph[two].append(one)
    products = [ [] for x in range(0,k) ]
    cities = [ None for x in range(0,n) ]
    for city in range(0,n):
       prod = int(sys.stdin.readline())-1
       products[prod].append(city)
       cities[city] = prod
    rootpaths = [ None for x in range(0,n) ]
    prodtrees = [ [ n for x in range(0,n) ] for x in range(0,k) ]
    queue = Queue.Queue()
    queue.put((b,[b]))
    dist0 = [ -1 for x in range(0,n) ]
    dist = bftraverse(queue, set([b]), graph, dist0,
            lambda g,h,j:
            build_dist_check_node(g,h,j,rootpaths,cities,prodtrees))
    queries = int(sys.stdin.readline())
    for query in range(0,queries):
        home,prod = [ int(x)-1 for x in sys.stdin.readline().split() ]
        if products[prod] == []:
            print "-1"
            continue
        for i in range(1, len(rootpaths[home])):
            if prodtrees[prod][rootpaths[home][-i]] < n:
                break
        print(prodtrees[prod][rootpaths[home][-i]]+1)

def build_dist_check_node(node, prev, dist, rootpaths, cities, prodtrees):
    dist[node] = dist[prev[-1]]+1
    rootpaths[node] = prev + [node]
    subtree = prodtrees[cities[node]]
    for i in range(1,len(rootpaths[node])):
        subtree[rootpaths[node][-i]] = min(node, subtree[rootpaths[node][-i]])
        if subtree[rootpaths[node][-i-1]] == node:
            break
    subtree[prev[0]] = min(node, subtree[prev[0]])
    return dist

def bftraverse(queue, visited, graph, acc, check_node):
    while not queue.empty():
        node,prev = queue.get()
        acc = check_node(node, prev, acc)
        newprev = prev + [node]
        for adj in graph[node]:
            if not adj in visited:
                queue.put((adj,newprev))
                visited.add(node)
    return acc

if __name__=="__main__":
    main()
