package com.tokera.ate.test.storage;

import com.tokera.ate.delegates.AteDelegate;
import org.jboss.weld.bootstrap.spi.BeanDiscoveryMode;
import org.jboss.weld.environment.se.Weld;
import org.jboss.weld.junit5.WeldInitiator;
import org.jboss.weld.junit5.WeldJunit5Extension;
import org.jboss.weld.junit5.WeldSetup;
import org.junit.jupiter.api.BeforeAll;
import org.junit.jupiter.api.Test;
import org.junit.jupiter.api.TestInstance;
import org.junit.jupiter.api.extension.ExtendWith;

import javax.enterprise.context.RequestScoped;

@ExtendWith(WeldJunit5Extension.class)
@TestInstance(TestInstance.Lifecycle.PER_CLASS)
public class FactoryTests {

    @WeldSetup
    public WeldInitiator weld = WeldInitiator
            .from(new Weld()
                    .setBeanDiscoveryMode(BeanDiscoveryMode.ANNOTATED)
                    .enableDiscovery())
            .activate(RequestScoped.class)
            .build();

    @BeforeAll
    public void init() {
        AteDelegate d = AteDelegate.get();

        // Build the default storage subsystem
        d.storageFactory.buildKafkaBackend()
                .addCacheLayer()
                .addAccessLoggerLayer();
    }

    @Test
    public void testBackend() {
        AteDelegate d = AteDelegate.get();
        d.dataRepository.backend().touch();
    }
}