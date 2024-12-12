# netstat_continuous

This command line application continuously emits statistics about the currently active TCP network connections, i.e. how many TCP connections are established, how many sockets are in listening mode, and how many are in status like TIME_WAIT etc:

## Install

```shell
REPO=chgeuer/netstat_continuous
TAG=0.1.1
PLATFORM=linux-x64
BINARY=netstat_continuous-${PLATFORM}

curl -LfsSO https://github.com/${REPO}/releases/download/v${TAG}/${BINARY}
mv ./${BINARY} ./netstat_continuous
chmod +x ./netstat_continuous
```

## Sample run

```shell
$ ./netstat_continuous-linux-x64

{"timestamp":"2024-12-12T10:30:25Z", "established":     10, "time_wait":     50, "close_wait":      0, "syn_sent":      0, "syn_received":      0, "fin_wait1":      0, "fin_wait2":      0, "closing":      0, "last_ack":      0, "listen":      5}
{"timestamp":"2024-12-12T10:30:26Z", "established":     10, "time_wait":     50, "close_wait":      0, "syn_sent":      0, "syn_received":      0, "fin_wait1":      0, "fin_wait2":      0, "closing":      0, "last_ack":      0, "listen":      5}
{"timestamp":"2024-12-12T10:30:27Z", "established":     11, "time_wait":     50, "close_wait":      0, "syn_sent":      0, "syn_received":      0, "fin_wait1":      0, "fin_wait2":      0, "closing":      0, "last_ack":      0, "listen":      5}
{"timestamp":"2024-12-12T10:30:28Z", "established":     10, "time_wait":     51, "close_wait":      0, "syn_sent":      1, "syn_received":      0, "fin_wait1":      1, "fin_wait2":      0, "closing":      0, "last_ack":      0, "listen":      5}
{"timestamp":"2024-12-12T10:30:29Z", "established":     11, "time_wait":     52, "close_wait":      0, "syn_sent":      0, "syn_received":      0, "fin_wait1":      0, "fin_wait2":      0, "closing":      0, "last_ack":      0, "listen":      5}
{"timestamp":"2024-12-12T10:30:30Z", "established":     10, "time_wait":     50, "close_wait":      0, "syn_sent":      0, "syn_received":      0, "fin_wait1":      0, "fin_wait2":      0, "closing":      0, "last_ack":      0, "listen":      5}
^C
```

