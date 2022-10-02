// SPDX-License-Identifier: GPL-2.0 OR BSD-3-Clause
/* Copyright (c) 2020 Facebook */
#include <linux/bpf.h>
#include <bpf/bpf_helpers.h>

char LICENSE[] SEC("license") = "Dual BSD/GPL";

int my_pid = 0;

SEC("tracepoint/block/block_rq_issue")
int handle_tp(void *ctx)
{

	bpf_printk("block_rq_issue hello");

	return 0;
}
