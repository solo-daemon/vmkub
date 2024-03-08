# vmkub
Decentralized virtual machine orchaestration


Provider binary : 

```

```

```
cli <vmkub-provider> [command] : 
    [setup] : starts the openssh server and the lauch a vm on the provider also updating the on chain hashtable
```

Client binary (optional) :

```
cli <vmkub> [command] :
    [connect]: starts the best match algorithm from the seeder
    [init]: starts client regi
```
```
RPC <req> <call> :
    [payment] : triggers the on-client wallet for payment
```

Chain :
```
struct {
    ipAddress,
    RAM,
    VCPU cores,
    Storage,
}
```