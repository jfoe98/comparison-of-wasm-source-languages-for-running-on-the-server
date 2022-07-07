def filesplit(n)
    IO.foreach "./workload/numbers_#{n}.txt" do |line|
        File.open("./workload/#{line.to_i % 10}.txt", 'a+') do |io|
            io.puts line
            io.flush
        end
    end
end

filesplit(ARGV[0].to_i)