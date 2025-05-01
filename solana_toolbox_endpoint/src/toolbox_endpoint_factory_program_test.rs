use solana_program_runtime::invoke_context::BuiltinFunctionWithContext;
use solana_program_test::ProgramTest;
use solana_sdk::pubkey::Pubkey;

use crate::toolbox_endpoint::ToolboxEndpoint;
use crate::toolbox_endpoint_proxy::ToolboxEndpointProxy;
use crate::toolbox_endpoint_proxy_program_test_context::ToolboxEndpointProxyProgramTestContext;

pub struct ToolboxEndpointProgramTestBuiltinProgram {
    pub id: Pubkey,
    pub name: &'static str,
    pub processor: Option<BuiltinFunctionWithContext>,
}

#[macro_export]
macro_rules! toolbox_endpoint_program_test_builtin_program {
    ($program_name:expr, $program_id:expr, $program_entry:expr) => {
        $crate::ToolboxEndpointProgramTestBuiltinProgram {
            id: $program_id,
            name: $program_name,
            processor: $crate::solana_program_test_processor!($program_entry),
        }
    };
}

#[macro_export]
macro_rules! toolbox_endpoint_program_test_builtin_program_anchor {
    ($program_name:expr, $program_id:expr, $program_entry:expr) => {
        $crate::ToolboxEndpointProgramTestBuiltinProgram {
            id: $program_id,
            name: $program_name,
            processor: $crate::solana_program_test_processor!(
                |program_id, accounts, data| {
                    let accounts = Box::leak(Box::new(accounts.to_vec()));
                    $program_entry(program_id, accounts, data)
                }
            ),
        }
    };
}

pub struct ToolboxEndpointProgramTestPreloadedProgram {
    pub id: Pubkey,
    pub path: &'static str,
}

impl ToolboxEndpoint {
    pub async fn new_program_test() -> ToolboxEndpoint {
        ToolboxEndpoint::new_program_test_with_builtin_and_preloaded_programs(
            &[],
            &[],
        )
        .await
    }

    pub async fn new_program_test_with_builtin_programs(
        builtin_programs: &[ToolboxEndpointProgramTestBuiltinProgram],
    ) -> ToolboxEndpoint {
        ToolboxEndpoint::new_program_test_with_builtin_and_preloaded_programs(
            builtin_programs,
            &[],
        )
        .await
    }

    pub async fn new_program_test_with_preloaded_programs(
        preloaded_programs: &[ToolboxEndpointProgramTestPreloadedProgram],
    ) -> ToolboxEndpoint {
        ToolboxEndpoint::new_program_test_with_builtin_and_preloaded_programs(
            &[],
            preloaded_programs,
        )
        .await
    }

    pub async fn new_program_test_with_builtin_and_preloaded_programs(
        builtin_programs: &[ToolboxEndpointProgramTestBuiltinProgram],
        preloaded_programs: &[ToolboxEndpointProgramTestPreloadedProgram],
    ) -> ToolboxEndpoint {
        let mut program_test = ProgramTest::default();
        for builtin_program in builtin_programs {
            program_test.add_program(
                builtin_program.name,
                builtin_program.id,
                builtin_program.processor,
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
        let context = program_test.start_with_context().await;
        let proxy: Box<dyn ToolboxEndpointProxy> = Box::new(
            ToolboxEndpointProxyProgramTestContext::new(context).await,
        );
        ToolboxEndpoint::from(proxy)
    }
}
