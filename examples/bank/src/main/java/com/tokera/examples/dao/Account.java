package com.tokera.examples.dao;

import com.tokera.ate.annotations.PermitParentFree;
import com.tokera.ate.common.ImmutalizableArrayList;
import com.tokera.ate.dao.base.BaseDaoRights;
import com.tokera.ate.units.DaoId;
import org.checkerframework.checker.nullness.qual.Nullable;

import javax.enterprise.context.Dependent;
import java.util.UUID;

@Dependent
@PermitParentFree
public class Account extends BaseDaoRights {
    public UUID id;
    public String name;
    @Nullable
    public String domain;
    public final ImmutalizableArrayList<UUID> monthlyActivities = new ImmutalizableArrayList<UUID>();
    public final ImmutalizableArrayList<UUID> individuals = new ImmutalizableArrayList<UUID>();
    public final ImmutalizableArrayList<UUID> accountRoles = new ImmutalizableArrayList<UUID>();

    @SuppressWarnings("initialization.fields.uninitialized")
    @Deprecated
    public Account() {
    }

    public Account(String name) {
        this.id = UUID.randomUUID();
        this.name = name;
    }

    public @DaoId UUID getId() {
        return this.id;
    }

    public @Nullable @DaoId UUID getParentId() {
        return null;
    }
}
