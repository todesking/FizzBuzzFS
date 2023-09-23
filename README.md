# FizzBuzzFS

```shellsession
root@8a2db3fc6292:/# cd /mnt/FizzBuzz/
root@8a2db3fc6292:/mnt/FizzBuzz# ls -l
total 9007199254740992
-rw-r--r-- 1 501 dialout 9223372036854775807 Jan  1  1970 FizzBuzz.txt
root@8a2db3fc6292:/mnt/FizzBuzz# ls -lh
total 8.0E
-rw-r--r-- 1 501 dialout 8.0E Jan  1  1970 FizzBuzz.txt
root@8a2db3fc6292:/mnt/FizzBuzz# head FizzBuzz.txt
1
2
Fizz
4
Buzz
Fizz
7
8
Fizz
Buzz
root@8a2db3fc6292:/mnt/FizzBuzz# dd if=FizzBuzz.txt ibs=512 count=1 skip=999999
1
Fizz
69989923
69989924
FizzBuzz
69989926
69989927
Fizz
69989929
Buzz
Fizz
69989932
69989933
Fizz
Buzz
69989936
Fizz
69989938
69989939
FizzBuzz
69989941
69989942
```

## Run

```
docker build -t fizz_buzz_fs .
docker run -it --cap-add SYS_ADMIN --device /dev/fuse fizzbuzzfs
```
