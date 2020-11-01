use legion::Resources;
use rustpython_compiler::mode::Mode;
use rustpython_vm::function::FuncArgs;
use rustpython_vm::pyobject::{ItemProtocol, PyRef, PyResult};
use rustpython_vm::{builtins::PyCode, scope::Scope, Interpreter, VirtualMachine};
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

pub fn inject_resource_dependencies(resources: &mut Resources) {
    let runtime = Interpreter::default();
    let scope = runtime.enter(|vm| vm.new_scope_with_builtins());

    resources.insert(runtime);
    resources.insert(scope);
}

pub struct PyScript {
    pub source_path: String,
    pub code_ref: PyRef<PyCode>,
}

impl PyScript {
    pub fn compile(source_path: String, runtime: &mut Interpreter) -> Self {
        let file = File::open(&source_path).unwrap();
        let mut buf_reader = BufReader::new(file);
        let mut contents = String::new();
        buf_reader.read_to_string(&mut contents).unwrap();
        let code_ref = runtime
            .enter(|vm| vm.compile(&contents, Mode::Exec, source_path.clone()))
            .unwrap();
        Self {
            source_path,
            code_ref,
        }
    }

    pub fn run_with_scope(&self, runtime: &mut Interpreter, scope: Scope) -> PyResult {
        let code_ref = self.code_ref.clone();
        runtime.enter(|vm| vm.run_code_obj(code_ref, scope))
    }

    pub fn run(&self, runtime: &mut Interpreter) -> PyResult {
        let code_ref = self.code_ref.clone();
        runtime.enter(|vm| {
            let scope = vm.new_scope_with_builtins();
            vm.run_code_obj(code_ref, scope)
        })
    }
}

type RustPyFunction = fn(FuncArgs, &VirtualMachine) -> PyResult;

pub fn inject_function(
    runtime: &mut Interpreter,
    scope: &mut Scope,
    key: &str,
    function: RustPyFunction,
) {
    runtime.enter(|vm| {
        let function = vm.ctx.new_function(function);
        scope.globals.set_item(key, function, &vm);
    });
}

// impl Python {
//     pub fn init(&mut self) {

//         self.runtime
//             .enter(|vm| {
//                 let scope = vm.new_scope_with_builtins();
//                 vm.run_code_obj(hello, scope)?;
//                 let function = vm.ctx.new_function(test_func);
//                 let args = PyFuncArgs::new([(vm.ctx.new_int(5))].to_vec(), Default::default());
//                 vm.invoke(&function, args)
//             })
//             .unwrap();
//     }
// }

fn test_func(mut py_args: FuncArgs, vm: &VirtualMachine) -> PyResult {
    let five = vm.ctx.new_int(5);
    let arg = py_args.shift();
    let ret = vm._add(&arg, &five);
    ret
}
