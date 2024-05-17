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
use IsExternalSyncAbort__1::*;
use ReportErrorAsUncategorized::*;
use ExtAbortToA64::*;
use StateIsRecoverable::*;
use ReportErrorAsUC::*;
use FaultIsCorrected::*;
use ReportErrorAsUER::*;
use ErrorIsContained::*;
use ReportErrorAsIMPDEF::*;
use common::*;
pub fn AArch64_PEErrorState<T: Tracer>(
    state: &mut State,
    tracer: &T,
    fault: ProductType1d757adad216cdef,
) -> u32 {
    #[derive(Default)]
    struct FunctionState {
        gs_9629: bool,
        gs_9628: bool,
        gs_9631: bool,
        return_value: u32,
        gs_9627: bool,
        gs_9626: bool,
        gs_9630: bool,
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
        // D s_0_0: read-var fault:struct
        let s_0_0: ProductType1d757adad216cdef = fn_state.fault;
        // D s_0_1: call IsExternalSyncAbort__1(s_0_0)
        let s_0_1: bool = IsExternalSyncAbort__1(state, tracer, s_0_0);
        // D s_0_2: not s_0_1
        let s_0_2: bool = !s_0_1;
        // N s_0_3: branch s_0_2 b34 b1
        if s_0_2 {
            return block_34(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u32 {
        // C s_1_0: const #0u : u8
        let s_1_0: bool = false;
        // D s_1_1: write-var gs#9626 <= s_1_0
        fn_state.gs_9626 = s_1_0;
        // N s_1_2: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u32 {
        // D s_2_0: read-var gs#9626:u8
        let s_2_0: bool = fn_state.gs_9626;
        // N s_2_1: branch s_2_0 b29 b3
        if s_2_0 {
            return block_29(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u32 {
        // N s_3_0: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u32 {
        // C s_4_0: const #() : ()
        let s_4_0: () = ();
        // S s_4_1: call FaultIsCorrected(s_4_0)
        let s_4_1: bool = FaultIsCorrected(state, tracer, s_4_0);
        // S s_4_2: not s_4_1
        let s_4_2: bool = !s_4_1;
        // N s_4_3: assert s_4_2
        let s_4_3: () = assert!(s_4_2);
        // C s_4_4: const #() : ()
        let s_4_4: () = ();
        // S s_4_5: call ErrorIsContained(s_4_4)
        let s_4_5: bool = ErrorIsContained(state, tracer, s_4_4);
        // S s_4_6: not s_4_5
        let s_4_6: bool = !s_4_5;
        // N s_4_7: branch s_4_6 b28 b5
        if s_4_6 {
            return block_28(state, tracer, fn_state);
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
        // S s_5_1: call ErrorIsSynchronized(s_5_0)
        let s_5_1: bool = ErrorIsSynchronized(state, tracer, s_5_0);
        // S s_5_2: not s_5_1
        let s_5_2: bool = !s_5_1;
        // N s_5_3: branch s_5_2 b27 b6
        if s_5_2 {
            return block_27(state, tracer, fn_state);
        } else {
            return block_6(state, tracer, fn_state);
        };
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u32 {
        // C s_6_0: const #0u : u8
        let s_6_0: bool = false;
        // D s_6_1: write-var gs#9627 <= s_6_0
        fn_state.gs_9627 = s_6_0;
        // N s_6_2: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u32 {
        // D s_7_0: read-var gs#9627:u8
        let s_7_0: bool = fn_state.gs_9627;
        // D s_7_1: write-var gs#9628 <= s_7_0
        fn_state.gs_9628 = s_7_0;
        // N s_7_2: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u32 {
        // D s_8_0: read-var gs#9628:u8
        let s_8_0: bool = fn_state.gs_9628;
        // N s_8_1: branch s_8_0 b26 b9
        if s_8_0 {
            return block_26(state, tracer, fn_state);
        } else {
            return block_9(state, tracer, fn_state);
        };
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u32 {
        // C s_9_0: const #() : ()
        let s_9_0: () = ();
        // S s_9_1: call ReportErrorAsUC(s_9_0)
        let s_9_1: bool = ReportErrorAsUC(state, tracer, s_9_0);
        // D s_9_2: write-var gs#9629 <= s_9_1
        fn_state.gs_9629 = s_9_1;
        // N s_9_3: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u32 {
        // D s_10_0: read-var gs#9629:u8
        let s_10_0: bool = fn_state.gs_9629;
        // N s_10_1: branch s_10_0 b25 b11
        if s_10_0 {
            return block_25(state, tracer, fn_state);
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
        // S s_11_1: call StateIsRecoverable(s_11_0)
        let s_11_1: bool = StateIsRecoverable(state, tracer, s_11_0);
        // S s_11_2: not s_11_1
        let s_11_2: bool = !s_11_1;
        // N s_11_3: branch s_11_2 b24 b12
        if s_11_2 {
            return block_24(state, tracer, fn_state);
        } else {
            return block_12(state, tracer, fn_state);
        };
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u32 {
        // C s_12_0: const #() : ()
        let s_12_0: () = ();
        // S s_12_1: call ReportErrorAsUEU(s_12_0)
        let s_12_1: bool = ReportErrorAsUEU(state, tracer, s_12_0);
        // D s_12_2: write-var gs#9630 <= s_12_1
        fn_state.gs_9630 = s_12_1;
        // N s_12_3: jump b13
        return block_13(state, tracer, fn_state);
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u32 {
        // D s_13_0: read-var gs#9630:u8
        let s_13_0: bool = fn_state.gs_9630;
        // N s_13_1: branch s_13_0 b21 b14
        if s_13_0 {
            return block_21(state, tracer, fn_state);
        } else {
            return block_14(state, tracer, fn_state);
        };
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u32 {
        // C s_14_0: const #() : ()
        let s_14_0: () = ();
        // S s_14_1: call ActionRequired(s_14_0)
        let s_14_1: bool = ActionRequired(state, tracer, s_14_0);
        // N s_14_2: branch s_14_1 b20 b15
        if s_14_1 {
            return block_20(state, tracer, fn_state);
        } else {
            return block_15(state, tracer, fn_state);
        };
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u32 {
        // C s_15_0: const #() : ()
        let s_15_0: () = ();
        // S s_15_1: call ReportErrorAsUER(s_15_0)
        let s_15_1: bool = ReportErrorAsUER(state, tracer, s_15_0);
        // D s_15_2: write-var gs#9631 <= s_15_1
        fn_state.gs_9631 = s_15_1;
        // N s_15_3: jump b16
        return block_16(state, tracer, fn_state);
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u32 {
        // D s_16_0: read-var gs#9631:u8
        let s_16_0: bool = fn_state.gs_9631;
        // N s_16_1: branch s_16_0 b19 b17
        if s_16_0 {
            return block_19(state, tracer, fn_state);
        } else {
            return block_17(state, tracer, fn_state);
        };
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u32 {
        // C s_17_0: const #2u : u32
        let s_17_0: u32 = 2;
        // D s_17_1: write-var return_value <= s_17_0
        fn_state.return_value = s_17_0;
        // N s_17_2: jump b18
        return block_18(state, tracer, fn_state);
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u32 {
        // D s_18_0: read-var return_value:u32
        let s_18_0: u32 = fn_state.return_value;
        // N s_18_1: return s_18_0
        return s_18_0;
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u32 {
        // C s_19_0: const #3u : u32
        let s_19_0: u32 = 3;
        // D s_19_1: write-var return_value <= s_19_0
        fn_state.return_value = s_19_0;
        // N s_19_2: jump b18
        return block_18(state, tracer, fn_state);
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u32 {
        // C s_20_0: const #1u : u8
        let s_20_0: bool = true;
        // D s_20_1: write-var gs#9631 <= s_20_0
        fn_state.gs_9631 = s_20_0;
        // N s_20_2: jump b16
        return block_16(state, tracer, fn_state);
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u32 {
        // D s_21_0: read-var fault:struct
        let s_21_0: ProductType1d757adad216cdef = fn_state.fault;
        // D s_21_1: call IsExternalSyncAbort__1(s_21_0)
        let s_21_1: bool = IsExternalSyncAbort__1(state, tracer, s_21_0);
        // N s_21_2: branch s_21_1 b23 b22
        if s_21_1 {
            return block_23(state, tracer, fn_state);
        } else {
            return block_22(state, tracer, fn_state);
        };
    }
    fn block_22<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u32 {
        // C s_22_0: const #1u : u32
        let s_22_0: u32 = 1;
        // D s_22_1: write-var return_value <= s_22_0
        fn_state.return_value = s_22_0;
        // N s_22_2: jump b18
        return block_18(state, tracer, fn_state);
    }
    fn block_23<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u32 {
        // C s_23_0: const #0u : u32
        let s_23_0: u32 = 0;
        // D s_23_1: write-var return_value <= s_23_0
        fn_state.return_value = s_23_0;
        // N s_23_2: jump b18
        return block_18(state, tracer, fn_state);
    }
    fn block_24<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u32 {
        // C s_24_0: const #1u : u8
        let s_24_0: bool = true;
        // D s_24_1: write-var gs#9630 <= s_24_0
        fn_state.gs_9630 = s_24_0;
        // N s_24_2: jump b13
        return block_13(state, tracer, fn_state);
    }
    fn block_25<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u32 {
        // C s_25_0: const #0u : u32
        let s_25_0: u32 = 0;
        // D s_25_1: write-var return_value <= s_25_0
        fn_state.return_value = s_25_0;
        // N s_25_2: jump b18
        return block_18(state, tracer, fn_state);
    }
    fn block_26<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u32 {
        // C s_26_0: const #1u : u8
        let s_26_0: bool = true;
        // D s_26_1: write-var gs#9629 <= s_26_0
        fn_state.gs_9629 = s_26_0;
        // N s_26_2: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_27<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u32 {
        // C s_27_0: const #() : ()
        let s_27_0: () = ();
        // S s_27_1: call StateIsRecoverable(s_27_0)
        let s_27_1: bool = StateIsRecoverable(state, tracer, s_27_0);
        // S s_27_2: not s_27_1
        let s_27_2: bool = !s_27_1;
        // D s_27_3: write-var gs#9627 <= s_27_2
        fn_state.gs_9627 = s_27_2;
        // N s_27_4: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_28<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u32 {
        // C s_28_0: const #1u : u8
        let s_28_0: bool = true;
        // D s_28_1: write-var gs#9628 <= s_28_0
        fn_state.gs_9628 = s_28_0;
        // N s_28_2: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_29<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u32 {
        // C s_29_0: const #() : ()
        let s_29_0: () = ();
        // S s_29_1: call ReportErrorAsUncategorized(s_29_0)
        let s_29_1: bool = ReportErrorAsUncategorized(state, tracer, s_29_0);
        // N s_29_2: branch s_29_1 b33 b30
        if s_29_1 {
            return block_33(state, tracer, fn_state);
        } else {
            return block_30(state, tracer, fn_state);
        };
    }
    fn block_30<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u32 {
        // C s_30_0: const #() : ()
        let s_30_0: () = ();
        // S s_30_1: call ReportErrorAsIMPDEF(s_30_0)
        let s_30_1: bool = ReportErrorAsIMPDEF(state, tracer, s_30_0);
        // N s_30_2: branch s_30_1 b32 b31
        if s_30_1 {
            return block_32(state, tracer, fn_state);
        } else {
            return block_31(state, tracer, fn_state);
        };
    }
    fn block_31<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u32 {
        // N s_31_0: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_32<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u32 {
        // C s_32_0: const #6u : u32
        let s_32_0: u32 = 6;
        // D s_32_1: write-var return_value <= s_32_0
        fn_state.return_value = s_32_0;
        // N s_32_2: jump b18
        return block_18(state, tracer, fn_state);
    }
    fn block_33<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u32 {
        // C s_33_0: const #5u : u32
        let s_33_0: u32 = 5;
        // D s_33_1: write-var return_value <= s_33_0
        fn_state.return_value = s_33_0;
        // N s_33_2: jump b18
        return block_18(state, tracer, fn_state);
    }
    fn block_34<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u32 {
        // D s_34_0: read-var fault:struct
        let s_34_0: ProductType1d757adad216cdef = fn_state.fault;
        // D s_34_1: call ExtAbortToA64(s_34_0)
        let s_34_1: bool = ExtAbortToA64(state, tracer, s_34_0);
        // D s_34_2: write-var gs#9626 <= s_34_1
        fn_state.gs_9626 = s_34_1;
        // N s_34_3: jump b2
        return block_2(state, tracer, fn_state);
    }
}
