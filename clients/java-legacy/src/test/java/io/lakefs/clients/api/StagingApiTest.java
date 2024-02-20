/*
 * lakeFS API
 * lakeFS HTTP API
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 *
 * NOTE: This class is auto generated by OpenAPI Generator (https://openapi-generator.tech).
 * https://openapi-generator.tech
 * Do not edit the class manually.
 */


package io.lakefs.clients.api;

import io.lakefs.clients.api.ApiException;
import io.lakefs.clients.api.model.Error;
import io.lakefs.clients.api.model.ObjectStats;
import io.lakefs.clients.api.model.StagingLocation;
import io.lakefs.clients.api.model.StagingMetadata;
import org.junit.Test;
import org.junit.Ignore;

import java.util.ArrayList;
import java.util.HashMap;
import java.util.List;
import java.util.Map;

/**
 * API tests for StagingApi
 */
@Ignore
public class StagingApiTest {

    private final StagingApi api = new StagingApi();

    
    /**
     * generate an address to which the client can upload an object
     *
     * 
     *
     * @throws ApiException
     *          if the Api call fails
     */
    @Test
    public void getPhysicalAddressTest() throws ApiException {
        String repository = null;
        String branch = null;
        String path = null;
        Boolean presign = null;
                StagingLocation response = api.getPhysicalAddress(repository, branch, path, presign);
        // TODO: test validations
    }
    
    /**
     * associate staging on this physical address with a path
     *
     * Link the physical address with the path in lakeFS, creating an uncommitted change. The given address can be one generated by getPhysicalAddress, or an address outside the repository&#39;s storage namespace. 
     *
     * @throws ApiException
     *          if the Api call fails
     */
    @Test
    public void linkPhysicalAddressTest() throws ApiException {
        String repository = null;
        String branch = null;
        String path = null;
        StagingMetadata stagingMetadata = null;
        String ifNoneMatch = null;
                ObjectStats response = api.linkPhysicalAddress(repository, branch, path, stagingMetadata, ifNoneMatch);
        // TODO: test validations
    }
    
}
