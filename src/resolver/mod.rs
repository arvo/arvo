//! Resolver

use super::air;
use super::ast;
use super::identifier::{Id, Identify, Name, Symbolise};

pub struct Resolver {
    // the source of truth for information about the nodes that have been
    // resolved
    context: air::Context,

    // modules can be declared inside other modules so we declare a stack of
    // them
    module_stack: air::Modules,

    // functions can be declared inside the body of other functions so we
    // declare a stack for them.
    function_stack: air::Functions,

    // blocks can be declared inside other blocks, so we declare a stack for
    // them
    block_stack: air::BlockExprs,
}

impl Resolver {
    pub fn new(context: air::Context) -> Resolver {
        Resolver {
            context: context,
            module_stack: air::Modules::new(),
            function_stack: air::Functions::new(),
            block_stack: air::BlockExprs::new(),
        }
    }

    pub fn context(self) -> air::Context {
        self.context
    }

    pub fn resolve_function(&mut self, function: &ast::Function) -> air::Function {
        // resolving the function profile is idempotent so we do it here just
        // in case it hasn't been done yet
        let mut resolved_function = self.resolve_function_profile(&function.profile);

        // descend into the function
        self.function_stack.push(resolved_function);
        let resolved_body = Some(self.resolve_expr(&function.body));
        let mut resolved_function = self.function_stack.pop().unwrap();
        resolved_function.body = resolved_body;

        // add the function back to the resovler now that it has been updated
        self.add_function(&resolved_function);
        resolved_function
    }

    pub fn resolve_function_profile(&mut self, function_profile: &ast::FunctionProfile) -> air::Function {
        let resolved_function = air::Function::new(
            function_profile.symbolise(),
            self.resolve_variables(&function_profile.formals),
            self.resolve_type(&function_profile.ret),
            None
        );
        self.add_function(&resolved_function);
        resolved_function
    }

    pub fn resolve_module(&mut self, module: &ast::Module) -> air::Module {
        let resolved_module = air::Module::new(module.symbolise(),
                                               air::FunctionTable::new(),
                                               air::ModuleTable::new(),
                                               air::TypeTable::new());
        self.add_module(&resolved_module);

        // descend into the module
        self.module_stack.push(resolved_module);
        self.insert_module_children(module);
        let resolved_module = self.module_stack.pop().unwrap();

        // the module has been changed and so it needs to be added to the
        // resolved
        self.add_module(&resolved_module);
        resolved_module
    }

    fn insert_module_children(&mut self,
                              module: &ast::Module) {
        // resolve all child modules because circular dependencies are not
        // allowed we know that the parent module does not have to be resolved
        // before the child module
        self.insert_module_submodules(module);

        // resolve all types next because types cannot use functions but
        // functions can use types
        self.insert_module_types(module);

        // resolve all function profiles next because functions can call each
        // other so the profiles must be resolved before the functions
        // themselves
        self.insert_module_function_profiles(module);

        // finally resolve the functions
        self.insert_module_functions(module);
    }

    fn insert_module_functions(&mut self, module: &ast::Module) {
        for decl in &module.declarations {
            match *decl {
                ast::Decl::Function(ref function) => {
                    let resolved_function = self.resolve_function(function);
                    self.add_function(&resolved_function);
                }
                _ => (),
            }
        }
    }

    fn insert_module_function_profiles(&mut self, module: &ast::Module) {
        for decl in &module.declarations {
            match *decl {
                ast::Decl::Function(ref function) => {
                    let resolved_function_profile = self.resolve_function_profile(&function.profile);
                    self.add_function(&resolved_function_profile);
                }
                ast::Decl::FunctionProfile(ref function_profile) => {
                    let resolved_function_profile = self.resolve_function_profile(function_profile);
                    self.add_function(&resolved_function_profile);
                }
                _ => (),
            }
        }
    }

    fn insert_module_submodules(&mut self, module: &ast::Module) {
        for decl in &module.declarations {
            match *decl {
                ast::Decl::Module(ref module) => {
                    let resolved_module = self.resolve_module(module);
                    self.add_module(&resolved_module);
                }
                _ => (),
            }
        }
    }

    fn insert_module_types(&mut self, module: &ast::Module) {
        for decl in &module.declarations {
            match *decl {
                ast::Decl::Type(ref ty) => {
                    let resolved_type = self.resolve_type(ty);
                    self.add_type(&resolved_type);
                }
                _ => (),
            }
        }
    }

    pub fn resolve_variable(&mut self, variable: &ast::Variable) -> air::Variable {
        air::Variable::new(variable.symbolise(), self.resolve_type(&variable.ty))
    }

    pub fn resolve_variables(&mut self, variables: &ast::Variables) -> air::Variables {
        variables
            .iter()
            .map(|variable| self.resolve_variable(variable))
            .collect()
    }

    pub fn resolve_expr(&mut self, expr: &ast::Expr) -> air::Expr {
        unimplemented!()
    }

    pub fn resolve_type(&mut self, ty: &ast::Type) -> air::Type {
        unimplemented!()
    }

    fn add_function(&mut self, function: &air::Function) {
        if let Some(parent_module) = self.module_stack.iter_mut().last() {
            parent_module.function_table.insert(function.identify(), function.clone());
        }
        self.context.function_table.insert(function.identify(), function.clone());
    }

    fn add_module(&mut self, module: &air::Module) {
        if let Some(parent_module) = self.module_stack.iter_mut().last() {
            let mut redeclared = false;
            for (identifier, m) in &parent_module.module_table {
                if m.identify().id() != identifier.id() && m.symbolise().name() == module.symbolise().name() {
                    redeclared = true;
                }
            }
            if redeclared {
                unimplemented!()
            } else {
                parent_module.module_table.insert(module.identify(), module.clone());
            }
        }
        self.context.module_table.insert(module.identify(), module.clone());
    }

    fn add_type(&mut self, ty: &air::Type) {
        unimplemented!()
    }
}