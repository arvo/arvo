```
fn main() void -> {

    object Queue A {
        list [A],
        top usize,
        free usize,
        empty bool,

        expose fn(queue mut ref Queue A) enqueue(x A) void -> when !queue.isFull() {
            let free := queue.free;
            queue.list [free] := x;
            queue.free := (free + 1) mod queue.list.size();
            queue.empty := false;
        }

        expose fn(queue mut ref Queue A) dequeue() A -> when !queue.isEmpty() {
            let top := queue.top;
            let item := queue.list [top];
            queue.top := (top + 1) mod queue.list.size();
            queue.empty := ((top + 1) mode queue.list.size()) = queue.free;
            item
        }

        expose fn(queue ref Queue A) isEmpty() bool ->
            queue.empty

        expose fn(queue ref Queue A) isFull() bool ->
            !queue.isEmpty() && (queue.top = queue.free)

    }

    let mut queue ~Queue := ~Queue.new();
    let mut num i64 := 100;
    let num_producers i64 := 5;
    let num_consumers i64 := 3;

    {
        fn produce(num mut ref usize) bool -> {
            if deref num = 0 {
                false
            } else {
                queue.enqueue(num++);
            }
        }

        fn producer() void -> {
            let mut num := num / num_producers;
            loop produce(mut ref num);
        }

        forall i in 1 .. num_producers {
            producer();
        }
    }

    {
        fn consume(num mut ref usize) bool ->
            if deref num = 0 {
                false
            } else {
                writeln(queue.dequeue());
                num--;
                true
            }

        fn consumer() void -> {
            loop consume(mut ref num);
        }

        forall i in 1 .. num_consumers {
            consumer();
        }
    }
}
```