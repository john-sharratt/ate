/*
 * To change this license header, choose License Headers in Project Properties.
 * To change this template file, choose Tools | Templates
 * and open the template in the editor.
 */
package com.tokera.ate.dao.base;

import com.fasterxml.jackson.annotation.JsonProperty;
import com.tokera.ate.common.Immutalizable;
import com.tokera.ate.common.ImmutalizableHashSet;
import com.tokera.ate.dao.IRights;
import com.tokera.ate.dao.IRoles;
import com.tokera.ate.dto.PrivateKeyWithSeedDto;
import com.tokera.ate.dto.msg.MessagePrivateKeyDto;
import com.tokera.ate.units.Alias;

import java.util.Set;

/**
 * Represents the common fields and methods of all data objects that are stored in the ATE data-store
 * plus it holds holds access rights to different read and write roles throughout the data model.
 * plus a set of user-defined key-value parameters that can be associated with the data object
 * If a user is able to read this record then they are able to gain access to the things that it has access to
 */
public abstract class BaseDaoRolesRights extends BaseDaoRoles implements IRights, Immutalizable
{
    @JsonProperty
    public final ImmutalizableHashSet<PrivateKeyWithSeedDto> rightsRead = new ImmutalizableHashSet<>();
    @JsonProperty
    public final ImmutalizableHashSet<PrivateKeyWithSeedDto> rightsWrite = new ImmutalizableHashSet<>();
    
    @Override
    public Set<PrivateKeyWithSeedDto> getRightsRead() {
        return rightsRead;
    }

    @Override
    public Set<PrivateKeyWithSeedDto> getRightsWrite() {
        return rightsWrite;
    }

    @Override
    public @Alias String getRightsAlias() {
        return this.getId().toString();
    }

    @Override
    public void onAddRight(IRoles to) {
    }

    @Override
    public void onRemoveRight(IRoles from) {
    }

    @Override
    public void immutalize() {
        super.immutalize();
        this.rightsRead.immutalize();
        this.rightsWrite.immutalize();
    }
}
