def fib(n)
    if n<2
        n
    else
        fib(n-2)+fib(n-1)
    end
end

fib(ARGV[0].to_i)