package channels

func Pipe[T any, K any](in <-chan T, f func(T) K) <-chan K {
	out := make(chan K)
	go func() {
		defer close(out)
		for n := range in {
			out <- f(n)
		}
	}()
	return out
}
