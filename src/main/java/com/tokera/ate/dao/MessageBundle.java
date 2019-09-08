package com.tokera.ate.dao;

import com.tokera.ate.dao.msg.MessageBase;

public class MessageBundle {
    public final int partition;
    public final long offset;
    public final MessageBase raw;

    public MessageBundle(int partition, long offset, MessageBase raw) {
        this.partition = partition;
        this.offset = offset;
        this.raw = raw;
    }
}