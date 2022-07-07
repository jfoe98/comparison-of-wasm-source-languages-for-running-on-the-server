def fib(n)
    if n < 2
        n
    else
        secondLast = 0
        last = 1
        fib = 0

        i = 1
        while i < n
            fib = last + secondLast
            secondLast = last
            last = fib
            i = i + 1
        end

        fib
    end
end

fib(ARGV[0].to_i)