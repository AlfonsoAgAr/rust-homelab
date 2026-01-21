To deploy we use 

```
cargo build --release
```

The command create an executable at "../target/release/" and is  faster than debug version.

We can use cron to execute it.

First I installed via apt:

```
apt update && apt install cron -y
```

Must be init with:

```
/etc/init.d/cron start
```

Next is neccesary schedule:
```
crontab -e
```

With this parameters:
```
* * * * * /path/to/project/target/release/project >> ~/cron.log 2>&1
```


If is necessary add this to ~/.bashrc:
```
service cron start
```

:w
