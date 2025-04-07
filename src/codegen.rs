use inkwell::{
    builder::Builder,
    context::Context,
    module::Module,
    values::{BasicValueEnum, FunctionValue, PointerValue, IntValue},
    IntPredicate,
};
use std::collections::HashMap;

pub struct CodeGenerator<'ctx> {
    context: &'ctx Context,
    module: Module<'ctx>,
    builder: Builder<'ctx>,
    variables: HashMap<String, PointerValue<'ctx>>,
    current_function: Option<FunctionValue<'ctx>>,
}

impl<'ctx> CodeGenerator<'ctx> {
    pub fn new(context: &'ctx Context) -> Self {
        let module = context.create_module("main");
        let builder = context.create_builder();
        CodeGenerator {
            context,
            module,
            builder,
            variables: HashMap::new(),
            current_function: None,
        }
    }

    pub fn compile(&mut self, program: &crate::ast::Program) -> Result<(), String> {
        
        for include in &program.includes {
            if let crate::ast::Statement::Include(path) = include {
                println!("Included: {}", path);
            }
        }

        for function in &program.functions {
            self.compile_function(function);
        }

        Ok(())
    }

    pub fn compile_function(&self, function: &crate::ast::Function) -> Result<(), String> {
        //create function type
        let i32_type = self.context.i32_type();
        let fn_type = i32_type.fn_type(&[], false);

        //create function
        let fn_value = self.module.add_function(&function.name, fn_type, None);

        //create entry block
        let entry_block = self.context.append_basic_block(fn_value, "entry");

        //process function body
        for stmt in &function.body {
            self.compile_statement(stmt);
        }

        if let Some(bb) = fn_value.get_last_basic_block() {
            if bb.get_terminator().is_none() {
                let zero = i32_type.const_int(0, false);
                self.builder.build_return(Some(&zero));
            }
        }

        Ok(())
    }

    pub fn compile_statement(&self, stmt: &crate::ast::Statement) -> Result<(), String> {
        match stmt {
            crate::ast::Statement::Declare(_, name, expr) => {
                let alloca = self.builder.build_alloca(self.context.i32_type(), name)
                    .map_err(|e| e.to_string())?;
                self.variables.insert(name.clone(), alloca);
                
                let value = self.compile_expr(expr)?;
                self.builder.build_store(alloca, value);
            },

            crate::ast::Statement::Assign(name, expr) => {
                if let Some(alloca) = self.variables.get(name) {
                    let value = self.compile_expr(expr)?;
                    
                }
            },

            crate::ast::Statement::Return(expr) => {},

            crate::ast::Statement::Print(args) => {},

            crate::ast::Statement::Scan(args) => {},

            _ => {}
        }

        Ok(())
    }

    pub fn compile_expr(&self, expr: &crate::ast::Expr) -> Result<IntValue<'ctx>, String> {
        Ok(())
    }
}
