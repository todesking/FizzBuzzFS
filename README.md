# FizzBuzzFS

## Run

```
docker build -t fizz_buzz_fs .
docker run -it --cap-add SYS_ADMIN --device /dev/fuse fizzbuzzfs
```
