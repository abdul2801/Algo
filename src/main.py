class NestedIterator:
   
    def __init__(self, nestedList):
        s = []
        self.push(nestedList)

    def push(self,l):
        for i in reversed(range(0,len(l))):
            self.s.append(l[i])
    
    def next(self) -> int:
        self.hasNext()
        t = self.s.pop().getInteger();
        return t
        

        
    
    def hasNext(self) -> bool:
        while len(self.s) != 0:
            if self.s[-1].isInteger():
                return True
            else:
                x =self.s.pop()
                self.push(x)
        return False



         