# Fault Recorder Service 

#### Storeing following information into the API Server:
- operating cycle
- operating cycle metrics
- process metrics

#### Function diagram:
```mermaid
flowchart TD;
   client([ApiRequest]);
   api((API Server));
   field_datas[FieldDatas];
   db[(Database)];
   general_info[GeneralInfoPage];
   work_cycle_points[WorkCyclePoints];
   work_cycles[WorkCyclesPage];
   failures[FailuresPage];
   tensosensor_calibration[TensosensorCalibrationPage];
   api <--> db;
   client<-->|json|api;
   field_datas  <--> client;
   general_info <--> |"List[FieldData]"| field_datas;
   client --> work_cycle_points;
   work_cycle_points --> |"List[WorkCyclePoint]"| work_cycles;
   client --> failures;
   client --> tensosensor_calibration;
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