#!/bin/bash

while true; do
    ss --tcp --all | awk -v timestamp="$(date -u +"%Y-%m-%dT%H:%M:%SZ")" '
        BEGIN { established = 0; time_wait = 0; close_wait = 0; syn_sent = 0; syn_recv = 0; fin_wait1 = 0; fin_wait2 = 0; closing = 0; last_ack = 0; listen = 0 }
        
        /^ESTAB /      { established++ }
        /^TIME-WAIT /  { time_wait++ }
        /^CLOSE-WAIT / { close_wait++ }
        /^SYN-SENT /   { syn_sent++ }
        /^SYN-RECV /   { syn_recv++ }
        /^FIN-WAIT-1 / { fin_wait1++ }
        /^FIN-WAIT-2 / { fin_wait2++ }
        /^CLOSING /    { closing++ }
        /^LAST-ACK /   { last_ack++ }
        /^LISTEN /     { listen++ }
        
        END {
            printf("{\"timestamp\":\"%s\", \"established\":%7d, \"time_wait\":%7d, \"close_wait\":%7d, \"syn_sent\":%7d, \"syn_received\":%7d, \"fin_wait1\":%7d, \"fin_wait2\":%7d, \"closing\":%7d, \"last_ack\":%7d, \"listen\":%7d}\n",
                timestamp, established, time_wait, close_wait, syn_sent, syn_recv, fin_wait1, fin_wait2, closing, last_ack, listen)
        }'
    
    sleep 1
done

