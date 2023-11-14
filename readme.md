# Fault Recorder Service

- receives data points from the CMA server
- stores number of configured metrics into the database

#### Storeing following information into the API Server

- operating cycle
  - start timestamp
  - stop timestamp
  - alarm class
  - avarage load
  - max load

- operating cycle metrics
  - list of all metrics...
  - to be added...

- process metrics
  - process values
  - faults values

#### Function diagram

```mermaid
flowchart LR;
    subgraph Backend
        subgraph CMA
            cmaServer[CMA Server];
        end

        subgraph Database
            apiServer((API Server</p>));
            db[(Database)];
        end

        subgraph FaultRecorder[Fault Recorder]
            subgraph Interfaces
                cmaServerRust[CMA Server];
                cmaClient[CMA Client];
                apiClient[API Client];
                profinetClient[Profinet<br>Client];
                udtClient[UDP<br>Client];
            end

            dataCache(("Poont Queue<br>Point Pipe"));
            subgraph Task
                task1[Task<br>Operating Cycle];
                task2[Task<br>Fault Detection];
                task3[Task<br>Additional];
                faultDetectionMetrics1[Metrics];
                faultDetectionMetrics2[Metrics];
                operatingCycleMetrics1[Metrics];
                operatingCycleMetrics2[Metrics];
                additionalMetrics1[Metrics];
                additionalMetrics2[Metrics];
                faultDetectionFunctions1[Functions];
                faultDetectionFunctions2[Functions];
                operatingCycleFunctions1[Functions];
                operatingCycleFunctions2[Functions];
                additionalfunctions1[Functions];
                additionalfunctions2[Functions];
            end
        end

        db <--> apiServer;
        apiClient<--->|point|task1;
        apiClient<--->|point|task2;

        cmaServer --> cmaClient;
        cmaClient --> dataCache;
        apiClient <--> |json|apiServer;

        dataCache <--> |point| task1;
        dataCache <--> |point| task2;
        dataCache <--> |point| task3;
        task1 <--> |sql| operatingCycleMetrics1;
        task1 <--> |sql| operatingCycleMetrics2;
        task2 <--> |sql| faultDetectionMetrics1
        task2 <--> |sql| faultDetectionMetrics2
        task3 <--> |sql| additionalMetrics1
        task3 <--> |sql| additionalMetrics2
        additionalMetrics1 <--> |value| additionalfunctions1
        additionalMetrics2 <--> |value| additionalfunctions2
        faultDetectionMetrics1 <--> |value| faultDetectionFunctions1
        faultDetectionMetrics2 <--> |value| faultDetectionFunctions2
        operatingCycleMetrics1 <--> |value| operatingCycleFunctions1
        operatingCycleMetrics2 <--> |value| operatingCycleFunctions2
    end
classDef gray fill:#DCDCDC,stroke:#DCDCDC,stroke-width:2px;
classDef lightBlue fill:#D3DCFF,stroke:#DCDCDC,stroke-width:2px;
classDef green fill:#DEFFD3,stroke:#333,stroke-width:2px;
classDef orange fill:#f96,stroke:#333,stroke-width:4px;
class Backend gray
class Interfaces lightBlue
class CMA green
%% class di orange    
```

#### Configuration fo the tasks, metrics, functions

```yaml
service CmaClient:
    addres: 127.0.0.1:8881  // Self local addres
    cycle: 1 ms           // operating cycle time of the module
    auth:                   // some auth credentials
    in queue in-queue:
        max-length: 10000
    out queue: MultiQueuue.in-queue

service ApiClient:
    cycle: 1 ms
    reconnect: 1 s  # default 3 s
    address: 127.0.0.1:8080
    in queue api-link:
        max-length: 10000
    out queue: MultiQueuue.in-queue

service MultiQueue:
    in queue in-queue:
        max-length: 10000
    out queue:
        - task1.recv-queue
        - CmaClient.in-queue
        - CmaServer.in-queue

task OperatingCycle:
    cycle: 500 ms       // operating cycle time of the task
    outputQueue: operatingCycleQueue
    metrics:
        metric MetricName1:
            initial: 0      # начальное значение
            input: 
                var VarName1:
                    fn count:
                        input: 
                            - /line1/ied1/db1/Dev1.State
        metric MetricName2:
            initial: 0      # начальное значение
            input: 
                var VarName2:
                    fn timer:
                        initial: VarName1
                        input:
                            fn or:
                                input: 
                                    - /line1/ied1/db1/Dev2.State
                                    - /line1/ied1/db1/Dev3.State
                                    - /line1/ied1/db1/Dev4.State
task FaultDetection:
    cycle: 100 ms       // operating cycle time of the module
    outputQueue: operatingCycleQueue
    metrics:
        metric MetricName1:
            ...
        metric MetricName2:
            ...
```

#### Complit configuration example

<details>

```yaml
server:
    net: TCP                // TCP/UDP
    protocol:               // CMA-Json / CMA-Byte
    addres: 127.0.0.1:8882  // Self local addres
    cycle: 100 ms           // operating cycle time of the module
    in:
        queue dataCacheQueue:
            max-length: 10000
    out:
client API:
    addres: 127.0.0.1:8080  // Self local addres
    cycle: 100 ms           // operating cycle time of the module
    auth:                   // some auth credentials
    in:
        queue operatingCycleQueue:
            max-length: 10000
        queue faultDetectionQueue:
            max-length: 10000
    out:
data-cache:
    client CMA:
        addres: 127.0.0.1:8881  // Self local addres
        cycle: 100 ms           // operating cycle time of the module
        auth:                   // some auth credentials
        in:
        out:
tasks:
    task OperatingCycle:
        cycle: 500 ms       // operating cycle time of the task
        outputQueue: operatingCycleQueue
        metrics:
            metric MetricName1:
                initial: 0      # начальное значение
                input: 
                    var VarName1:
                        fn count:
                            input: 
                                - /line1/ied1/db1/Dev1.State
            metric MetricName2:
                initial: 0      # начальное значение
                input: 
                    var VarName2:
                        fn timer:
                            initial: VarName1
                            input:
                                fn or:
                                    input: 
                                        - /line1/ied1/db1/Dev2.State
                                        - /line1/ied1/db1/Dev3.State
                                        - /line1/ied1/db1/Dev4.State
    task FaultDetection:
        cycle: 100 ms       // operating cycle time of the module
        outputQueue: operatingCycleQueue
        metrics:
            metric MetricName1:
                ...
            metric MetricName2:
                ...
```

</details>
Given configuration creates following classes

```JS
inputs = {
    '/line1/ied1/db1/Dev1.State': FnInput{}
    '/line1/ied1/db1/Dev2.State': FnInput{}
    '/line1/ied1/db1/Dev3.State': FnInput{}
    '/line1/ied1/db1/Dev4.State': FnInput{}
}
outs = {
    'VarName1': FnOut{
        input: FnCount{
            input: '/line1/ied1/db1/Dev1.State'
        },
    },
    'VarName2': FnOut{
        input: FnTimer{
            input: FnOr{
                input: '/line1/ied1/db1/Dev2.State'
                input: '/line1/ied1/db1/Dev3.State'
                input: '/line1/ied1/db1/Dev4.State'
            },
        },
    },
}
metrics = {
    'MetricName1': Metric{
        id: 'MetricName1',
        input: VarName1,
    },
    'MetricName2': Metric{
        id: 'MetricName1',
        input: VarName2,
    },
}
```
