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
flowchart TD;
   clients[Client];
   server[TCP / UDP<br>Server];
   dataCache((Data Cache));
   api((API Server</p>));
   db[(Database)];
   task1[Operating Cycle<br>Task];
   task2[Fault Detection<br>Task];
   task3[Additional<br>Task];
   faultDetectionMetrics1[Metrics]
   faultDetectionMetrics2[Metrics]
   operatingCycleMetrics1[Metrics];
   operatingCycleMetrics2[Metrics];
   additionalMetrics1[Metrics];
   additionalMetrics2[Metrics];
   faultDetectionFunctions1[Functions]
   faultDetectionFunctions2[Functions]
   operatingCycleFunctions1[Functions];
   operatingCycleFunctions2[Functions];
   additionalfunctions1[Functions];
   additionalfunctions2[Functions];

   db <--> api;
   api<--->|json|task1;
   api<-->|json|task2;

   clients <---> |"json{point}"| server;
   server  <--> |point| dataCache;
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
```

#### Configuration fo the tasks, metrics, functions

```yaml
server:
    net: TCP            // TCP/UDP
    protocol:           // CMA-Json / CMA-Byte
    addres: 127.0.0.1   // Self local addres
tasks:
    task OperatingCycle:
        cycle: 500 ms
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
        cycle: 100 ms
        metrics:
            metric MetricName1:
                ...
            metric MetricName2:
                ...
```

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
