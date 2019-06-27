package com.tokera.ate.io.task;

import com.tokera.ate.dao.base.BaseDao;
import com.tokera.ate.delegates.AteDelegate;
import com.tokera.ate.dto.TokenDto;
import com.tokera.ate.dto.msg.MessageDataMetaDto;
import com.tokera.ate.io.api.*;
import org.checkerframework.checker.nullness.qual.Nullable;

import java.util.LinkedList;
import java.util.List;
import java.util.Map;
import java.util.TreeMap;
import java.util.stream.Collectors;

/**
 * Represents a partition and class context that callbacks will be invoked under
 */
public class TaskContext<T extends BaseDao> implements ITaskContext {
    AteDelegate d = AteDelegate.get();

    public final IPartitionKey partitionKey;
    public final Class<T> clazz;
    public final List<Task<T>> tasks;

    public TaskContext(IPartitionKey partitionKey, Class<T> clazz) {
        this.partitionKey = partitionKey;
        this.clazz = clazz;
        this.tasks = new LinkedList<>();
    }

    @Override
    public IPartitionKey partitionKey() {
        return this.partitionKey;
    }

    @Override
    @SuppressWarnings("unchecked")
    public <A extends BaseDao> ITask addTask(ITaskCallback<A> callback, Class<A> clazz, @Nullable TokenDto token) {
        AteDelegate d = AteDelegate.get();

        if (this.clazz != clazz) {
            throw new RuntimeException("Clazz type of the callback must match.");
        }

        Task processorContext;
        synchronized (tasks) {
            processorContext = tasks.stream().filter(p -> p.callback == callback).findFirst().orElse(null);
            if (processorContext != null) return processorContext;
        }

        // Add the processor to the subscription list
        processorContext = new Task(this, clazz, callback, token);
        synchronized (tasks) {
            this.tasks.add(processorContext);
        }

        processorContext.start();
        return processorContext;
    }

    @Override
    public boolean removeTask(ITask task) {
        Task<T> ret;
        synchronized (tasks) {
            ret = this.tasks.stream().filter(t -> t == task).findFirst().orElse(null);
            if (ret == null) return false;
        }
        ret.stop();
        return true;
    }

    @Override
    public void feed(MessageDataMetaDto msg) {
        synchronized (tasks) {
            for (Task<T> context : this.tasks) {
                context.add(msg);
            }
        }
    }

    @Override
    public List<ITask> tasks() {
        return this.tasks.stream()
                .collect(Collectors.toList());
    }
}