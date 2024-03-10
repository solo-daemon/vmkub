// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

contract ProviderRegistry {
    struct Provider {
        address providerAddress;
        uint vcpu;
        uint ram;
        uint storageSize;
        string clientPublicKey;
        string ipAddress;
        uint timestamp; // Timestamp of registration
    }

    mapping(address => Provider) private providers;
    address[] private providerAddresses;
    uint private lastQueryTimestamp; // Timestamp of the last query

    event NoNewProviderFound();
    event ClientPublicKeyNotAssigned(address indexed provider);

    // Function to register a provider
    function registerProvider(
        uint _vcpu,
        uint _ram,
        uint _storageSize,
        string memory _ipAddress
    ) public {
        require(providers[msg.sender].timestamp == 0, "Provider already registered");

        providers[msg.sender] = Provider({
            providerAddress: msg.sender,
            vcpu: _vcpu,
            ram: _ram,
            storageSize: _storageSize,
            clientPublicKey: "",
            ipAddress: _ipAddress,
            timestamp: block.timestamp // Store current block timestamp
        });
        providerAddresses.push(msg.sender); // Add provider address to the array
    }

    // Function to retrieve provider information along with client public key for providers registered after the last query
    function getProviderInformation() public returns (Provider[] memory) {
        uint count = 0;
        for(uint i=0; i<providerAddresses.length; i++) {
            if(providers[providerAddresses[i]].timestamp > lastQueryTimestamp) {
                count++;
            }
        }

        bool updatedLastQueryTimestamp = false; // Flag to track if lastQueryTimestamp was updated

        address[] memory addresses = new address[](count);
        Provider[] memory data = new Provider[](count);

        uint index=0;
        for (uint i = 0; i < providerAddresses.length; i++) {
            if (providers[providerAddresses[i]].timestamp > lastQueryTimestamp) {
                 addresses[index] = providerAddresses[i];
                 data[index] = providers[providerAddresses[i]];
                 updatedLastQueryTimestamp=true;
            } 
        }

        // Update the last query timestamp if at least one provider was found
        if (updatedLastQueryTimestamp) {
            lastQueryTimestamp = block.timestamp;
        } else {
            emit NoNewProviderFound(); // Emit event if no new provider was found
        }

        return (data);
    }

    // Function to retrieve the client public key of a provider
    function getClientPublicKey(address _provider) public returns (string memory) {
        require(providers[_provider].timestamp != 0, "Provider not registered");

        string memory clientPublicKey = providers[_provider].clientPublicKey;
        if (bytes(clientPublicKey).length == 0) {
            emit ClientPublicKeyNotAssigned(_provider); // Emit event if client public key is not assigned
        }
        
        return clientPublicKey;
    }

    // Function to update the client public key of a provider
    function updateClientPublicKey(address _provider, string memory _clientPublicKey) public {
        require(providers[_provider].timestamp != 0, "Provider not registered");

        providers[_provider].clientPublicKey = _clientPublicKey;
    }
}
