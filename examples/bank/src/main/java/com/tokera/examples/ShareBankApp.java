package com.tokera.examples;

import com.tokera.ate.ApiServer;
import com.tokera.ate.BootstrapApp;
import com.tokera.ate.BootstrapConfig;

import javax.ws.rs.ApplicationPath;

@ApplicationPath("1-0")
public class ShareBankApp extends BootstrapApp {

    public ShareBankApp() { }

    public static void main(String[] args) {
        BootstrapConfig config = ApiServer.startWeld(args, ShareBankApp.class);
        config.setDeploymentName("ShareBank");
        config.setLoggingChainOfTrust(true);
        config.setLoggingWrites(true);

        ApiServer.startApiServer(config);
    }
}