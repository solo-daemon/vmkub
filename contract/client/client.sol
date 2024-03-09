// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

contract ClientDataRegistry {
    struct ClientData {
        address clientAdress;
        uint vcpu;
        uint ram;
        uint storageSize;
        string rsaPublicKey;
        string providerIPAddress;
        uint timestamp;
    }

    mapping(address => ClientData) private clients;
    address[] private clientAddresses;
    uint private lastQueryTimestamp;

    event NoNewClientFound();
    event ProviderNotAssigned(address indexed client);

    // Function to register client data
    function registerClient(
        uint _vcpu,
        uint _ram,
        uint _storageSize,
        string memory _rsaPublicKey
    ) public {
        require(clients[msg.sender].timestamp == 0, "Client already registered");

        clients[msg.sender] = ClientData({
            clientAdress: msg.sender,
            vcpu: _vcpu,
            ram: _ram,
            storageSize: _storageSize,
            rsaPublicKey: _rsaPublicKey,
            providerIPAddress: "",
            timestamp: block.timestamp
        });
        clientAddresses.push(msg.sender);
    }

    // Function to retrieve all client data along with provider information for clients registered after the last query
    function getClientData() public returns (ClientData[] memory) {
        uint count = 0;
        for (uint i = 0; i < clientAddresses.length; i++) {
            if (clients[clientAddresses[i]].timestamp > lastQueryTimestamp) {
                count++;
            }
        }

        bool updatedLastQueryTimestamp = false; // Flag to track if lastQueryTimestamp was updated

        address[] memory addresses = new address[](count);
        ClientData[] memory data = new ClientData[](count);

        uint index = 0;
        for (uint i = 0; i < clientAddresses.length; i++) {
            if (clients[clientAddresses[i]].timestamp > lastQueryTimestamp) {
                addresses[index] = clientAddresses[i];
                data[index] = clients[clientAddresses[i]];
                updatedLastQueryTimestamp = true; // Set flag to true if at least one client was found
                index++;
            }
        }
        if (updatedLastQueryTimestamp) {
                    lastQueryTimestamp = block.timestamp;
                } else {
                    emit NoNewClientFound(); // Emit event if no new client was found
                }
        return (data);
    }

    // Function to retrieve the IP address of a client
    function getProviderIPAddress(address _client) public returns (string memory) {
        require(clients[_client].timestamp != 0, "Client not registered");

        string memory ipAddress = clients[_client].providerIPAddress;
        if (bytes(ipAddress).length == 0) {
            emit ProviderNotAssigned(_client);
        }
        
        return ipAddress;
    }

    // Function to update the IP address of a client
    function updateProviderIPAddress(address _client, string memory _ipAddress) public {
        require(clients[_client].timestamp != 0, "Client not registered");

        clients[_client].providerIPAddress = _ipAddress;
    }
}
