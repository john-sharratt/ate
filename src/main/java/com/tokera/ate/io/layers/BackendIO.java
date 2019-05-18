package com.tokera.ate.io.layers;

import com.tokera.ate.dao.base.BaseDao;
import com.tokera.ate.dto.msg.*;
import com.tokera.ate.io.api.IAteIO;
import com.tokera.ate.units.DaoId;
import com.tokera.ate.units.Hash;
import com.tokera.ate.io.repo.DataContainer;
import com.tokera.ate.io.repo.DataRepository;
import com.tokera.ate.io.repo.DataSubscriber;
import org.checkerframework.checker.nullness.qual.Nullable;

import java.util.Collection;
import java.util.List;
import java.util.Set;
import java.util.UUID;

/**
 * IO implementation that simple passes through IO commands to the next IO module with built in callbacks
 */
final public class BackendIO implements IAteIO {

    private final DataRepository next;
    private final DataSubscriber backend;

    public BackendIO(DataRepository next, DataSubscriber backend) {
        this.next = next;
        this.backend = backend;
    }

    @Override
    public boolean merge(BaseDao t) {
        return next.merge(t);
    }

    @Override
    public boolean mergeAsync(BaseDao t) {
        return next.mergeAsync(t);
    }

    @Override
    public boolean mergeWithoutValidation(BaseDao t) {
        return next.mergeWithoutValidation(t);
    }

    @Override
    public boolean mergeAsyncWithoutValidation(BaseDao t) {
        return next.mergeAsyncWithoutValidation(t);
    }

    @Override
    public boolean merge(MessagePublicKeyDto t) {
        return next.merge(t);
    }

    @Override
    public boolean merge(MessageEncryptTextDto t) {
        return next.merge(t);
    }
    
    @Override
    public void mergeLater(BaseDao t) {
        next.mergeLater(t);
    }

    @Override
    public void mergeLaterWithoutValidation(BaseDao t) {
        next.mergeLaterWithoutValidation(t);
    }
    
    @Override
    public void mergeDeferred() {
        next.mergeDeferred();
    }
    
    @Override
    public void clearDeferred() {
        next.clearDeferred();
    }
    
    @Override
    public void clearCache(@DaoId UUID id) {
        next.clearCache(id);
    }

    @Override
    public boolean remove(BaseDao t) {
        return next.remove(t);
    }
    
    @Override
    public void removeLater(BaseDao t) {
        next.removeLater(t);
    }
    
    @Override
    public boolean remove(@DaoId UUID id, Class<?> type) {
        return next.remove(id, type);
    }

    @Override
    public void cache(BaseDao entity) {
        next.cache(entity);
    }

    @Override
    public void decache(BaseDao entity) {
        next.decache(entity);
    }

    @Override
    public @Nullable MessageDataHeaderDto getRootOfTrust(UUID id) {
        return next.getRootOfTrust(id);
    }
    
    @Override
    public void warm() {
        next.warm();
    }

    @Override
    public void sync() {
        next.sync();
    }

    @Override
    public boolean sync(MessageSyncDto sync) {
        return next.sync(sync);
    }

    @Override
    public @Nullable MessagePublicKeyDto publicKeyOrNull(@Hash String hash) {
        return next.publicKeyOrNull(hash);
    }
    
    @Override
    public boolean exists(@Nullable @DaoId UUID id) {
        return next.exists(id);
    }
    
    @Override
    public boolean ethereal() {
        return next.ethereal();
    }
    
    @Override
    public boolean everExisted(@Nullable @DaoId UUID id) {
        return next.everExisted(id);
    }
    
    @Override
    public boolean immutable(@DaoId UUID id) {
        return next.immutable(id);
    }

    @Override
    public @Nullable BaseDao getOrNull(@DaoId UUID id) {
        return next.getOrNull(id);
    }

    @Override
    public @Nullable DataContainer getRawOrNull(@DaoId UUID id) {
        return next.getRawOrNull(id);
    }
    
    @Override
    public @Nullable BaseDao getVersionOrNull(@DaoId UUID id, MessageMetaDto meta) {
        return next.getVersionOrNull(id, meta);
    }
    
    @Override
    public @Nullable MessageDataDto getVersionMsgOrNull(@DaoId UUID id, MessageMetaDto meta) {
        return next.getVersionMsgOrNull(id, meta);
    }
    
    @Override
    public <T extends BaseDao> Iterable<MessageMetaDto> getHistory(@DaoId UUID id, Class<T> clazz) {
        return next.getHistory(id, clazz);
    }

    @Override
    public Set<BaseDao> getAll() {
        return next.getAll();
    }

    @Override
    public <T extends BaseDao> Set<T> getAll(Class<T> type) {
        return next.getAll(type);
    }

    @Override
    public <T extends BaseDao> List<DataContainer> getAllRaw() {
        return next.getAllRaw();
    }

    @Override
    public <T extends BaseDao> List<DataContainer> getAllRaw(Class<T> type) {
        return next.getAllRaw(type);
    }
    
    @Override
    public <T extends BaseDao> List<T> getMany(Collection<@DaoId UUID> ids, Class<T> type) {
        return next.getMany(ids, type);
    }

    @Override
    public DataSubscriber backend() {
        return this.backend;
    }
}