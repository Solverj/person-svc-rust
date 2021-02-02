# person-svc-rust
solve@solve ~/git-repositories/Hobby/rust/person-svc-rust (master) $ ab -n 100000 -c 100 http://localhost:8080/
This is ApacheBench, Version 2.3 <$Revision: 1843412 $>
Copyright 1996 Adam Twiss, Zeus Technology Ltd, http://www.zeustech.net/
Licensed to The Apache Software Foundation, http://www.apache.org/

Benchmarking localhost (be patient)
Completed 10000 requests
Completed 20000 requests
Completed 30000 requests
Completed 40000 requests
Completed 50000 requests
Completed 60000 requests
Completed 70000 requests
Completed 80000 requests
Completed 90000 requests
Completed 100000 requests
Finished 100000 requests


Server Software:        
Server Hostname:        localhost
Server Port:            8080

Document Path:          /
Document Length:        16 bytes

Concurrency Level:      100
Time taken for tests:   6.868 seconds
Complete requests:      100000
Failed requests:        0
Total transferred:      13300000 bytes
HTML transferred:       1600000 bytes
Requests per second:    14560.97 [#/sec] (mean)
Time per request:       6.868 [ms] (mean)
Time per request:       0.069 [ms] (mean, across all concurrent requests)
Transfer rate:          1891.22 [Kbytes/sec] received

Connection Times (ms)
              min  mean[+/-sd] median   max
Connect:        0    3   0.5      3      13
Processing:     1    4   0.6      3      14
Waiting:        1    3   0.6      3       9
Total:          4    7   0.6      7      19
WARNING: The median and mean for the processing time are not within a normal deviation
        These results are probably not that reliable.

Percentage of the requests served within a certain time (ms)
  50%      7
  66%      7
  75%      7
  80%      7
  90%      7
  95%      8
  98%      8
  99%      8
 100%     19 (longest request)
solve@solve ~/git-repositories/Hobby/rust/person-svc-rust (master) $ ab -n 100000 -c 100 http://localhost:8080/
This is ApacheBench, Version 2.3 <$Revision: 1843412 $>
Copyright 1996 Adam Twiss, Zeus Technology Ltd, http://www.zeustech.net/
Licensed to The Apache Software Foundation, http://www.apache.org/

Benchmarking localhost (be patient)
Completed 10000 requests
Completed 20000 requests
Completed 30000 requests
Completed 40000 requests
Completed 50000 requests
Completed 60000 requests
Completed 70000 requests
Completed 80000 requests
Completed 90000 requests
Completed 100000 requests
Finished 100000 requests


Server Software:        
Server Hostname:        localhost
Server Port:            8080

Document Path:          /
Document Length:        16 bytes

Concurrency Level:      100
Time taken for tests:   6.776 seconds
Complete requests:      100000
Failed requests:        0
Total transferred:      13300000 bytes
HTML transferred:       1600000 bytes
Requests per second:    14758.42 [#/sec] (mean)
Time per request:       6.776 [ms] (mean)
Time per request:       0.068 [ms] (mean, across all concurrent requests)
Transfer rate:          1916.86 [Kbytes/sec] received

Connection Times (ms)
              min  mean[+/-sd] median   max
Connect:        0    3   0.6      3      14
Processing:     0    4   0.8      3      16
Waiting:        0    3   0.7      2      16
Total:          0    7   0.7      7      21
WARNING: The median and mean for the processing time are not within a normal deviation
        These results are probably not that reliable.
WARNING: The median and mean for the waiting time are not within a normal deviation
        These results are probably not that reliable.

Percentage of the requests served within a certain time (ms)
  50%      7
  66%      7
  75%      7
  80%      7
  90%      7
  95%      8
  98%      8
  99%      8
 100%     21 (longest request)
solve@solve ~/git-repositories/Hobby/rust/person-svc-rust (master) $ ab -n 100000 -c 100 http://localhost:8080/
This is ApacheBench, Version 2.3 <$Revision: 1843412 $>
Copyright 1996 Adam Twiss, Zeus Technology Ltd, http://www.zeustech.net/
Licensed to The Apache Software Foundation, http://www.apache.org/

Benchmarking localhost (be patient)
Completed 10000 requests
Completed 20000 requests
Completed 30000 requests
Completed 40000 requests
Completed 50000 requests
Completed 60000 requests
Completed 70000 requests
Completed 80000 requests
Completed 90000 requests
Completed 100000 requests
Finished 100000 requests


Server Software:        
Server Hostname:        localhost
Server Port:            8080

Document Path:          /
Document Length:        16 bytes

Concurrency Level:      100
Time taken for tests:   6.792 seconds
Complete requests:      100000
Failed requests:        0
Total transferred:      13300000 bytes
HTML transferred:       1600000 bytes
Requests per second:    14723.19 [#/sec] (mean)
Time per request:       6.792 [ms] (mean)
Time per request:       0.068 [ms] (mean, across all concurrent requests)
Transfer rate:          1912.29 [Kbytes/sec] received

Connection Times (ms)
              min  mean[+/-sd] median   max
Connect:        0    3   0.6      3      15
Processing:     0    4   0.8      3      16
Waiting:        0    3   0.8      2      15
Total:          0    7   0.8      7      25
WARNING: The median and mean for the processing time are not within a normal deviation
        These results are probably not that reliable.
WARNING: The median and mean for the waiting time are not within a normal deviation
        These results are probably not that reliable.

Percentage of the requests served within a certain time (ms)
  50%      7
  66%      7
  75%      7
  80%      7
  90%      7
  95%      8
  98%      8
  99%      9
 100%     25 (longest request)
solve@solve ~/git-repositories/Hobby/rust/person-svc-rust (master) $ ab -n 100000 -c 100 http://localhost:8080/persons/
This is ApacheBench, Version 2.3 <$Revision: 1843412 $>
Copyright 1996 Adam Twiss, Zeus Technology Ltd, http://www.zeustech.net/
Licensed to The Apache Software Foundation, http://www.apache.org/

Benchmarking localhost (be patient)
Completed 10000 requests
Completed 20000 requests
Completed 30000 requests
Completed 40000 requests
Completed 50000 requests
Completed 60000 requests
Completed 70000 requests
Completed 80000 requests
Completed 90000 requests
Completed 100000 requests
Finished 100000 requests


Server Software:        
Server Hostname:        localhost
Server Port:            8080

Document Path:          /persons/
Document Length:        0 bytes

Concurrency Level:      100
Time taken for tests:   6.787 seconds
Complete requests:      100000
Failed requests:        0
Non-2xx responses:      100000
Total transferred:      8200000 bytes
HTML transferred:       0 bytes
Requests per second:    14733.92 [#/sec] (mean)
Time per request:       6.787 [ms] (mean)
Time per request:       0.068 [ms] (mean, across all concurrent requests)
Transfer rate:          1179.87 [Kbytes/sec] received

Connection Times (ms)
              min  mean[+/-sd] median   max
Connect:        0    3   0.6      3      16
Processing:     0    4   0.8      3      16
Waiting:        0    3   0.7      2      13
Total:          0    7   0.7      7      20
WARNING: The median and mean for the processing time are not within a normal deviation
        These results are probably not that reliable.
WARNING: The median and mean for the waiting time are not within a normal deviation
        These results are probably not that reliable.

Percentage of the requests served within a certain time (ms)
  50%      7
  66%      7
  75%      7
  80%      7
  90%      7
  95%      8
  98%      8
  99%      9
 100%     20 (longest request)
solve@solve ~/git-repositories/Hobby/rust/person-svc-rust (master) $ ab -n 100000 -c 100 http://localhost:8080/persons
This is ApacheBench, Version 2.3 <$Revision: 1843412 $>
Copyright 1996 Adam Twiss, Zeus Technology Ltd, http://www.zeustech.net/
Licensed to The Apache Software Foundation, http://www.apache.org/

Benchmarking localhost (be patient)
Completed 10000 requests
Completed 20000 requests
Completed 30000 requests
Completed 40000 requests
Completed 50000 requests
Completed 60000 requests
Completed 70000 requests
Completed 80000 requests
Completed 90000 requests
Completed 100000 requests
Finished 100000 requests


Server Software:        
Server Hostname:        localhost
Server Port:            8080

Document Path:          /persons
Document Length:        282 bytes

Concurrency Level:      100
Time taken for tests:   14.071 seconds
Complete requests:      100000
Failed requests:        0
Total transferred:      39100000 bytes
HTML transferred:       28200000 bytes
Requests per second:    7106.84 [#/sec] (mean)
Time per request:       14.071 [ms] (mean)
Time per request:       0.141 [ms] (mean, across all concurrent requests)
Transfer rate:          2713.65 [Kbytes/sec] received

Connection Times (ms)
              min  mean[+/-sd] median   max
Connect:        0    3   1.7      3      13
Processing:     1   11   3.6     10      99
Waiting:        1   10   3.6      9      97
Total:          3   14   3.2     14     108

Percentage of the requests served within a certain time (ms)
  50%     14
  66%     15
  75%     16
  80%     16
  90%     18
  95%     19
  98%     21
  99%     23
 100%    108 (longest request)
