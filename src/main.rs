#[derive(Debug)]
struct Queue<T>{
    arr:Vec<T>,
    front:isize
}

impl <T> Queue<T>{
    fn new() -> Self{
        Self { arr: vec![], front: -1 }
    }

    fn push(&mut self,val:T){
        if self.front == -1{
            self.arr.push(val);
            self.front = 0;
            return;
        }
        self.arr.push(val);
    }

    fn empty(&self) -> bool{
        if self.front == -1 || self.front as usize >= self.arr.len(){
            return true;
        }
        return false;
    }

    fn pop(&mut self){
        if self.empty(){
            panic!("Queue is empty cannot pop out the element");
        }
        self.front += 1;
    }

    fn top(&self) -> &T{
        if self.empty(){
            panic!("Queue is empty cannot look up the element");
        }
        return &self.arr[self.front as usize];    // right now a reference only 
    }
}
type Wgrapgh = Vec<Vec<(usize,usize)>>;
type Graph   = Vec<Vec<usize>>;

fn dijkstras(gr:Wgrapgh,src:usize) -> Vec<usize>{
    let mut dist:Vec<usize> = Vec::with_capacity(gr.len());
    dist.resize(gr.len(),0);

    for ele in dist.iter_mut(){
        *ele = usize::MAX;
    }
    dist[src] = 0;
    let mut q:Queue<usize> = Queue::new();
    q.push(src);

    while !q.empty(){
        let top = *q.top();q.pop();    
        
        for i in &gr[top]{
            let v = i.0;
            let wt = i.1;

            if dist[top] + wt < dist[v]{
                dist[v] = dist[top] + wt;
                q.push(v);   
            }   
        }
    }   

    return dist;
}

fn dfs(g:&Graph,src:usize){
    let mut vis:Vec<usize> = Vec::with_capacity(g.len());
    vis.resize(g.len(),0);

    fn ddfs(g:&Graph,vis:&mut Vec<usize>,src:usize){
        vis[src] = 1;
        println!("{}",src);

        for i in &g[src]{
            if vis[*i] == 0{
                ddfs(g, vis, *i);
            }
        }    
    }

    ddfs(g,&mut vis,0);
}


fn bfs(g:&Graph, src:usize){
    let mut vis:Vec<usize> = vec![0;g.len()];
    let mut q:Queue<usize> = Queue::new();
    q.push(src);
    vis[src] = 1;
    
    while !q.empty(){
        let top = *q.top();q.pop();
        println!("{}",top);

        for i in &g[top]{
            if vis[*i] == 0{
                q.push(*i);
                vis[*i] = 1;
            }
        }

    }
}

fn top_sort(g:&Graph) -> Vec<usize>{
    let mut vis:Vec<usize> = vec![0;g.len()];
    let mut st:Vec<usize>  = vec![];

    fn dfs(g:&Graph,src:usize,vis:&mut Vec<usize>,st:&mut Vec<usize>){
        vis[src] = 1;
        
        for i in &g[src]{
            if vis[*i] == 0{
                dfs(g, *i, vis, st); // recursive call
            }
        }
        st.push(src);
    }

    for i in 0..g.len(){
        if vis[i] == 0{
            dfs(g, i, &mut vis, &mut st);
        }
    }

    return st;
}

fn articulation_point(g:&Graph) -> Vec<usize>{
    let mut vis:Vec<usize> = vec![0;g.len()];
    let mut low:Vec<usize> = vec![0;g.len()];
    let mut disc:Vec<usize> = vec![0;g.len()];
    let mut is_Ap:Vec<usize> = vec![];
    let mut time:usize = 0;

    // huge function definition in here 
    fn dfs(g:&Graph,u:usize,parent:usize,vis:&mut Vec<usize>,low:&mut Vec<usize>,desc:&mut Vec<usize>,is_Ap:&mut Vec<usize>,time:&mut usize){
        let mut children:usize = 0;
        vis[u] = 1;
        *time += 1;
        low[u] = *time;
        desc[u] = *time;
        
        for v in &g[u]{  
            if vis[*v] == 0{
                children+=1;
                dfs(g,*v,u,vis,low,desc,is_Ap,time);
                low[u] = usize::min(low[u],low[*v]);

                if parent != 1001 && desc[u] <= low[*v]{
                    is_Ap.push(u);
                }

            }else if *v != parent{
                low[u] = usize::min(low[u],desc[*v]);
            }
        }

        if parent == 1001 && children > 1{
            is_Ap.push(u);
        }
    }

    for i in 0..g.len(){
        if vis[i] == 0{
            dfs(g, i, 1001, &mut vis, &mut low, &mut disc, &mut is_Ap, &mut time);
        }
    }
    is_Ap

}


// Kahn's ALgorithms for doing topological sort in a graph
fn top_sort_kahn(g:&Graph) -> Vec<usize>{
    let mut indegree:Vec<usize> = vec![0;g.len()];
    let mut q:Queue<usize> = Queue::new();
    let mut st:Vec<usize> = vec![];

    for i in 0..g.len(){
        for k in &g[i]{
            indegree[*k]+=1;
        }
    }
    
    for i in 0..g.len(){
        if indegree[i] == 0{
            q.push(i);
        }
    }

    let mut bfs = || {   // closure heuristics
        
        while !q.empty(){
            let top = *q.top();q.pop();
            st.push(top);
            
            for v in &g[top]{
                indegree[*v] -= 1;
                
                if indegree[*v] == 0{
                    q.push(*v);
                }
            }
        }
    };

    bfs();

    st

}


// base implmentation for SCC SOLVER (strongly connected companents)
struct SccSolver{
    low:Vec<usize>,
    desc:Vec<usize>,
    st:Vec<usize>,
    time:usize,
    is_stack:Vec<usize>,
}

impl SccSolver{
    fn new() -> Self{
        Self { low: Vec::new(), desc: Vec::new(), st: Vec::new(), time: 0, is_stack: Vec::new() }
    }

    fn set_desc(&mut self){
        for ele in self.desc.iter_mut(){
            *ele = 1001;
        }
    }

    fn dfs(&mut self,g:&Graph,u:usize){
        self.time += 1;
        self.desc[u] = self.time;
        self.low[u]  = self.time;
        self.st.push(u);
        self.is_stack[u] = 1;

        for v in &g[u]{
            if self.desc[*v] == 1001{
                self.dfs(g, *v);                
                self.low[u] = usize::min(self.low[u],self.low[*v]);
            }else if self.is_stack[*v] == 1{
                self.low[u] = usize::min(self.low[u],self.desc[*v]);
            }
        }

        if self.low[u] == self.desc[u]{
            loop {
                let val = *self.st.last().unwrap();
                self.st.pop();
                println!("{}",val);
                if val == u{
                    break;
                }
                self.is_stack[val] = 0;
            }
        }
    }

    fn tarjans_method(&mut self, g:&Graph){
        self.desc.resize(g.len(),0);
        self.low.resize(g.len(),0);
        self.is_stack.resize(g.len(),0);

        self.set_desc(); // setting up the desc max value for a directed graph 
        
        for i in 0..g.len(){
            if self.desc[i] == 1001{
                self.dfs(g,i);
            }
        }

        //println!("{:?}",self.st);
    }
}

// cycle detection 
// using normal STACK and dfs approach

struct CycleDetectionSolver{
    vis:Vec<usize>,
    in_stack:Vec<usize>
}

impl CycleDetectionSolver{
    fn new() -> Self{
        Self { vis: Vec::new(),in_stack: Vec::new() }      // design decision
    }

    fn dfs(&mut self, g:&Graph,u:usize) -> bool{
        self.in_stack[u] = 1;

        for i in &g[u]{
            if self.in_stack[*i] == 1{
                return true;
            }
            if self.vis[*i] == 0 && self.dfs(g,*i){
                self.dfs(g,*i);
            }
        }
        self.in_stack[u] = 0;
        false
    }

    fn solve(&mut self,g:&Graph) -> bool{
        self.in_stack.resize(g.len(),0);
        self.vis.resize(g.len(),0);

        for i in 0..g.len(){
            if self.vis[i] == 0{
                if self.dfs(g,i){
                    return true;
                }
            }    
        }
        false
    }
}

fn main() {
    //let g:Wgrapgh = vec![vec![(1,12),(2,1),(3,2)],vec![(0,12),(2,2)],vec![(3,4),(0,1),(1,2)],vec![(0,2),(2,4)]];
    //let g:Graph = vec![vec![1,2,3],vec![0],vec![0,3],vec![2,0]];
    //let t:Graph = vec![vec![2,1],vec![3],vec![3],vec![],vec![1]];
    //let rr :Graph = vec![vec![1],vec![0,2,3],vec![1,3],vec![1,2]];
    //let r :Graph = vec![vec![1],vec![0,2],vec![1]];
    //let rrr:Graph = vec![vec![1,2],vec![0],vec![0]];
    //let aa = articulation_point(&rrr);

    //let aa:Graph = vec![vec![1,2],vec![],vec![1],vec![1]];
    //let r = top_sort_kahn(&aa);
    // let ra:Graph = vec![vec![1],vec![2],vec![3],vec![1]];
    // let mut solver:SccSolver = SccSolver::new();
    // solver.tarjans_method(&ra);

    let grp:Graph = vec![vec![1],vec![2],vec![3],vec![1]];
    let mut cycle_solve:CycleDetectionSolver = CycleDetectionSolver::new();
    let val = cycle_solve.solve(&grp);
    
    println!("{val}");
    //let a = scc(&rr);
}
