///
/// Function | Returns ...
#[derive(Debug)]
pub struct FnTemplate {
    id: String,
    kind: FnKind,
    input: FnInOutRef,
}
///
/// 
impl FnTemplate {
    ///
    /// Creates new instance of the FnTemplate
    // #[allow(dead_code)]
    pub fn new(parent: impl Into<String>, input: FnInOutRef) -> Self {
        Self { 
            id: format!("{}/FnTemplate{}", parent.into(), COUNT.fetch_add(1, Ordering::Relaxed)),
            kind: FnKind::Fn,
            input,
        }
    }    
}
///
/// 
impl FnIn for FnTemplate {}
///
/// 
impl FnOut for FnTemplate { 
    //
    fn id(&self) -> String {
        self.id.clone()
    }
    //
    fn kind(&self) -> &FnKind {
        &self.kind
    }
    //
    fn inputs(&self) -> Vec<String> {
        self.input.borrow().inputs()
    }
    //
    //
    fn out(&mut self) -> FnResult {
        let input = self.input.borrow_mut().out();
        trace!("{}.out | input: {:?}", self.id, input);
        match input {
            FnResult::Ok(point) => {
            }
            FnResult::Err(err) => FnResult::Err(err),
            FnResult::None => FnResult::None,
        }
    }
    //
    //
    fn reset(&mut self) {
        self.input.borrow_mut().reset();
    }
}
///
/// 
impl FnInOut for FnTemplate {}
///
/// Global static counter of FnOut instances
static COUNT: AtomicUsize = AtomicUsize::new(1);
