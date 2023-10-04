# Fault Recorder Service 

#### Storeing following information into the API Server:
- operating cycle
- operating cycle metrics
- process metrics

#### Function diagram:
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

math task functions configuration:
```yaml
let VarName1:
   input: fn functionName:
      initial: point '/path/Point.Name/'

let VarName2:
   input: fn functionName:
      initial: VarName1
      input: functionName:
         input1: const someValue
         input2: point '/path/Point.Name/'
         input: fn functionName:
            input: point '/path/Point.Name/'
...
```

```yaml
MetricName1
    default: 0      # начальное значение
    input: 
        var VarName1:
            fn count:
                input: 
                    - /line1/ied1/db1/Dev1.State

MetricName2
    default: 0      # начальное значение
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