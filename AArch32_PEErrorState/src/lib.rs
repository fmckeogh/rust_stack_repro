#![no_std]
#![allow(non_snake_case)]
#![allow(unused_assignments)]
#![allow(unused_mut)]
#![allow(unused_parens)]
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unreachable_code)]
#![allow(unused_doc_comments)]
#![allow(non_upper_case_globals)]
//! BOREALIS GENERATED FILE
extern crate alloc;
use ErrorIsSynchronized::*;
use ActionRequired::*;
use ReportErrorAsUEU::*;
use StateIsRecoverable::*;
use ReportErrorAsUC::*;
use ReportErrorAsUER::*;
use ErrorIsContained::*;
use common::*;
pub fn AArch32_PEErrorState<T: Tracer>(
    state: &mut State,
    tracer: &T,
    fault: ProductType1d757adad216cdef,
) -> u32 {
    #[derive(Default)]
    struct FunctionState {
        return_value: u32,
        gs_8622: bool,
        gs_8624: bool,
        gs_8623: bool,
        gs_8626: bool,
        gs_8625: bool,
        fault: ProductType1d757adad216cdef,
    }
    let fn_state = FunctionState {
        fault,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u32 {
        // C s_0_0: const #() : ()
        let s_0_0: () = ();
        // S s_0_1: call ErrorIsContained(s_0_0)
        let s_0_1: bool = ErrorIsContained(state, tracer, s_0_0);
        // S s_0_2: not s_0_1
        let s_0_2: bool = !s_0_1;
        // N s_0_3: branch s_0_2 b22 b1
        if s_0_2 {
            return block_22(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u32 {
        // C s_1_0: const #() : ()
        let s_1_0: () = ();
        // S s_1_1: call ErrorIsSynchronized(s_1_0)
        let s_1_1: bool = ErrorIsSynchronized(state, tracer, s_1_0);
        // S s_1_2: not s_1_1
        let s_1_2: bool = !s_1_1;
        // N s_1_3: branch s_1_2 b21 b2
        if s_1_2 {
            return block_21(state, tracer, fn_state);
        } else {
            return block_2(state, tracer, fn_state);
        };
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u32 {
        // C s_2_0: const #0u : u8
        let s_2_0: bool = false;
        // D s_2_1: write-var gs#8622 <= s_2_0
        fn_state.gs_8622 = s_2_0;
        // N s_2_2: jump b3
        return block_3(state, tracer, fn_state);
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u32 {
        // D s_3_0: read-var gs#8622:u8
        let s_3_0: bool = fn_state.gs_8622;
        // D s_3_1: write-var gs#8623 <= s_3_0
        fn_state.gs_8623 = s_3_0;
        // N s_3_2: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u32 {
        // D s_4_0: read-var gs#8623:u8
        let s_4_0: bool = fn_state.gs_8623;
        // N s_4_1: branch s_4_0 b20 b5
        if s_4_0 {
            return block_20(state, tracer, fn_state);
        } else {
            return block_5(state, tracer, fn_state);
        };
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u32 {
        // C s_5_0: const #() : ()
        let s_5_0: () = ();
        // S s_5_1: call ReportErrorAsUC(s_5_0)
        let s_5_1: bool = ReportErrorAsUC(state, tracer, s_5_0);
        // D s_5_2: write-var gs#8624 <= s_5_1
        fn_state.gs_8624 = s_5_1;
        // N s_5_3: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u32 {
        // D s_6_0: read-var gs#8624:u8
        let s_6_0: bool = fn_state.gs_8624;
        // N s_6_1: branch s_6_0 b19 b7
        if s_6_0 {
            return block_19(state, tracer, fn_state);
        } else {
            return block_7(state, tracer, fn_state);
        };
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u32 {
        // C s_7_0: const #() : ()
        let s_7_0: () = ();
        // S s_7_1: call StateIsRecoverable(s_7_0)
        let s_7_1: bool = StateIsRecoverable(state, tracer, s_7_0);
        // S s_7_2: not s_7_1
        let s_7_2: bool = !s_7_1;
        // N s_7_3: branch s_7_2 b18 b8
        if s_7_2 {
            return block_18(state, tracer, fn_state);
        } else {
            return block_8(state, tracer, fn_state);
        };
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u32 {
        // C s_8_0: const #() : ()
        let s_8_0: () = ();
        // S s_8_1: call ReportErrorAsUEU(s_8_0)
        let s_8_1: bool = ReportErrorAsUEU(state, tracer, s_8_0);
        // D s_8_2: write-var gs#8625 <= s_8_1
        fn_state.gs_8625 = s_8_1;
        // N s_8_3: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u32 {
        // D s_9_0: read-var gs#8625:u8
        let s_9_0: bool = fn_state.gs_8625;
        // N s_9_1: branch s_9_0 b17 b10
        if s_9_0 {
            return block_17(state, tracer, fn_state);
        } else {
            return block_10(state, tracer, fn_state);
        };
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u32 {
        // C s_10_0: const #() : ()
        let s_10_0: () = ();
        // S s_10_1: call ActionRequired(s_10_0)
        let s_10_1: bool = ActionRequired(state, tracer, s_10_0);
        // N s_10_2: branch s_10_1 b16 b11
        if s_10_1 {
            return block_16(state, tracer, fn_state);
        } else {
            return block_11(state, tracer, fn_state);
        };
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u32 {
        // C s_11_0: const #() : ()
        let s_11_0: () = ();
        // S s_11_1: call ReportErrorAsUER(s_11_0)
        let s_11_1: bool = ReportErrorAsUER(state, tracer, s_11_0);
        // D s_11_2: write-var gs#8626 <= s_11_1
        fn_state.gs_8626 = s_11_1;
        // N s_11_3: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u32 {
        // D s_12_0: read-var gs#8626:u8
        let s_12_0: bool = fn_state.gs_8626;
        // N s_12_1: branch s_12_0 b15 b13
        if s_12_0 {
            return block_15(state, tracer, fn_state);
        } else {
            return block_13(state, tracer, fn_state);
        };
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u32 {
        // C s_13_0: const #2u : u32
        let s_13_0: u32 = 2;
        // D s_13_1: write-var return_value <= s_13_0
        fn_state.return_value = s_13_0;
        // N s_13_2: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u32 {
        // D s_14_0: read-var return_value:u32
        let s_14_0: u32 = fn_state.return_value;
        // N s_14_1: return s_14_0
        return s_14_0;
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u32 {
        // C s_15_0: const #3u : u32
        let s_15_0: u32 = 3;
        // D s_15_1: write-var return_value <= s_15_0
        fn_state.return_value = s_15_0;
        // N s_15_2: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u32 {
        // C s_16_0: const #1u : u8
        let s_16_0: bool = true;
        // D s_16_1: write-var gs#8626 <= s_16_0
        fn_state.gs_8626 = s_16_0;
        // N s_16_2: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u32 {
        // C s_17_0: const #1u : u32
        let s_17_0: u32 = 1;
        // D s_17_1: write-var return_value <= s_17_0
        fn_state.return_value = s_17_0;
        // N s_17_2: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u32 {
        // C s_18_0: const #1u : u8
        let s_18_0: bool = true;
        // D s_18_1: write-var gs#8625 <= s_18_0
        fn_state.gs_8625 = s_18_0;
        // N s_18_2: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u32 {
        // C s_19_0: const #0u : u32
        let s_19_0: u32 = 0;
        // D s_19_1: write-var return_value <= s_19_0
        fn_state.return_value = s_19_0;
        // N s_19_2: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u32 {
        // C s_20_0: const #1u : u8
        let s_20_0: bool = true;
        // D s_20_1: write-var gs#8624 <= s_20_0
        fn_state.gs_8624 = s_20_0;
        // N s_20_2: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u32 {
        // C s_21_0: const #() : ()
        let s_21_0: () = ();
        // S s_21_1: call StateIsRecoverable(s_21_0)
        let s_21_1: bool = StateIsRecoverable(state, tracer, s_21_0);
        // S s_21_2: not s_21_1
        let s_21_2: bool = !s_21_1;
        // D s_21_3: write-var gs#8622 <= s_21_2
        fn_state.gs_8622 = s_21_2;
        // N s_21_4: jump b3
        return block_3(state, tracer, fn_state);
    }
    fn block_22<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u32 {
        // C s_22_0: const #1u : u8
        let s_22_0: bool = true;
        // D s_22_1: write-var gs#8623 <= s_22_0
        fn_state.gs_8623 = s_22_0;
        // N s_22_2: jump b4
        return block_4(state, tracer, fn_state);
    }
}
