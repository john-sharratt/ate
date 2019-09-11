package com.tokera.ate.io.ram;

import com.tokera.ate.dao.GenericPartitionKey;
import com.tokera.ate.delegates.AteDelegate;
import com.tokera.ate.io.api.IPartitionKey;
import com.tokera.ate.io.kafka.KafkaTopicFactory;
import com.tokera.ate.io.repo.DataPartitionChain;
import com.tokera.ate.io.repo.IDataPartitionBridge;

import javax.enterprise.context.ApplicationScoped;
import javax.ws.rs.WebApplicationException;

@ApplicationScoped
public class RamBridgeBuilder {
    private final AteDelegate d = AteDelegate.get();

    public IDataPartitionBridge createPartition(IPartitionKey key) {
        if (key.partitionIndex() >= KafkaTopicFactory.maxPartitionsPerTopic) {
            throw new WebApplicationException("Partition index can not exceed the maximum of " + KafkaTopicFactory.maxPartitionsPerTopic + " per topic.");
        }

        GenericPartitionKey wrapKey = new GenericPartitionKey(key);
        DataPartitionChain chain = new DataPartitionChain(key);
        RamPartitionBridge ret = new RamPartitionBridge(chain, key.partitionType());

        ret.feed(d.ramDataRepository.read(wrapKey));
        return ret;
    }

    public void removePartition(IPartitionKey key) {
    }
}
