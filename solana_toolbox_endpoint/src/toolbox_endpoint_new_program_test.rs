use solana_program_runtime::invoke_context::BuiltinFunctionWithContext;
use solana_program_test::ProgramTest;
use solana_program_test::ProgramTestContext;
use solana_sdk::pubkey::Pubkey;

use crate::toolbox_endpoint::ToolboxEndpoint;
use crate::toolbox_endpoint_inner::ToolboxEndpointInner;

pub struct ToolboxEndpointProgramTestBuiltinProgram {
    pub id: Pubkey,
    pub name: &'static str,
    pub function: Option<BuiltinFunctionWithContext>,
}

#[macro_export]
macro_rules! toolbox_endpoint_program_test_builtin_program {
    ($builtin_id: expr, $builtin_function: expr) => {
        solana_toolbox_endpoint::ToolboxEndpointProgramTestBuiltinProgram {
            id: $builtin_id,
            name: "",
            function: $crate::solana_program_test_processor!($builtin_function),
        }
    };
}

pub struct ToolboxEndpointProgramTestPreloadedProgram {
    pub id: Pubkey,
    pub path: &'static str,
}

impl ToolboxEndpoint {
    pub async fn new_program_test_with_builtin_and_preloaded_programs(
        builtin_programs: &[ToolboxEndpointProgramTestBuiltinProgram],
        preloaded_programs: &[ToolboxEndpointProgramTestPreloadedProgram],
    ) -> ToolboxEndpoint {
        let mut program_test = ProgramTest::default();
        for builtin_program in builtin_programs {
            program_test.add_program(
                builtin_program.name,
                builtin_program.id,
                builtin_program.function,
            );
        }
        program_test.prefer_bpf(true);
        for preloaded_program in preloaded_programs {
            program_test.add_program(
                preloaded_program.path,
                preloaded_program.id,
                None,
            );
        }
        program_test.start_with_context().await.into()
    }

    pub async fn new_program_test_with_preloaded_programs(
        preloaded_programs: &[ToolboxEndpointProgramTestPreloadedProgram]
    ) -> ToolboxEndpoint {
        let mut program_test = ProgramTest::default();
        program_test.prefer_bpf(true);
        for preloaded_program in preloaded_programs {
            program_test.add_program(
                preloaded_program.path,
                preloaded_program.id,
                None,
            );
        }
        program_test.start_with_context().await.into()
    }
}

impl From<ProgramTestContext> for ToolboxEndpoint {
    fn from(program_test_context: ProgramTestContext) -> Self {
        let endpoint_inner: Box<dyn ToolboxEndpointInner> =
            Box::new(program_test_context);
        ToolboxEndpoint::from(endpoint_inner)
    }
}
