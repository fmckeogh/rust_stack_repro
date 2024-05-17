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
use HasArchVersion::*;
use HaveStatisticalProfilingv1p2::*;
use HighestEL::*;
use common::*;
pub fn ImpDefBool<T: Tracer>(state: &mut State, tracer: &T, x: &'static str) -> bool {
    #[derive(Default)]
    struct FunctionState {
        gs_331368: bool,
        gs_331366: bool,
        return_value: bool,
        gs_331370: bool,
        gs_331367: bool,
        gs_331369: bool,
        x: &'static str,
    }
    let fn_state = FunctionState {
        x,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_0_0: read-var x:str
        let s_0_0: &'static str = fn_state.x;
        // C s_0_1: const #"the PE resets into EL2 or EL3" : str
        let s_0_1: &'static str = "the PE resets into EL2 or EL3";
        // D s_0_2: cmp-eq s_0_0 s_0_1
        let s_0_2: bool = ((s_0_0) == (s_0_1));
        // N s_0_3: branch s_0_2 b661 b1
        if s_0_2 {
            return block_661(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_1_0: read-var x:str
        let s_1_0: &'static str = fn_state.x;
        // C s_1_1: const #"the PE resets into EL1" : str
        let s_1_1: &'static str = "the PE resets into EL1";
        // D s_1_2: cmp-eq s_1_0 s_1_1
        let s_1_2: bool = ((s_1_0) == (s_1_1));
        // N s_1_3: branch s_1_2 b660 b2
        if s_1_2 {
            return block_660(state, tracer, fn_state);
        } else {
            return block_2(state, tracer, fn_state);
        };
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_2_0: read-var x:str
        let s_2_0: &'static str = fn_state.x;
        // C s_2_1: const #"the PE resets into EL2" : str
        let s_2_1: &'static str = "the PE resets into EL2";
        // D s_2_2: cmp-eq s_2_0 s_2_1
        let s_2_2: bool = ((s_2_0) == (s_2_1));
        // N s_2_3: branch s_2_2 b659 b3
        if s_2_2 {
            return block_659(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_3_0: read-var x:str
        let s_3_0: &'static str = fn_state.x;
        // C s_3_1: const #"the PE resets into EL3" : str
        let s_3_1: &'static str = "the PE resets into EL3";
        // D s_3_2: cmp-eq s_3_0 s_3_1
        let s_3_2: bool = ((s_3_0) == (s_3_1));
        // N s_3_3: branch s_3_2 b658 b4
        if s_3_2 {
            return block_658(state, tracer, fn_state);
        } else {
            return block_4(state, tracer, fn_state);
        };
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_4_0: read-var x:str
        let s_4_0: &'static str = fn_state.x;
        // C s_4_1: const #"IMPLEMENTED_ITD" : str
        let s_4_1: &'static str = "IMPLEMENTED_ITD";
        // D s_4_2: cmp-eq s_4_0 s_4_1
        let s_4_2: bool = ((s_4_0) == (s_4_1));
        // N s_4_3: branch s_4_2 b657 b5
        if s_4_2 {
            return block_657(state, tracer, fn_state);
        } else {
            return block_5(state, tracer, fn_state);
        };
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_5_0: read-var x:str
        let s_5_0: &'static str = fn_state.x;
        // C s_5_1: const #"IMPLEMENTED_CP15BEN" : str
        let s_5_1: &'static str = "IMPLEMENTED_CP15BEN";
        // D s_5_2: cmp-eq s_5_0 s_5_1
        let s_5_2: bool = ((s_5_0) == (s_5_1));
        // N s_5_3: branch s_5_2 b656 b6
        if s_5_2 {
            return block_656(state, tracer, fn_state);
        } else {
            return block_6(state, tracer, fn_state);
        };
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_6_0: read-var x:str
        let s_6_0: &'static str = fn_state.x;
        // C s_6_1: const #"in a system that supports only a single Security state" : str
        let s_6_1: &'static str = "in a system that supports only a single Security state";
        // D s_6_2: cmp-eq s_6_0 s_6_1
        let s_6_2: bool = ((s_6_0) == (s_6_1));
        // N s_6_3: branch s_6_2 b655 b7
        if s_6_2 {
            return block_655(state, tracer, fn_state);
        } else {
            return block_7(state, tracer, fn_state);
        };
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_7_0: read-var x:str
        let s_7_0: &'static str = fn_state.x;
        // C s_7_1: const #"access is Secure, in a system that supports two Security states" : str
        let s_7_1: &'static str = "access is Secure, in a system that supports two Security states";
        // D s_7_2: cmp-eq s_7_0 s_7_1
        let s_7_2: bool = ((s_7_0) == (s_7_1));
        // N s_7_3: branch s_7_2 b654 b8
        if s_7_2 {
            return block_654(state, tracer, fn_state);
        } else {
            return block_8(state, tracer, fn_state);
        };
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_8_0: read-var x:str
        let s_8_0: &'static str = fn_state.x;
        // C s_8_1: const #"GICD_CTLR.DS==0, Secure access" : str
        let s_8_1: &'static str = "GICD_CTLR.DS==0, Secure access";
        // D s_8_2: cmp-eq s_8_0 s_8_1
        let s_8_2: bool = ((s_8_0) == (s_8_1));
        // N s_8_3: branch s_8_2 b653 b9
        if s_8_2 {
            return block_653(state, tracer, fn_state);
        } else {
            return block_9(state, tracer, fn_state);
        };
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_9_0: read-var x:str
        let s_9_0: &'static str = fn_state.x;
        // C s_9_1: const #"Has FP16 extension" : str
        let s_9_1: &'static str = "Has FP16 extension";
        // D s_9_2: cmp-eq s_9_0 s_9_1
        let s_9_2: bool = ((s_9_0) == (s_9_1));
        // N s_9_3: branch s_9_2 b652 b10
        if s_9_2 {
            return block_652(state, tracer, fn_state);
        } else {
            return block_10(state, tracer, fn_state);
        };
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_10_0: read-var x:str
        let s_10_0: &'static str = fn_state.x;
        // C s_10_1: const #"Edge-triggered SError" : str
        let s_10_1: &'static str = "Edge-triggered SError";
        // D s_10_2: cmp-eq s_10_0 s_10_1
        let s_10_2: bool = ((s_10_0) == (s_10_1));
        // N s_10_3: branch s_10_2 b651 b11
        if s_10_2 {
            return block_651(state, tracer, fn_state);
        } else {
            return block_11(state, tracer, fn_state);
        };
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_11_0: read-var x:str
        let s_11_0: &'static str = fn_state.x;
        // C s_11_1: const #"AMEVCNTR1[m] is fixed" : str
        let s_11_1: &'static str = "AMEVCNTR1[m] is fixed";
        // D s_11_2: cmp-eq s_11_0 s_11_1
        let s_11_2: bool = ((s_11_0) == (s_11_1));
        // N s_11_3: branch s_11_2 b650 b12
        if s_11_2 {
            return block_650(state, tracer, fn_state);
        } else {
            return block_12(state, tracer, fn_state);
        };
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_12_0: read-var x:str
        let s_12_0: &'static str = fn_state.x;
        // C s_12_1: const #"AMEVCNTR1_EL0[m] is fixed" : str
        let s_12_1: &'static str = "AMEVCNTR1_EL0[m] is fixed";
        // D s_12_2: cmp-eq s_12_0 s_12_1
        let s_12_2: bool = ((s_12_0) == (s_12_1));
        // N s_12_3: branch s_12_2 b649 b13
        if s_12_2 {
            return block_649(state, tracer, fn_state);
        } else {
            return block_13(state, tracer, fn_state);
        };
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_13_0: read-var x:str
        let s_13_0: &'static str = fn_state.x;
        // C s_13_1: const #"Has LRCPC3 support" : str
        let s_13_1: &'static str = "Has LRCPC3 support";
        // D s_13_2: cmp-eq s_13_0 s_13_1
        let s_13_2: bool = ((s_13_0) == (s_13_1));
        // N s_13_3: branch s_13_2 b648 b14
        if s_13_2 {
            return block_648(state, tracer, fn_state);
        } else {
            return block_14(state, tracer, fn_state);
        };
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_14_0: read-var x:str
        let s_14_0: &'static str = fn_state.x;
        // C s_14_1: const #"accessed from EL0" : str
        let s_14_1: &'static str = "accessed from EL0";
        // D s_14_2: cmp-eq s_14_0 s_14_1
        let s_14_2: bool = ((s_14_0) == (s_14_1));
        // N s_14_3: branch s_14_2 b647 b15
        if s_14_2 {
            return block_647(state, tracer, fn_state);
        } else {
            return block_15(state, tracer, fn_state);
        };
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_15_0: read-var x:str
        let s_15_0: &'static str = fn_state.x;
        // C s_15_1: const #"Highest EL using AArch64" : str
        let s_15_1: &'static str = "Highest EL using AArch64";
        // D s_15_2: cmp-eq s_15_0 s_15_1
        let s_15_2: bool = ((s_15_0) == (s_15_1));
        // N s_15_3: branch s_15_2 b646 b16
        if s_15_2 {
            return block_646(state, tracer, fn_state);
        } else {
            return block_16(state, tracer, fn_state);
        };
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_16_0: read-var x:str
        let s_16_0: &'static str = fn_state.x;
        // C s_16_1: const #"PE can reset into AArch64 state" : str
        let s_16_1: &'static str = "PE can reset into AArch64 state";
        // D s_16_2: cmp-eq s_16_0 s_16_1
        let s_16_2: bool = ((s_16_0) == (s_16_1));
        // N s_16_3: branch s_16_2 b645 b17
        if s_16_2 {
            return block_645(state, tracer, fn_state);
        } else {
            return block_17(state, tracer, fn_state);
        };
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_17_0: read-var x:str
        let s_17_0: &'static str = fn_state.x;
        // C s_17_1: const #"PE can reset into AArch32 state" : str
        let s_17_1: &'static str = "PE can reset into AArch32 state";
        // D s_17_2: cmp-eq s_17_0 s_17_1
        let s_17_2: bool = ((s_17_0) == (s_17_1));
        // N s_17_3: branch s_17_2 b644 b18
        if s_17_2 {
            return block_644(state, tracer, fn_state);
        } else {
            return block_18(state, tracer, fn_state);
        };
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_18_0: read-var x:str
        let s_18_0: &'static str = fn_state.x;
        // C s_18_1: const #"IMPLEMENTED_FPSCR.LEN,STRIDE as RAZ" : str
        let s_18_1: &'static str = "IMPLEMENTED_FPSCR.LEN,STRIDE as RAZ";
        // D s_18_2: cmp-eq s_18_0 s_18_1
        let s_18_2: bool = ((s_18_0) == (s_18_1));
        // N s_18_3: branch s_18_2 b643 b19
        if s_18_2 {
            return block_643(state, tracer, fn_state);
        } else {
            return block_19(state, tracer, fn_state);
        };
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_19_0: read-var x:str
        let s_19_0: &'static str = fn_state.x;
        // C s_19_1: const #"System register access to the PE Trace Unit registers is implemented" : str
        let s_19_1: &'static str = "System register access to the PE Trace Unit registers is implemented";
        // D s_19_2: cmp-eq s_19_0 s_19_1
        let s_19_2: bool = ((s_19_0) == (s_19_1));
        // N s_19_3: branch s_19_2 b642 b20
        if s_19_2 {
            return block_642(state, tracer, fn_state);
        } else {
            return block_20(state, tracer, fn_state);
        };
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_20_0: read-var x:str
        let s_20_0: &'static str = fn_state.x;
        // C s_20_1: const #"AArch32 floating-point is implemented" : str
        let s_20_1: &'static str = "AArch32 floating-point is implemented";
        // D s_20_2: cmp-eq s_20_0 s_20_1
        let s_20_2: bool = ((s_20_0) == (s_20_1));
        // N s_20_3: branch s_20_2 b641 b21
        if s_20_2 {
            return block_641(state, tracer, fn_state);
        } else {
            return block_21(state, tracer, fn_state);
        };
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_21_0: read-var x:str
        let s_21_0: &'static str = fn_state.x;
        // C s_21_1: const #"the implementation includes a PMU event export bus" : str
        let s_21_1: &'static str = "the implementation includes a PMU event export bus";
        // D s_21_2: cmp-eq s_21_0 s_21_1
        let s_21_2: bool = ((s_21_0) == (s_21_1));
        // N s_21_3: branch s_21_2 b640 b22
        if s_21_2 {
            return block_640(state, tracer, fn_state);
        } else {
            return block_22(state, tracer, fn_state);
        };
    }
    fn block_22<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_22_0: read-var x:str
        let s_22_0: &'static str = fn_state.x;
        // C s_22_1: const #"Edge-triggered SError" : str
        let s_22_1: &'static str = "Edge-triggered SError";
        // D s_22_2: cmp-eq s_22_0 s_22_1
        let s_22_2: bool = ((s_22_0) == (s_22_1));
        // N s_22_3: branch s_22_2 b639 b23
        if s_22_2 {
            return block_639(state, tracer, fn_state);
        } else {
            return block_23(state, tracer, fn_state);
        };
    }
    fn block_23<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_23_0: read-var x:str
        let s_23_0: &'static str = fn_state.x;
        // C s_23_1: const #"Support trapping of floating-point exceptions" : str
        let s_23_1: &'static str = "Support trapping of floating-point exceptions";
        // D s_23_2: cmp-eq s_23_0 s_23_1
        let s_23_2: bool = ((s_23_0) == (s_23_1));
        // N s_23_3: branch s_23_2 b638 b24
        if s_23_2 {
            return block_638(state, tracer, fn_state);
        } else {
            return block_24(state, tracer, fn_state);
        };
    }
    fn block_24<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_24_0: read-var x:str
        let s_24_0: &'static str = fn_state.x;
        // C s_24_1: const #"IMPLEMENTED_trapping of floating-point exceptions" : str
        let s_24_1: &'static str = "IMPLEMENTED_trapping of floating-point exceptions";
        // D s_24_2: cmp-eq s_24_0 s_24_1
        let s_24_2: bool = ((s_24_0) == (s_24_1));
        // N s_24_3: branch s_24_2 b637 b25
        if s_24_2 {
            return block_637(state, tracer, fn_state);
        } else {
            return block_25(state, tracer, fn_state);
        };
    }
    fn block_25<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_25_0: read-var x:str
        let s_25_0: &'static str = fn_state.x;
        // C s_25_1: const #"Has 16-bit VMID" : str
        let s_25_1: &'static str = "Has 16-bit VMID";
        // D s_25_2: cmp-eq s_25_0 s_25_1
        let s_25_2: bool = ((s_25_0) == (s_25_1));
        // N s_25_3: branch s_25_2 b636 b26
        if s_25_2 {
            return block_636(state, tracer, fn_state);
        } else {
            return block_26(state, tracer, fn_state);
        };
    }
    fn block_26<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_26_0: read-var x:str
        let s_26_0: &'static str = fn_state.x;
        // C s_26_1: const #"DC_ZVA tag fault reported with lowest faulting address" : str
        let s_26_1: &'static str = "DC_ZVA tag fault reported with lowest faulting address";
        // D s_26_2: cmp-eq s_26_0 s_26_1
        let s_26_2: bool = ((s_26_0) == (s_26_1));
        // N s_26_3: branch s_26_2 b635 b27
        if s_26_2 {
            return block_635(state, tracer, fn_state);
        } else {
            return block_27(state, tracer, fn_state);
        };
    }
    fn block_27<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_27_0: read-var x:str
        let s_27_0: &'static str = fn_state.x;
        // C s_27_1: const #"When PoC is before any level of cache, data cache maintenance operations are NOP" : str
        let s_27_1: &'static str = "When PoC is before any level of cache, data cache maintenance operations are NOP";
        // D s_27_2: cmp-eq s_27_0 s_27_1
        let s_27_2: bool = ((s_27_0) == (s_27_1));
        // N s_27_3: branch s_27_2 b634 b28
        if s_27_2 {
            return block_634(state, tracer, fn_state);
        } else {
            return block_28(state, tracer, fn_state);
        };
    }
    fn block_28<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_28_0: read-var x:str
        let s_28_0: &'static str = fn_state.x;
        // C s_28_1: const #"DCIMVAC generates watchpoint" : str
        let s_28_1: &'static str = "DCIMVAC generates watchpoint";
        // D s_28_2: cmp-eq s_28_0 s_28_1
        let s_28_2: bool = ((s_28_0) == (s_28_1));
        // N s_28_3: branch s_28_2 b633 b29
        if s_28_2 {
            return block_633(state, tracer, fn_state);
        } else {
            return block_29(state, tracer, fn_state);
        };
    }
    fn block_29<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_29_0: read-var x:str
        let s_29_0: &'static str = fn_state.x;
        // C s_29_1: const #"Trapped by MDCR_EL2.TDOSA" : str
        let s_29_1: &'static str = "Trapped by MDCR_EL2.TDOSA";
        // D s_29_2: cmp-eq s_29_0 s_29_1
        let s_29_2: bool = ((s_29_0) == (s_29_1));
        // N s_29_3: branch s_29_2 b632 b30
        if s_29_2 {
            return block_632(state, tracer, fn_state);
        } else {
            return block_30(state, tracer, fn_state);
        };
    }
    fn block_30<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_30_0: read-var x:str
        let s_30_0: &'static str = fn_state.x;
        // C s_30_1: const #"Trapped by MDCR_EL3.TDOSA" : str
        let s_30_1: &'static str = "Trapped by MDCR_EL3.TDOSA";
        // D s_30_2: cmp-eq s_30_0 s_30_1
        let s_30_2: bool = ((s_30_0) == (s_30_1));
        // N s_30_3: branch s_30_2 b631 b31
        if s_30_2 {
            return block_631(state, tracer, fn_state);
        } else {
            return block_31(state, tracer, fn_state);
        };
    }
    fn block_31<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_31_0: read-var x:str
        let s_31_0: &'static str = fn_state.x;
        // C s_31_1: const #"Trapped by HDCR.TDOSA" : str
        let s_31_1: &'static str = "Trapped by HDCR.TDOSA";
        // D s_31_2: cmp-eq s_31_0 s_31_1
        let s_31_2: bool = ((s_31_0) == (s_31_1));
        // N s_31_3: branch s_31_2 b630 b32
        if s_31_2 {
            return block_630(state, tracer, fn_state);
        } else {
            return block_32(state, tracer, fn_state);
        };
    }
    fn block_32<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_32_0: read-var x:str
        let s_32_0: &'static str = fn_state.x;
        // C s_32_1: const #"EL3 trap priority when SDD == '1'" : str
        let s_32_1: &'static str = "EL3 trap priority when SDD == '1'";
        // D s_32_2: cmp-eq s_32_0 s_32_1
        let s_32_2: bool = ((s_32_0) == (s_32_1));
        // N s_32_3: branch s_32_2 b629 b33
        if s_32_2 {
            return block_629(state, tracer, fn_state);
        } else {
            return block_33(state, tracer, fn_state);
        };
    }
    fn block_33<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_33_0: read-var x:str
        let s_33_0: &'static str = fn_state.x;
        // C s_33_1: const #"Address Size Fault on LPA descriptor bits [15:12]" : str
        let s_33_1: &'static str = "Address Size Fault on LPA descriptor bits [15:12]";
        // D s_33_2: cmp-eq s_33_0 s_33_1
        let s_33_2: bool = ((s_33_0) == (s_33_1));
        // N s_33_3: branch s_33_2 b628 b34
        if s_33_2 {
            return block_628(state, tracer, fn_state);
        } else {
            return block_34(state, tracer, fn_state);
        };
    }
    fn block_34<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_34_0: read-var x:str
        let s_34_0: &'static str = fn_state.x;
        // C s_34_1: const #"ID_MMFR5_EL1 trapped by HCR_EL2.TID3" : str
        let s_34_1: &'static str = "ID_MMFR5_EL1 trapped by HCR_EL2.TID3";
        // D s_34_2: cmp-eq s_34_0 s_34_1
        let s_34_2: bool = ((s_34_0) == (s_34_1));
        // N s_34_3: branch s_34_2 b627 b35
        if s_34_2 {
            return block_627(state, tracer, fn_state);
        } else {
            return block_35(state, tracer, fn_state);
        };
    }
    fn block_35<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_35_0: read-var x:str
        let s_35_0: &'static str = fn_state.x;
        // C s_35_1: const #"ID_MMFR5 trapped by HCR.TID3" : str
        let s_35_1: &'static str = "ID_MMFR5 trapped by HCR.TID3";
        // D s_35_2: cmp-eq s_35_0 s_35_1
        let s_35_2: bool = ((s_35_0) == (s_35_1));
        // N s_35_3: branch s_35_2 b626 b36
        if s_35_2 {
            return block_626(state, tracer, fn_state);
        } else {
            return block_36(state, tracer, fn_state);
        };
    }
    fn block_36<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_36_0: read-var x:str
        let s_36_0: &'static str = fn_state.x;
        // C s_36_1: const #"ID_MMFR5 trapped by HCR_EL2.TID3" : str
        let s_36_1: &'static str = "ID_MMFR5 trapped by HCR_EL2.TID3";
        // D s_36_2: cmp-eq s_36_0 s_36_1
        let s_36_2: bool = ((s_36_0) == (s_36_1));
        // N s_36_3: branch s_36_2 b625 b37
        if s_36_2 {
            return block_625(state, tracer, fn_state);
        } else {
            return block_37(state, tracer, fn_state);
        };
    }
    fn block_37<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_37_0: read-var x:str
        let s_37_0: &'static str = fn_state.x;
        // C s_37_1: const #"Has Trace Architecture functionality" : str
        let s_37_1: &'static str = "Has Trace Architecture functionality";
        // D s_37_2: cmp-eq s_37_0 s_37_1
        let s_37_2: bool = ((s_37_0) == (s_37_1));
        // N s_37_3: branch s_37_2 b624 b38
        if s_37_2 {
            return block_624(state, tracer, fn_state);
        } else {
            return block_38(state, tracer, fn_state);
        };
    }
    fn block_38<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_38_0: read-var x:str
        let s_38_0: &'static str = fn_state.x;
        // C s_38_1: const #"Reserved Control Space Supported" : str
        let s_38_1: &'static str = "Reserved Control Space Supported";
        // D s_38_2: cmp-eq s_38_0 s_38_1
        let s_38_2: bool = ((s_38_0) == (s_38_1));
        // N s_38_3: branch s_38_2 b623 b39
        if s_38_2 {
            return block_623(state, tracer, fn_state);
        } else {
            return block_39(state, tracer, fn_state);
        };
    }
    fn block_39<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_39_0: read-var x:str
        let s_39_0: &'static str = fn_state.x;
        // C s_39_1: const #"Reserved Control Space Traps Supported" : str
        let s_39_1: &'static str = "Reserved Control Space Traps Supported";
        // D s_39_2: cmp-eq s_39_0 s_39_1
        let s_39_2: bool = ((s_39_0) == (s_39_1));
        // N s_39_3: branch s_39_2 b622 b40
        if s_39_2 {
            return block_622(state, tracer, fn_state);
        } else {
            return block_40(state, tracer, fn_state);
        };
    }
    fn block_40<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_40_0: read-var x:str
        let s_40_0: &'static str = fn_state.x;
        // C s_40_1: const #"Reserved Control Space EL0 Trapped" : str
        let s_40_1: &'static str = "Reserved Control Space EL0 Trapped";
        // D s_40_2: cmp-eq s_40_0 s_40_1
        let s_40_2: bool = ((s_40_0) == (s_40_1));
        // N s_40_3: branch s_40_2 b621 b41
        if s_40_2 {
            return block_621(state, tracer, fn_state);
        } else {
            return block_41(state, tracer, fn_state);
        };
    }
    fn block_41<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_41_0: read-var x:str
        let s_41_0: &'static str = fn_state.x;
        // C s_41_1: const #"Illegal Execution State on return to AArch32" : str
        let s_41_1: &'static str = "Illegal Execution State on return to AArch32";
        // D s_41_2: cmp-eq s_41_0 s_41_1
        let s_41_2: bool = ((s_41_0) == (s_41_1));
        // N s_41_3: branch s_41_2 b620 b42
        if s_41_2 {
            return block_620(state, tracer, fn_state);
        } else {
            return block_42(state, tracer, fn_state);
        };
    }
    fn block_42<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_42_0: read-var x:str
        let s_42_0: &'static str = fn_state.x;
        // C s_42_1: const #"Floating-Point Traps Information" : str
        let s_42_1: &'static str = "Floating-Point Traps Information";
        // D s_42_2: cmp-eq s_42_0 s_42_1
        let s_42_2: bool = ((s_42_0) == (s_42_1));
        // N s_42_3: branch s_42_2 b619 b43
        if s_42_2 {
            return block_619(state, tracer, fn_state);
        } else {
            return block_43(state, tracer, fn_state);
        };
    }
    fn block_43<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_43_0: read-var x:str
        let s_43_0: &'static str = fn_state.x;
        // C s_43_1: const #"Condition valid for trapped T32" : str
        let s_43_1: &'static str = "Condition valid for trapped T32";
        // D s_43_2: cmp-eq s_43_0 s_43_1
        let s_43_2: bool = ((s_43_0) == (s_43_1));
        // N s_43_3: branch s_43_2 b618 b44
        if s_43_2 {
            return block_618(state, tracer, fn_state);
        } else {
            return block_44(state, tracer, fn_state);
        };
    }
    fn block_44<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_44_0: read-var x:str
        let s_44_0: &'static str = fn_state.x;
        // C s_44_1: const #"Translation fault on misprogrammed contiguous bit" : str
        let s_44_1: &'static str = "Translation fault on misprogrammed contiguous bit";
        // D s_44_2: cmp-eq s_44_0 s_44_1
        let s_44_2: bool = ((s_44_0) == (s_44_1));
        // N s_44_3: branch s_44_2 b617 b45
        if s_44_2 {
            return block_617(state, tracer, fn_state);
        } else {
            return block_45(state, tracer, fn_state);
        };
    }
    fn block_45<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_45_0: read-var x:str
        let s_45_0: &'static str = fn_state.x;
        // C s_45_1: const #"Virtual SError syndrome valid" : str
        let s_45_1: &'static str = "Virtual SError syndrome valid";
        // D s_45_2: cmp-eq s_45_0 s_45_1
        let s_45_2: bool = ((s_45_0) == (s_45_1));
        // N s_45_3: branch s_45_2 b616 b46
        if s_45_2 {
            return block_616(state, tracer, fn_state);
        } else {
            return block_46(state, tracer, fn_state);
        };
    }
    fn block_46<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_46_0: read-var x:str
        let s_46_0: &'static str = fn_state.x;
        // C s_46_1: const #"Have CRC extension" : str
        let s_46_1: &'static str = "Have CRC extension";
        // D s_46_2: cmp-eq s_46_0 s_46_1
        let s_46_2: bool = ((s_46_0) == (s_46_1));
        // N s_46_3: branch s_46_2 b615 b47
        if s_46_2 {
            return block_615(state, tracer, fn_state);
        } else {
            return block_47(state, tracer, fn_state);
        };
    }
    fn block_47<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_47_0: read-var x:str
        let s_47_0: &'static str = fn_state.x;
        // C s_47_1: const #"Report I-cache maintenance fault in IFSR" : str
        let s_47_1: &'static str = "Report I-cache maintenance fault in IFSR";
        // D s_47_2: cmp-eq s_47_0 s_47_1
        let s_47_2: bool = ((s_47_0) == (s_47_1));
        // N s_47_3: branch s_47_2 b614 b48
        if s_47_2 {
            return block_614(state, tracer, fn_state);
        } else {
            return block_48(state, tracer, fn_state);
        };
    }
    fn block_48<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_48_0: read-var x:str
        let s_48_0: &'static str = fn_state.x;
        // C s_48_1: const #"Permission fault on EL0 IC_IVAU execution" : str
        let s_48_1: &'static str = "Permission fault on EL0 IC_IVAU execution";
        // D s_48_2: cmp-eq s_48_0 s_48_1
        let s_48_2: bool = ((s_48_0) == (s_48_1));
        // N s_48_3: branch s_48_2 b613 b49
        if s_48_2 {
            return block_613(state, tracer, fn_state);
        } else {
            return block_49(state, tracer, fn_state);
        };
    }
    fn block_49<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_49_0: read-var x:str
        let s_49_0: &'static str = fn_state.x;
        // C s_49_1: const #"UNDEF unallocated CP15 access at EL0" : str
        let s_49_1: &'static str = "UNDEF unallocated CP15 access at EL0";
        // D s_49_2: cmp-eq s_49_0 s_49_1
        let s_49_2: bool = ((s_49_0) == (s_49_1));
        // N s_49_3: branch s_49_2 b612 b50
        if s_49_2 {
            return block_612(state, tracer, fn_state);
        } else {
            return block_50(state, tracer, fn_state);
        };
    }
    fn block_50<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_50_0: read-var x:str
        let s_50_0: &'static str = fn_state.x;
        // C s_50_1: const #"Align PC on illegal exception return" : str
        let s_50_1: &'static str = "Align PC on illegal exception return";
        // D s_50_2: cmp-eq s_50_0 s_50_1
        let s_50_2: bool = ((s_50_0) == (s_50_1));
        // N s_50_3: branch s_50_2 b611 b51
        if s_50_2 {
            return block_611(state, tracer, fn_state);
        } else {
            return block_51(state, tracer, fn_state);
        };
    }
    fn block_51<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_51_0: read-var x:str
        let s_51_0: &'static str = fn_state.x;
        // C s_51_1: const #"EL from SPSR on illegal exception return" : str
        let s_51_1: &'static str = "EL from SPSR on illegal exception return";
        // D s_51_2: cmp-eq s_51_0 s_51_1
        let s_51_2: bool = ((s_51_0) == (s_51_1));
        // N s_51_3: branch s_51_2 b610 b52
        if s_51_2 {
            return block_610(state, tracer, fn_state);
        } else {
            return block_52(state, tracer, fn_state);
        };
    }
    fn block_52<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_52_0: read-var x:str
        let s_52_0: &'static str = fn_state.x;
        // C s_52_1: const #"Has SHA1 Crypto instructions" : str
        let s_52_1: &'static str = "Has SHA1 Crypto instructions";
        // D s_52_2: cmp-eq s_52_0 s_52_1
        let s_52_2: bool = ((s_52_0) == (s_52_1));
        // N s_52_3: branch s_52_2 b609 b53
        if s_52_2 {
            return block_609(state, tracer, fn_state);
        } else {
            return block_53(state, tracer, fn_state);
        };
    }
    fn block_53<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_53_0: read-var x:str
        let s_53_0: &'static str = fn_state.x;
        // C s_53_1: const #"Has SHA256 Crypto instructions" : str
        let s_53_1: &'static str = "Has SHA256 Crypto instructions";
        // D s_53_2: cmp-eq s_53_0 s_53_1
        let s_53_2: bool = ((s_53_0) == (s_53_1));
        // N s_53_3: branch s_53_2 b608 b54
        if s_53_2 {
            return block_608(state, tracer, fn_state);
        } else {
            return block_54(state, tracer, fn_state);
        };
    }
    fn block_54<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_54_0: read-var x:str
        let s_54_0: &'static str = fn_state.x;
        // C s_54_1: const #"Has 128-bit form of PMULL instructions" : str
        let s_54_1: &'static str = "Has 128-bit form of PMULL instructions";
        // D s_54_2: cmp-eq s_54_0 s_54_1
        let s_54_2: bool = ((s_54_0) == (s_54_1));
        // N s_54_3: branch s_54_2 b604 b55
        if s_54_2 {
            return block_604(state, tracer, fn_state);
        } else {
            return block_55(state, tracer, fn_state);
        };
    }
    fn block_55<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_55_0: read-var x:str
        let s_55_0: &'static str = fn_state.x;
        // C s_55_1: const #"vector instructions set TFV to 1" : str
        let s_55_1: &'static str = "vector instructions set TFV to 1";
        // D s_55_2: cmp-eq s_55_0 s_55_1
        let s_55_2: bool = ((s_55_0) == (s_55_1));
        // N s_55_3: branch s_55_2 b603 b56
        if s_55_2 {
            return block_603(state, tracer, fn_state);
        } else {
            return block_56(state, tracer, fn_state);
        };
    }
    fn block_56<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_56_0: read-var x:str
        let s_56_0: &'static str = fn_state.x;
        // C s_56_1: const #"JOSCR UNDEFINED at EL0" : str
        let s_56_1: &'static str = "JOSCR UNDEFINED at EL0";
        // D s_56_2: cmp-eq s_56_0 s_56_1
        let s_56_2: bool = ((s_56_0) == (s_56_1));
        // N s_56_3: branch s_56_2 b602 b57
        if s_56_2 {
            return block_602(state, tracer, fn_state);
        } else {
            return block_57(state, tracer, fn_state);
        };
    }
    fn block_57<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_57_0: read-var x:str
        let s_57_0: &'static str = fn_state.x;
        // C s_57_1: const #"JMCR UNDEFINED at EL0" : str
        let s_57_1: &'static str = "JMCR UNDEFINED at EL0";
        // D s_57_2: cmp-eq s_57_0 s_57_1
        let s_57_2: bool = ((s_57_0) == (s_57_1));
        // N s_57_3: branch s_57_2 b601 b58
        if s_57_2 {
            return block_601(state, tracer, fn_state);
        } else {
            return block_58(state, tracer, fn_state);
        };
    }
    fn block_58<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_58_0: read-var x:str
        let s_58_0: &'static str = fn_state.x;
        // C s_58_1: const #"JIDR UNDEFINED at EL0" : str
        let s_58_1: &'static str = "JIDR UNDEFINED at EL0";
        // D s_58_2: cmp-eq s_58_0 s_58_1
        let s_58_2: bool = ((s_58_0) == (s_58_1));
        // N s_58_3: branch s_58_2 b600 b59
        if s_58_2 {
            return block_600(state, tracer, fn_state);
        } else {
            return block_59(state, tracer, fn_state);
        };
    }
    fn block_59<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_59_0: read-var x:str
        let s_59_0: &'static str = fn_state.x;
        // C s_59_1: const #"Has accumulate FP16 product into FP32 extension" : str
        let s_59_1: &'static str = "Has accumulate FP16 product into FP32 extension";
        // D s_59_2: cmp-eq s_59_0 s_59_1
        let s_59_2: bool = ((s_59_0) == (s_59_1));
        // N s_59_3: branch s_59_2 b599 b60
        if s_59_2 {
            return block_599(state, tracer, fn_state);
        } else {
            return block_60(state, tracer, fn_state);
        };
    }
    fn block_60<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_60_0: read-var x:str
        let s_60_0: &'static str = fn_state.x;
        // C s_60_1: const #"Has Dot Product extension" : str
        let s_60_1: &'static str = "Has Dot Product extension";
        // D s_60_2: cmp-eq s_60_0 s_60_1
        let s_60_2: bool = ((s_60_0) == (s_60_1));
        // N s_60_3: branch s_60_2 b598 b61
        if s_60_2 {
            return block_598(state, tracer, fn_state);
        } else {
            return block_61(state, tracer, fn_state);
        };
    }
    fn block_61<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_61_0: read-var x:str
        let s_61_0: &'static str = fn_state.x;
        // C s_61_1: const #"Has SHA512 Crypto instructions" : str
        let s_61_1: &'static str = "Has SHA512 Crypto instructions";
        // D s_61_2: cmp-eq s_61_0 s_61_1
        let s_61_2: bool = ((s_61_0) == (s_61_1));
        // N s_61_3: branch s_61_2 b597 b62
        if s_61_2 {
            return block_597(state, tracer, fn_state);
        } else {
            return block_62(state, tracer, fn_state);
        };
    }
    fn block_62<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_62_0: read-var x:str
        let s_62_0: &'static str = fn_state.x;
        // C s_62_1: const #"Has SHA3 Crypto instructions" : str
        let s_62_1: &'static str = "Has SHA3 Crypto instructions";
        // D s_62_2: cmp-eq s_62_0 s_62_1
        let s_62_2: bool = ((s_62_0) == (s_62_1));
        // N s_62_3: branch s_62_2 b596 b63
        if s_62_2 {
            return block_596(state, tracer, fn_state);
        } else {
            return block_63(state, tracer, fn_state);
        };
    }
    fn block_63<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_63_0: read-var x:str
        let s_63_0: &'static str = fn_state.x;
        // C s_63_1: const #"Has SM3 Crypto instructions" : str
        let s_63_1: &'static str = "Has SM3 Crypto instructions";
        // D s_63_2: cmp-eq s_63_0 s_63_1
        let s_63_2: bool = ((s_63_0) == (s_63_1));
        // N s_63_3: branch s_63_2 b595 b64
        if s_63_2 {
            return block_595(state, tracer, fn_state);
        } else {
            return block_64(state, tracer, fn_state);
        };
    }
    fn block_64<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_64_0: read-var x:str
        let s_64_0: &'static str = fn_state.x;
        // C s_64_1: const #"Has SM4 Crypto instructions" : str
        let s_64_1: &'static str = "Has SM4 Crypto instructions";
        // D s_64_2: cmp-eq s_64_0 s_64_1
        let s_64_2: bool = ((s_64_0) == (s_64_1));
        // N s_64_3: branch s_64_2 b594 b65
        if s_64_2 {
            return block_594(state, tracer, fn_state);
        } else {
            return block_65(state, tracer, fn_state);
        };
    }
    fn block_65<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_65_0: read-var x:str
        let s_65_0: &'static str = fn_state.x;
        // C s_65_1: const #"CPY* instructions use Option A" : str
        let s_65_1: &'static str = "CPY* instructions use Option A";
        // D s_65_2: cmp-eq s_65_0 s_65_1
        let s_65_2: bool = ((s_65_0) == (s_65_1));
        // N s_65_3: branch s_65_2 b593 b66
        if s_65_2 {
            return block_593(state, tracer, fn_state);
        } else {
            return block_66(state, tracer, fn_state);
        };
    }
    fn block_66<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_66_0: read-var x:str
        let s_66_0: &'static str = fn_state.x;
        // C s_66_1: const #"CPYF* instructions use Option A" : str
        let s_66_1: &'static str = "CPYF* instructions use Option A";
        // D s_66_2: cmp-eq s_66_0 s_66_1
        let s_66_2: bool = ((s_66_0) == (s_66_1));
        // N s_66_3: branch s_66_2 b592 b67
        if s_66_2 {
            return block_592(state, tracer, fn_state);
        } else {
            return block_67(state, tracer, fn_state);
        };
    }
    fn block_67<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_67_0: read-var x:str
        let s_67_0: &'static str = fn_state.x;
        // C s_67_1: const #"SET* instructions use Option A" : str
        let s_67_1: &'static str = "SET* instructions use Option A";
        // D s_67_2: cmp-eq s_67_0 s_67_1
        let s_67_2: bool = ((s_67_0) == (s_67_1));
        // N s_67_3: branch s_67_2 b591 b68
        if s_67_2 {
            return block_591(state, tracer, fn_state);
        } else {
            return block_68(state, tracer, fn_state);
        };
    }
    fn block_68<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_68_0: read-var x:str
        let s_68_0: &'static str = fn_state.x;
        // C s_68_1: const #"SETG* instructions use Option A" : str
        let s_68_1: &'static str = "SETG* instructions use Option A";
        // D s_68_2: cmp-eq s_68_0 s_68_1
        let s_68_2: bool = ((s_68_0) == (s_68_1));
        // N s_68_3: branch s_68_2 b590 b69
        if s_68_2 {
            return block_590(state, tracer, fn_state);
        } else {
            return block_69(state, tracer, fn_state);
        };
    }
    fn block_69<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_69_0: read-var x:str
        let s_69_0: &'static str = fn_state.x;
        // C s_69_1: const #"Has PMUv3 threshold extension" : str
        let s_69_1: &'static str = "Has PMUv3 threshold extension";
        // D s_69_2: cmp-eq s_69_0 s_69_1
        let s_69_2: bool = ((s_69_0) == (s_69_1));
        // N s_69_3: branch s_69_2 b589 b70
        if s_69_2 {
            return block_589(state, tracer, fn_state);
        } else {
            return block_70(state, tracer, fn_state);
        };
    }
    fn block_70<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_70_0: read-var x:str
        let s_70_0: &'static str = fn_state.x;
        // C s_70_1: const #"Has PMUv3 fixed-function instruction counter" : str
        let s_70_1: &'static str = "Has PMUv3 fixed-function instruction counter";
        // D s_70_2: cmp-eq s_70_0 s_70_1
        let s_70_2: bool = ((s_70_0) == (s_70_1));
        // N s_70_3: branch s_70_2 b588 b71
        if s_70_2 {
            return block_588(state, tracer, fn_state);
        } else {
            return block_71(state, tracer, fn_state);
        };
    }
    fn block_71<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_71_0: read-var x:str
        let s_71_0: &'static str = fn_state.x;
        // C s_71_1: const #"Has Flag manipulate extension" : str
        let s_71_1: &'static str = "Has Flag manipulate extension";
        // D s_71_2: cmp-eq s_71_0 s_71_1
        let s_71_2: bool = ((s_71_0) == (s_71_1));
        // N s_71_3: branch s_71_2 b587 b72
        if s_71_2 {
            return block_587(state, tracer, fn_state);
        } else {
            return block_72(state, tracer, fn_state);
        };
    }
    fn block_72<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_72_0: read-var x:str
        let s_72_0: &'static str = fn_state.x;
        // C s_72_1: const #"Has Nested Virtualization" : str
        let s_72_1: &'static str = "Has Nested Virtualization";
        // D s_72_2: cmp-eq s_72_0 s_72_1
        let s_72_2: bool = ((s_72_0) == (s_72_1));
        // N s_72_3: branch s_72_2 b586 b73
        if s_72_2 {
            return block_586(state, tracer, fn_state);
        } else {
            return block_73(state, tracer, fn_state);
        };
    }
    fn block_73<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_73_0: read-var x:str
        let s_73_0: &'static str = fn_state.x;
        // C s_73_1: const #"Has support for Enhanced Nested Virtualization" : str
        let s_73_1: &'static str = "Has support for Enhanced Nested Virtualization";
        // D s_73_2: cmp-eq s_73_0 s_73_1
        let s_73_2: bool = ((s_73_0) == (s_73_1));
        // N s_73_3: branch s_73_2 b585 b74
        if s_73_2 {
            return block_585(state, tracer, fn_state);
        } else {
            return block_74(state, tracer, fn_state);
        };
    }
    fn block_74<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_74_0: read-var x:str
        let s_74_0: &'static str = fn_state.x;
        // C s_74_1: const #"BBM level 1 or 2 support nT bit causes Translation Fault" : str
        let s_74_1: &'static str = "BBM level 1 or 2 support nT bit causes Translation Fault";
        // D s_74_2: cmp-eq s_74_0 s_74_1
        let s_74_2: bool = ((s_74_0) == (s_74_1));
        // N s_74_3: branch s_74_2 b584 b75
        if s_74_2 {
            return block_584(state, tracer, fn_state);
        } else {
            return block_75(state, tracer, fn_state);
        };
    }
    fn block_75<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_75_0: read-var x:str
        let s_75_0: &'static str = fn_state.x;
        // C s_75_1: const #"Apply effective shareability at stage 1" : str
        let s_75_1: &'static str = "Apply effective shareability at stage 1";
        // D s_75_2: cmp-eq s_75_0 s_75_1
        let s_75_2: bool = ((s_75_0) == (s_75_1));
        // N s_75_3: branch s_75_2 b583 b76
        if s_75_2 {
            return block_583(state, tracer, fn_state);
        } else {
            return block_76(state, tracer, fn_state);
        };
    }
    fn block_76<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_76_0: read-var x:str
        let s_76_0: &'static str = fn_state.x;
        // C s_76_1: const #"Has MPAM extension" : str
        let s_76_1: &'static str = "Has MPAM extension";
        // D s_76_2: cmp-eq s_76_0 s_76_1
        let s_76_2: bool = ((s_76_0) == (s_76_1));
        // N s_76_3: branch s_76_2 b582 b77
        if s_76_2 {
            return block_582(state, tracer, fn_state);
        } else {
            return block_77(state, tracer, fn_state);
        };
    }
    fn block_77<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_77_0: read-var x:str
        let s_77_0: &'static str = fn_state.x;
        // C s_77_1: const #"Has enhanced MPAMv0p1 extension" : str
        let s_77_1: &'static str = "Has enhanced MPAMv0p1 extension";
        // D s_77_2: cmp-eq s_77_0 s_77_1
        let s_77_2: bool = ((s_77_0) == (s_77_1));
        // N s_77_3: branch s_77_2 b578 b78
        if s_77_2 {
            return block_578(state, tracer, fn_state);
        } else {
            return block_78(state, tracer, fn_state);
        };
    }
    fn block_78<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_78_0: read-var x:str
        let s_78_0: &'static str = fn_state.x;
        // C s_78_1: const #"Has enhanced MPAMv1p1 extension" : str
        let s_78_1: &'static str = "Has enhanced MPAMv1p1 extension";
        // D s_78_2: cmp-eq s_78_0 s_78_1
        let s_78_2: bool = ((s_78_0) == (s_78_1));
        // N s_78_3: branch s_78_2 b574 b79
        if s_78_2 {
            return block_574(state, tracer, fn_state);
        } else {
            return block_79(state, tracer, fn_state);
        };
    }
    fn block_79<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_79_0: read-var x:str
        let s_79_0: &'static str = fn_state.x;
        // C s_79_1: const #"Has EMPAM SDEFLT" : str
        let s_79_1: &'static str = "Has EMPAM SDEFLT";
        // D s_79_2: cmp-eq s_79_0 s_79_1
        let s_79_2: bool = ((s_79_0) == (s_79_1));
        // N s_79_3: branch s_79_2 b573 b80
        if s_79_2 {
            return block_573(state, tracer, fn_state);
        } else {
            return block_80(state, tracer, fn_state);
        };
    }
    fn block_80<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_80_0: read-var x:str
        let s_80_0: &'static str = fn_state.x;
        // C s_80_1: const #"Has EMPAM FORCE_NS" : str
        let s_80_1: &'static str = "Has EMPAM FORCE_NS";
        // D s_80_2: cmp-eq s_80_0 s_80_1
        let s_80_2: bool = ((s_80_0) == (s_80_1));
        // N s_80_3: branch s_80_2 b572 b81
        if s_80_2 {
            return block_572(state, tracer, fn_state);
        } else {
            return block_81(state, tracer, fn_state);
        };
    }
    fn block_81<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_81_0: read-var x:str
        let s_81_0: &'static str = fn_state.x;
        // C s_81_1: const #"EMPAM FORCE_NS is RAO" : str
        let s_81_1: &'static str = "EMPAM FORCE_NS is RAO";
        // D s_81_2: cmp-eq s_81_0 s_81_1
        let s_81_2: bool = ((s_81_0) == (s_81_1));
        // N s_81_3: branch s_81_2 b571 b82
        if s_81_2 {
            return block_571(state, tracer, fn_state);
        } else {
            return block_82(state, tracer, fn_state);
        };
    }
    fn block_82<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_82_0: read-var x:str
        let s_82_0: &'static str = fn_state.x;
        // C s_82_1: const #"Has EMPAM TIDR" : str
        let s_82_1: &'static str = "Has EMPAM TIDR";
        // D s_82_2: cmp-eq s_82_0 s_82_1
        let s_82_2: bool = ((s_82_0) == (s_82_1));
        // N s_82_3: branch s_82_2 b570 b83
        if s_82_2 {
            return block_570(state, tracer, fn_state);
        } else {
            return block_83(state, tracer, fn_state);
        };
    }
    fn block_83<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_83_0: read-var x:str
        let s_83_0: &'static str = fn_state.x;
        // C s_83_1: const #"Has AArch32 hierarchical permission disables" : str
        let s_83_1: &'static str = "Has AArch32 hierarchical permission disables";
        // D s_83_2: cmp-eq s_83_0 s_83_1
        let s_83_2: bool = ((s_83_0) == (s_83_1));
        // N s_83_3: branch s_83_2 b569 b84
        if s_83_2 {
            return block_569(state, tracer, fn_state);
        } else {
            return block_84(state, tracer, fn_state);
        };
    }
    fn block_84<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_84_0: read-var x:str
        let s_84_0: &'static str = fn_state.x;
        // C s_84_1: const #"Has enhanced PAC functionality" : str
        let s_84_1: &'static str = "Has enhanced PAC functionality";
        // D s_84_2: cmp-eq s_84_0 s_84_1
        let s_84_2: bool = ((s_84_0) == (s_84_1));
        // N s_84_3: branch s_84_2 b568 b85
        if s_84_2 {
            return block_568(state, tracer, fn_state);
        } else {
            return block_85(state, tracer, fn_state);
        };
    }
    fn block_85<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_85_0: read-var x:str
        let s_85_0: &'static str = fn_state.x;
        // C s_85_1: const #"Has enhanced PAC 2 functionality" : str
        let s_85_1: &'static str = "Has enhanced PAC 2 functionality";
        // D s_85_2: cmp-eq s_85_0 s_85_1
        let s_85_2: bool = ((s_85_0) == (s_85_1));
        // N s_85_3: branch s_85_2 b567 b86
        if s_85_2 {
            return block_567(state, tracer, fn_state);
        } else {
            return block_86(state, tracer, fn_state);
        };
    }
    fn block_86<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_86_0: read-var x:str
        let s_86_0: &'static str = fn_state.x;
        // C s_86_1: const #"Has FPAC functionality" : str
        let s_86_1: &'static str = "Has FPAC functionality";
        // D s_86_2: cmp-eq s_86_0 s_86_1
        let s_86_2: bool = ((s_86_0) == (s_86_1));
        // N s_86_3: branch s_86_2 b566 b87
        if s_86_2 {
            return block_566(state, tracer, fn_state);
        } else {
            return block_87(state, tracer, fn_state);
        };
    }
    fn block_87<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_87_0: read-var x:str
        let s_87_0: &'static str = fn_state.x;
        // C s_87_1: const #"Has FPAC Combined functionality" : str
        let s_87_1: &'static str = "Has FPAC Combined functionality";
        // D s_87_2: cmp-eq s_87_0 s_87_1
        let s_87_2: bool = ((s_87_0) == (s_87_1));
        // N s_87_3: branch s_87_2 b565 b88
        if s_87_2 {
            return block_565(state, tracer, fn_state);
        } else {
            return block_88(state, tracer, fn_state);
        };
    }
    fn block_88<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_88_0: read-var x:str
        let s_88_0: &'static str = fn_state.x;
        // C s_88_1: const #"Has MTE extension" : str
        let s_88_1: &'static str = "Has MTE extension";
        // D s_88_2: cmp-eq s_88_0 s_88_1
        let s_88_2: bool = ((s_88_0) == (s_88_1));
        // N s_88_3: branch s_88_2 b564 b89
        if s_88_2 {
            return block_564(state, tracer, fn_state);
        } else {
            return block_89(state, tracer, fn_state);
        };
    }
    fn block_89<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_89_0: read-var x:str
        let s_89_0: &'static str = fn_state.x;
        // C s_89_1: const #"Has MTE2 extension" : str
        let s_89_1: &'static str = "Has MTE2 extension";
        // D s_89_2: cmp-eq s_89_0 s_89_1
        let s_89_2: bool = ((s_89_0) == (s_89_1));
        // N s_89_3: branch s_89_2 b563 b90
        if s_89_2 {
            return block_563(state, tracer, fn_state);
        } else {
            return block_90(state, tracer, fn_state);
        };
    }
    fn block_90<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_90_0: read-var x:str
        let s_90_0: &'static str = fn_state.x;
        // C s_90_1: const #"Has MTE Asynchronous Faulting extension" : str
        let s_90_1: &'static str = "Has MTE Asynchronous Faulting extension";
        // D s_90_2: cmp-eq s_90_0 s_90_1
        let s_90_2: bool = ((s_90_0) == (s_90_1));
        // N s_90_3: branch s_90_2 b562 b91
        if s_90_2 {
            return block_562(state, tracer, fn_state);
        } else {
            return block_91(state, tracer, fn_state);
        };
    }
    fn block_91<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_91_0: read-var x:str
        let s_91_0: &'static str = fn_state.x;
        // C s_91_1: const #"Has Store-only Tag Checking support" : str
        let s_91_1: &'static str = "Has Store-only Tag Checking support";
        // D s_91_2: cmp-eq s_91_0 s_91_1
        let s_91_2: bool = ((s_91_0) == (s_91_1));
        // N s_91_3: branch s_91_2 b561 b92
        if s_91_2 {
            return block_561(state, tracer, fn_state);
        } else {
            return block_92(state, tracer, fn_state);
        };
    }
    fn block_92<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_92_0: read-var x:str
        let s_92_0: &'static str = fn_state.x;
        // C s_92_1: const #"Has CLRBHB instruction" : str
        let s_92_1: &'static str = "Has CLRBHB instruction";
        // D s_92_2: cmp-eq s_92_0 s_92_1
        let s_92_2: bool = ((s_92_0) == (s_92_1));
        // N s_92_3: branch s_92_2 b560 b93
        if s_92_2 {
            return block_560(state, tracer, fn_state);
        } else {
            return block_93(state, tracer, fn_state);
        };
    }
    fn block_93<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_93_0: read-var x:str
        let s_93_0: &'static str = fn_state.x;
        // C s_93_1: const #"SPE PAT for tag unchecked access zero" : str
        let s_93_1: &'static str = "SPE PAT for tag unchecked access zero";
        // D s_93_2: cmp-eq s_93_0 s_93_1
        let s_93_2: bool = ((s_93_0) == (s_93_1));
        // N s_93_3: branch s_93_2 b559 b94
        if s_93_2 {
            return block_559(state, tracer, fn_state);
        } else {
            return block_94(state, tracer, fn_state);
        };
    }
    fn block_94<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_94_0: read-var x:str
        let s_94_0: &'static str = fn_state.x;
        // C s_94_1: const #"SPE non-tbi tag is zero" : str
        let s_94_1: &'static str = "SPE non-tbi tag is zero";
        // D s_94_2: cmp-eq s_94_0 s_94_1
        let s_94_2: bool = ((s_94_0) == (s_94_1));
        // N s_94_3: branch s_94_2 b558 b95
        if s_94_2 {
            return block_558(state, tracer, fn_state);
        } else {
            return block_95(state, tracer, fn_state);
        };
    }
    fn block_95<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_95_0: read-var x:str
        let s_95_0: &'static str = fn_state.x;
        // C s_95_1: const #"Has AArch64 DGH extension" : str
        let s_95_1: &'static str = "Has AArch64 DGH extension";
        // D s_95_2: cmp-eq s_95_0 s_95_1
        let s_95_2: bool = ((s_95_0) == (s_95_1));
        // N s_95_3: branch s_95_2 b557 b96
        if s_95_2 {
            return block_557(state, tracer, fn_state);
        } else {
            return block_96(state, tracer, fn_state);
        };
    }
    fn block_96<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_96_0: read-var x:str
        let s_96_0: &'static str = fn_state.x;
        // C s_96_1: const #"Has Performance Monitors extension" : str
        let s_96_1: &'static str = "Has Performance Monitors extension";
        // D s_96_2: cmp-eq s_96_0 s_96_1
        let s_96_2: bool = ((s_96_0) == (s_96_1));
        // N s_96_3: branch s_96_2 b556 b97
        if s_96_2 {
            return block_556(state, tracer, fn_state);
        } else {
            return block_97(state, tracer, fn_state);
        };
    }
    fn block_97<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_97_0: read-var x:str
        let s_97_0: &'static str = fn_state.x;
        // C s_97_1: const #"Has RNG extension" : str
        let s_97_1: &'static str = "Has RNG extension";
        // D s_97_2: cmp-eq s_97_0 s_97_1
        let s_97_2: bool = ((s_97_0) == (s_97_1));
        // N s_97_3: branch s_97_2 b555 b98
        if s_97_2 {
            return block_555(state, tracer, fn_state);
        } else {
            return block_98(state, tracer, fn_state);
        };
    }
    fn block_98<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_98_0: read-var x:str
        let s_98_0: &'static str = fn_state.x;
        // C s_98_1: const #"Has Small Translation Table extension" : str
        let s_98_1: &'static str = "Has Small Translation Table extension";
        // D s_98_2: cmp-eq s_98_0 s_98_1
        let s_98_2: bool = ((s_98_0) == (s_98_1));
        // N s_98_3: branch s_98_2 b554 b99
        if s_98_2 {
            return block_554(state, tracer, fn_state);
        } else {
            return block_99(state, tracer, fn_state);
        };
    }
    fn block_99<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_99_0: read-var x:str
        let s_99_0: &'static str = fn_state.x;
        // C s_99_1: const #"Secure-only implementation" : str
        let s_99_1: &'static str = "Secure-only implementation";
        // D s_99_2: cmp-eq s_99_0 s_99_1
        let s_99_2: bool = ((s_99_0) == (s_99_1));
        // N s_99_3: branch s_99_2 b553 b100
        if s_99_2 {
            return block_553(state, tracer, fn_state);
        } else {
            return block_100(state, tracer, fn_state);
        };
    }
    fn block_100<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_100_0: read-var x:str
        let s_100_0: &'static str = fn_state.x;
        // C s_100_1: const #"Non-secure only implementation" : str
        let s_100_1: &'static str = "Non-secure only implementation";
        // D s_100_2: cmp-eq s_100_0 s_100_1
        let s_100_2: bool = ((s_100_0) == (s_100_1));
        // N s_100_3: branch s_100_2 b552 b101
        if s_100_2 {
            return block_552(state, tracer, fn_state);
        } else {
            return block_101(state, tracer, fn_state);
        };
    }
    fn block_101<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_101_0: read-var x:str
        let s_101_0: &'static str = fn_state.x;
        // C s_101_1: const #"OS Double Lock is implemented" : str
        let s_101_1: &'static str = "OS Double Lock is implemented";
        // D s_101_2: cmp-eq s_101_0 s_101_1
        let s_101_2: bool = ((s_101_0) == (s_101_1));
        // N s_101_3: branch s_101_2 b551 b102
        if s_101_2 {
            return block_551(state, tracer, fn_state);
        } else {
            return block_102(state, tracer, fn_state);
        };
    }
    fn block_102<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_102_0: read-var x:str
        let s_102_0: &'static str = fn_state.x;
        // C s_102_1: const #"Have Common Short Sequence Compression instructions extension" : str
        let s_102_1: &'static str = "Have Common Short Sequence Compression instructions extension";
        // D s_102_2: cmp-eq s_102_0 s_102_1
        let s_102_2: bool = ((s_102_0) == (s_102_1));
        // N s_102_3: branch s_102_2 b550 b103
        if s_102_2 {
            return block_550(state, tracer, fn_state);
        } else {
            return block_103(state, tracer, fn_state);
        };
    }
    fn block_103<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_103_0: read-var x:str
        let s_103_0: &'static str = fn_state.x;
        // C s_103_1: const #"Have SVE ISA" : str
        let s_103_1: &'static str = "Have SVE ISA";
        // D s_103_2: cmp-eq s_103_0 s_103_1
        let s_103_2: bool = ((s_103_0) == (s_103_1));
        // N s_103_3: branch s_103_2 b549 b104
        if s_103_2 {
            return block_549(state, tracer, fn_state);
        } else {
            return block_104(state, tracer, fn_state);
        };
    }
    fn block_104<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_104_0: read-var x:str
        let s_104_0: &'static str = fn_state.x;
        // C s_104_1: const #"Have SVE FP32 Matrix Multiply extension" : str
        let s_104_1: &'static str = "Have SVE FP32 Matrix Multiply extension";
        // D s_104_2: cmp-eq s_104_0 s_104_1
        let s_104_2: bool = ((s_104_0) == (s_104_1));
        // N s_104_3: branch s_104_2 b548 b105
        if s_104_2 {
            return block_548(state, tracer, fn_state);
        } else {
            return block_105(state, tracer, fn_state);
        };
    }
    fn block_105<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_105_0: read-var x:str
        let s_105_0: &'static str = fn_state.x;
        // C s_105_1: const #"Have SVE FP64 Matrix Multiply extension" : str
        let s_105_1: &'static str = "Have SVE FP64 Matrix Multiply extension";
        // D s_105_2: cmp-eq s_105_0 s_105_1
        let s_105_2: bool = ((s_105_0) == (s_105_1));
        // N s_105_3: branch s_105_2 b547 b106
        if s_105_2 {
            return block_547(state, tracer, fn_state);
        } else {
            return block_106(state, tracer, fn_state);
        };
    }
    fn block_106<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_106_0: read-var x:str
        let s_106_0: &'static str = fn_state.x;
        // C s_106_1: const #"Have SVE2 AES extension" : str
        let s_106_1: &'static str = "Have SVE2 AES extension";
        // D s_106_2: cmp-eq s_106_0 s_106_1
        let s_106_2: bool = ((s_106_0) == (s_106_1));
        // N s_106_3: branch s_106_2 b546 b107
        if s_106_2 {
            return block_546(state, tracer, fn_state);
        } else {
            return block_107(state, tracer, fn_state);
        };
    }
    fn block_107<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_107_0: read-var x:str
        let s_107_0: &'static str = fn_state.x;
        // C s_107_1: const #"Have SVE2 128 bit PMULL extension" : str
        let s_107_1: &'static str = "Have SVE2 128 bit PMULL extension";
        // D s_107_2: cmp-eq s_107_0 s_107_1
        let s_107_2: bool = ((s_107_0) == (s_107_1));
        // N s_107_3: branch s_107_2 b545 b108
        if s_107_2 {
            return block_545(state, tracer, fn_state);
        } else {
            return block_108(state, tracer, fn_state);
        };
    }
    fn block_108<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_108_0: read-var x:str
        let s_108_0: &'static str = fn_state.x;
        // C s_108_1: const #"Have SVE2 SHA3 extension" : str
        let s_108_1: &'static str = "Have SVE2 SHA3 extension";
        // D s_108_2: cmp-eq s_108_0 s_108_1
        let s_108_2: bool = ((s_108_0) == (s_108_1));
        // N s_108_3: branch s_108_2 b544 b109
        if s_108_2 {
            return block_544(state, tracer, fn_state);
        } else {
            return block_109(state, tracer, fn_state);
        };
    }
    fn block_109<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_109_0: read-var x:str
        let s_109_0: &'static str = fn_state.x;
        // C s_109_1: const #"Have SVE2 SHA256 extension" : str
        let s_109_1: &'static str = "Have SVE2 SHA256 extension";
        // D s_109_2: cmp-eq s_109_0 s_109_1
        let s_109_2: bool = ((s_109_0) == (s_109_1));
        // N s_109_3: branch s_109_2 b543 b110
        if s_109_2 {
            return block_543(state, tracer, fn_state);
        } else {
            return block_110(state, tracer, fn_state);
        };
    }
    fn block_110<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_110_0: read-var x:str
        let s_110_0: &'static str = fn_state.x;
        // C s_110_1: const #"Have SVE2 SHA512 extension" : str
        let s_110_1: &'static str = "Have SVE2 SHA512 extension";
        // D s_110_2: cmp-eq s_110_0 s_110_1
        let s_110_2: bool = ((s_110_0) == (s_110_1));
        // N s_110_3: branch s_110_2 b542 b111
        if s_110_2 {
            return block_542(state, tracer, fn_state);
        } else {
            return block_111(state, tracer, fn_state);
        };
    }
    fn block_111<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_111_0: read-var x:str
        let s_111_0: &'static str = fn_state.x;
        // C s_111_1: const #"Have SVE2 SM4 extension" : str
        let s_111_1: &'static str = "Have SVE2 SM4 extension";
        // D s_111_2: cmp-eq s_111_0 s_111_1
        let s_111_2: bool = ((s_111_0) == (s_111_1));
        // N s_111_3: branch s_111_2 b541 b112
        if s_111_2 {
            return block_541(state, tracer, fn_state);
        } else {
            return block_112(state, tracer, fn_state);
        };
    }
    fn block_112<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_112_0: read-var x:str
        let s_112_0: &'static str = fn_state.x;
        // C s_112_1: const #"Have SVE2 SM3 extension" : str
        let s_112_1: &'static str = "Have SVE2 SM3 extension";
        // D s_112_2: cmp-eq s_112_0 s_112_1
        let s_112_2: bool = ((s_112_0) == (s_112_1));
        // N s_112_3: branch s_112_2 b540 b113
        if s_112_2 {
            return block_540(state, tracer, fn_state);
        } else {
            return block_113(state, tracer, fn_state);
        };
    }
    fn block_113<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_113_0: read-var x:str
        let s_113_0: &'static str = fn_state.x;
        // C s_113_1: const #"Have SVE2GF  extension" : str
        let s_113_1: &'static str = "Have SVE2GF  extension";
        // D s_113_2: cmp-eq s_113_0 s_113_1
        let s_113_2: bool = ((s_113_0) == (s_113_1));
        // N s_113_3: branch s_113_2 b539 b114
        if s_113_2 {
            return block_539(state, tracer, fn_state);
        } else {
            return block_114(state, tracer, fn_state);
        };
    }
    fn block_114<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_114_0: read-var x:str
        let s_114_0: &'static str = fn_state.x;
        // C s_114_1: const #"Have SVE2.1 extension" : str
        let s_114_1: &'static str = "Have SVE2.1 extension";
        // D s_114_2: cmp-eq s_114_0 s_114_1
        let s_114_2: bool = ((s_114_0) == (s_114_1));
        // N s_114_3: branch s_114_2 b538 b115
        if s_114_2 {
            return block_538(state, tracer, fn_state);
        } else {
            return block_115(state, tracer, fn_state);
        };
    }
    fn block_115<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_115_0: read-var x:str
        let s_115_0: &'static str = fn_state.x;
        // C s_115_1: const #"Have SME extension" : str
        let s_115_1: &'static str = "Have SME extension";
        // D s_115_2: cmp-eq s_115_0 s_115_1
        let s_115_2: bool = ((s_115_0) == (s_115_1));
        // N s_115_3: branch s_115_2 b537 b116
        if s_115_2 {
            return block_537(state, tracer, fn_state);
        } else {
            return block_116(state, tracer, fn_state);
        };
    }
    fn block_116<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_116_0: read-var x:str
        let s_116_0: &'static str = fn_state.x;
        // C s_116_1: const #"SME only" : str
        let s_116_1: &'static str = "SME only";
        // D s_116_2: cmp-eq s_116_0 s_116_1
        let s_116_2: bool = ((s_116_0) == (s_116_1));
        // N s_116_3: branch s_116_2 b536 b117
        if s_116_2 {
            return block_536(state, tracer, fn_state);
        } else {
            return block_117(state, tracer, fn_state);
        };
    }
    fn block_117<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_117_0: read-var x:str
        let s_117_0: &'static str = fn_state.x;
        // C s_117_1: const #"Has SME priority control" : str
        let s_117_1: &'static str = "Has SME priority control";
        // D s_117_2: cmp-eq s_117_0 s_117_1
        let s_117_2: bool = ((s_117_0) == (s_117_1));
        // N s_117_3: branch s_117_2 b535 b118
        if s_117_2 {
            return block_535(state, tracer, fn_state);
        } else {
            return block_118(state, tracer, fn_state);
        };
    }
    fn block_118<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_118_0: read-var x:str
        let s_118_0: &'static str = fn_state.x;
        // C s_118_1: const #"Have SME FA64 extension" : str
        let s_118_1: &'static str = "Have SME FA64 extension";
        // D s_118_2: cmp-eq s_118_0 s_118_1
        let s_118_2: bool = ((s_118_0) == (s_118_1));
        // N s_118_3: branch s_118_2 b534 b119
        if s_118_2 {
            return block_534(state, tracer, fn_state);
        } else {
            return block_119(state, tracer, fn_state);
        };
    }
    fn block_119<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_119_0: read-var x:str
        let s_119_0: &'static str = fn_state.x;
        // C s_119_1: const #"No tag checking of SME LDR & STR instructions" : str
        let s_119_1: &'static str = "No tag checking of SME LDR & STR instructions";
        // D s_119_2: cmp-eq s_119_0 s_119_1
        let s_119_2: bool = ((s_119_0) == (s_119_1));
        // N s_119_3: branch s_119_2 b533 b120
        if s_119_2 {
            return block_533(state, tracer, fn_state);
        } else {
            return block_120(state, tracer, fn_state);
        };
    }
    fn block_120<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_120_0: read-var x:str
        let s_120_0: &'static str = fn_state.x;
        // C s_120_1: const #"Shared SMCU" : str
        let s_120_1: &'static str = "Shared SMCU";
        // D s_120_2: cmp-eq s_120_0 s_120_1
        let s_120_2: bool = ((s_120_0) == (s_120_1));
        // N s_120_3: branch s_120_2 b532 b121
        if s_120_2 {
            return block_532(state, tracer, fn_state);
        } else {
            return block_121(state, tracer, fn_state);
        };
    }
    fn block_121<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_121_0: read-var x:str
        let s_121_0: &'static str = fn_state.x;
        // C s_121_1: const #"MPAMSM_EL1 label precedence" : str
        let s_121_1: &'static str = "MPAMSM_EL1 label precedence";
        // D s_121_2: cmp-eq s_121_0 s_121_1
        let s_121_2: bool = ((s_121_0) == (s_121_1));
        // N s_121_3: branch s_121_2 b531 b122
        if s_121_2 {
            return block_531(state, tracer, fn_state);
        } else {
            return block_122(state, tracer, fn_state);
        };
    }
    fn block_122<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_122_0: read-var x:str
        let s_122_0: &'static str = fn_state.x;
        // C s_122_1: const #"Have EBF16 extension" : str
        let s_122_1: &'static str = "Have EBF16 extension";
        // D s_122_2: cmp-eq s_122_0 s_122_1
        let s_122_2: bool = ((s_122_0) == (s_122_1));
        // N s_122_3: branch s_122_2 b530 b123
        if s_122_2 {
            return block_530(state, tracer, fn_state);
        } else {
            return block_123(state, tracer, fn_state);
        };
    }
    fn block_123<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_123_0: read-var x:str
        let s_123_0: &'static str = fn_state.x;
        // C s_123_1: const #"Have SME2 extension" : str
        let s_123_1: &'static str = "Have SME2 extension";
        // D s_123_2: cmp-eq s_123_0 s_123_1
        let s_123_2: bool = ((s_123_0) == (s_123_1));
        // N s_123_3: branch s_123_2 b529 b124
        if s_123_2 {
            return block_529(state, tracer, fn_state);
        } else {
            return block_124(state, tracer, fn_state);
        };
    }
    fn block_124<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_124_0: read-var x:str
        let s_124_0: &'static str = fn_state.x;
        // C s_124_1: const #"Have SME2.1 extension" : str
        let s_124_1: &'static str = "Have SME2.1 extension";
        // D s_124_2: cmp-eq s_124_0 s_124_1
        let s_124_2: bool = ((s_124_0) == (s_124_1));
        // N s_124_3: branch s_124_2 b528 b125
        if s_124_2 {
            return block_528(state, tracer, fn_state);
        } else {
            return block_125(state, tracer, fn_state);
        };
    }
    fn block_125<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_125_0: read-var x:str
        let s_125_0: &'static str = fn_state.x;
        // C s_125_1: const #"Have SME2.1 half-precision floating-point instructions" : str
        let s_125_1: &'static str = "Have SME2.1 half-precision floating-point instructions";
        // D s_125_2: cmp-eq s_125_0 s_125_1
        let s_125_2: bool = ((s_125_0) == (s_125_1));
        // N s_125_3: branch s_125_2 b527 b126
        if s_125_2 {
            return block_527(state, tracer, fn_state);
        } else {
            return block_126(state, tracer, fn_state);
        };
    }
    fn block_126<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_126_0: read-var x:str
        let s_126_0: &'static str = fn_state.x;
        // C s_126_1: const #"Have non-widening BFloat16 instructions" : str
        let s_126_1: &'static str = "Have non-widening BFloat16 instructions";
        // D s_126_2: cmp-eq s_126_0 s_126_1
        let s_126_2: bool = ((s_126_0) == (s_126_1));
        // N s_126_3: branch s_126_2 b526 b127
        if s_126_2 {
            return block_526(state, tracer, fn_state);
        } else {
            return block_127(state, tracer, fn_state);
        };
    }
    fn block_127<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_127_0: read-var x:str
        let s_127_0: &'static str = fn_state.x;
        // C s_127_1: const #"Has Transactional Memory extension" : str
        let s_127_1: &'static str = "Has Transactional Memory extension";
        // D s_127_2: cmp-eq s_127_0 s_127_1
        let s_127_2: bool = ((s_127_0) == (s_127_1));
        // N s_127_3: branch s_127_2 b525 b128
        if s_127_2 {
            return block_525(state, tracer, fn_state);
        } else {
            return block_128(state, tracer, fn_state);
        };
    }
    fn block_128<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_128_0: read-var x:str
        let s_128_0: &'static str = fn_state.x;
        // C s_128_1: const #"Memory Region does not support Transactional access" : str
        let s_128_1: &'static str = "Memory Region does not support Transactional access";
        // D s_128_2: cmp-eq s_128_0 s_128_1
        let s_128_2: bool = ((s_128_0) == (s_128_1));
        // N s_128_3: branch s_128_2 b524 b129
        if s_128_2 {
            return block_524(state, tracer, fn_state);
        } else {
            return block_129(state, tracer, fn_state);
        };
    }
    fn block_129<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_129_0: read-var x:str
        let s_129_0: &'static str = fn_state.x;
        // C s_129_1: const #"Has AArch64 BFloat16 extension" : str
        let s_129_1: &'static str = "Has AArch64 BFloat16 extension";
        // D s_129_2: cmp-eq s_129_0 s_129_1
        let s_129_2: bool = ((s_129_0) == (s_129_1));
        // N s_129_3: branch s_129_2 b523 b130
        if s_129_2 {
            return block_523(state, tracer, fn_state);
        } else {
            return block_130(state, tracer, fn_state);
        };
    }
    fn block_130<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_130_0: read-var x:str
        let s_130_0: &'static str = fn_state.x;
        // C s_130_1: const #"Has TWED extension" : str
        let s_130_1: &'static str = "Has TWED extension";
        // D s_130_2: cmp-eq s_130_0 s_130_1
        let s_130_2: bool = ((s_130_0) == (s_130_1));
        // N s_130_3: branch s_130_2 b522 b131
        if s_130_2 {
            return block_522(state, tracer, fn_state);
        } else {
            return block_131(state, tracer, fn_state);
        };
    }
    fn block_131<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_131_0: read-var x:str
        let s_131_0: &'static str = fn_state.x;
        // C s_131_1: const #"Has AArch32 BFloat16 extension" : str
        let s_131_1: &'static str = "Has AArch32 BFloat16 extension";
        // D s_131_2: cmp-eq s_131_0 s_131_1
        let s_131_2: bool = ((s_131_0) == (s_131_1));
        // N s_131_3: branch s_131_2 b521 b132
        if s_131_2 {
            return block_521(state, tracer, fn_state);
        } else {
            return block_132(state, tracer, fn_state);
        };
    }
    fn block_132<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_132_0: read-var x:str
        let s_132_0: &'static str = fn_state.x;
        // C s_132_1: const #"Has AArch64 Int8 Mat Mul extension" : str
        let s_132_1: &'static str = "Has AArch64 Int8 Mat Mul extension";
        // D s_132_2: cmp-eq s_132_0 s_132_1
        let s_132_2: bool = ((s_132_0) == (s_132_1));
        // N s_132_3: branch s_132_2 b520 b133
        if s_132_2 {
            return block_520(state, tracer, fn_state);
        } else {
            return block_133(state, tracer, fn_state);
        };
    }
    fn block_133<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_133_0: read-var x:str
        let s_133_0: &'static str = fn_state.x;
        // C s_133_1: const #"Has AArch32 Int8 Mat Mul extension" : str
        let s_133_1: &'static str = "Has AArch32 Int8 Mat Mul extension";
        // D s_133_2: cmp-eq s_133_0 s_133_1
        let s_133_2: bool = ((s_133_0) == (s_133_1));
        // N s_133_3: branch s_133_2 b519 b134
        if s_133_2 {
            return block_519(state, tracer, fn_state);
        } else {
            return block_134(state, tracer, fn_state);
        };
    }
    fn block_134<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_134_0: read-var x:str
        let s_134_0: &'static str = fn_state.x;
        // C s_134_1: const #"ID_MMFR4_EL1 trapped by HCR_EL2.TID3" : str
        let s_134_1: &'static str = "ID_MMFR4_EL1 trapped by HCR_EL2.TID3";
        // D s_134_2: cmp-eq s_134_0 s_134_1
        let s_134_2: bool = ((s_134_0) == (s_134_1));
        // N s_134_3: branch s_134_2 b518 b135
        if s_134_2 {
            return block_518(state, tracer, fn_state);
        } else {
            return block_135(state, tracer, fn_state);
        };
    }
    fn block_135<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_135_0: read-var x:str
        let s_135_0: &'static str = fn_state.x;
        // C s_135_1: const #"ID_ISAR6_EL1 trapped by HCR_EL2.TID3" : str
        let s_135_1: &'static str = "ID_ISAR6_EL1 trapped by HCR_EL2.TID3";
        // D s_135_2: cmp-eq s_135_0 s_135_1
        let s_135_2: bool = ((s_135_0) == (s_135_1));
        // N s_135_3: branch s_135_2 b517 b136
        if s_135_2 {
            return block_517(state, tracer, fn_state);
        } else {
            return block_136(state, tracer, fn_state);
        };
    }
    fn block_136<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_136_0: read-var x:str
        let s_136_0: &'static str = fn_state.x;
        // C s_136_1: const #"ID_AA64MMFR4_EL1 trapped by HCR_EL2.TID3" : str
        let s_136_1: &'static str = "ID_AA64MMFR4_EL1 trapped by HCR_EL2.TID3";
        // D s_136_2: cmp-eq s_136_0 s_136_1
        let s_136_2: bool = ((s_136_0) == (s_136_1));
        // N s_136_3: branch s_136_2 b516 b137
        if s_136_2 {
            return block_516(state, tracer, fn_state);
        } else {
            return block_137(state, tracer, fn_state);
        };
    }
    fn block_137<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_137_0: read-var x:str
        let s_137_0: &'static str = fn_state.x;
        // C s_137_1: const #"ID_AA64SMFR0_EL1 trapped by HCR_EL2.TID3" : str
        let s_137_1: &'static str = "ID_AA64SMFR0_EL1 trapped by HCR_EL2.TID3";
        // D s_137_2: cmp-eq s_137_0 s_137_1
        let s_137_2: bool = ((s_137_0) == (s_137_1));
        // N s_137_3: branch s_137_2 b515 b138
        if s_137_2 {
            return block_515(state, tracer, fn_state);
        } else {
            return block_138(state, tracer, fn_state);
        };
    }
    fn block_138<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_138_0: read-var x:str
        let s_138_0: &'static str = fn_state.x;
        // C s_138_1: const #"Has Branch Record Buffer Extension" : str
        let s_138_1: &'static str = "Has Branch Record Buffer Extension";
        // D s_138_2: cmp-eq s_138_0 s_138_1
        let s_138_2: bool = ((s_138_0) == (s_138_1));
        // N s_138_3: branch s_138_2 b514 b139
        if s_138_2 {
            return block_514(state, tracer, fn_state);
        } else {
            return block_139(state, tracer, fn_state);
        };
    }
    fn block_139<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_139_0: read-var x:str
        let s_139_0: &'static str = fn_state.x;
        // C s_139_1: const #"ISB generates Branch records" : str
        let s_139_1: &'static str = "ISB generates Branch records";
        // D s_139_2: cmp-eq s_139_0 s_139_1
        let s_139_2: bool = ((s_139_0) == (s_139_1));
        // N s_139_3: branch s_139_2 b513 b140
        if s_139_2 {
            return block_513(state, tracer, fn_state);
        } else {
            return block_140(state, tracer, fn_state);
        };
    }
    fn block_140<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_140_0: read-var x:str
        let s_140_0: &'static str = fn_state.x;
        // C s_140_1: const #"Has BRBEv1p1 extension" : str
        let s_140_1: &'static str = "Has BRBEv1p1 extension";
        // D s_140_2: cmp-eq s_140_0 s_140_1
        let s_140_2: bool = ((s_140_0) == (s_140_1));
        // N s_140_3: branch s_140_2 b512 b141
        if s_140_2 {
            return block_512(state, tracer, fn_state);
        } else {
            return block_141(state, tracer, fn_state);
        };
    }
    fn block_141<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_141_0: read-var x:str
        let s_141_0: &'static str = fn_state.x;
        // C s_141_1: const #"ID_AA64ZFR0_EL1 trapped by HCR_EL2.TID3" : str
        let s_141_1: &'static str = "ID_AA64ZFR0_EL1 trapped by HCR_EL2.TID3";
        // D s_141_2: cmp-eq s_141_0 s_141_1
        let s_141_2: bool = ((s_141_0) == (s_141_1));
        // N s_141_3: branch s_141_2 b511 b142
        if s_141_2 {
            return block_511(state, tracer, fn_state);
        } else {
            return block_142(state, tracer, fn_state);
        };
    }
    fn block_142<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_142_0: read-var x:str
        let s_142_0: &'static str = fn_state.x;
        // C s_142_1: const #"ID_DFR1_EL1 trapped by HCR_EL2.TID3" : str
        let s_142_1: &'static str = "ID_DFR1_EL1 trapped by HCR_EL2.TID3";
        // D s_142_2: cmp-eq s_142_0 s_142_1
        let s_142_2: bool = ((s_142_0) == (s_142_1));
        // N s_142_3: branch s_142_2 b510 b143
        if s_142_2 {
            return block_510(state, tracer, fn_state);
        } else {
            return block_143(state, tracer, fn_state);
        };
    }
    fn block_143<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_143_0: read-var x:str
        let s_143_0: &'static str = fn_state.x;
        // C s_143_1: const #"ID_DFR1 trapped by HCR.TID3" : str
        let s_143_1: &'static str = "ID_DFR1 trapped by HCR.TID3";
        // D s_143_2: cmp-eq s_143_0 s_143_1
        let s_143_2: bool = ((s_143_0) == (s_143_1));
        // N s_143_3: branch s_143_2 b509 b144
        if s_143_2 {
            return block_509(state, tracer, fn_state);
        } else {
            return block_144(state, tracer, fn_state);
        };
    }
    fn block_144<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_144_0: read-var x:str
        let s_144_0: &'static str = fn_state.x;
        // C s_144_1: const #"ID_PFR2_EL1 trapped by HCR_EL2.TID3" : str
        let s_144_1: &'static str = "ID_PFR2_EL1 trapped by HCR_EL2.TID3";
        // D s_144_2: cmp-eq s_144_0 s_144_1
        let s_144_2: bool = ((s_144_0) == (s_144_1));
        // N s_144_3: branch s_144_2 b508 b145
        if s_144_2 {
            return block_508(state, tracer, fn_state);
        } else {
            return block_145(state, tracer, fn_state);
        };
    }
    fn block_145<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_145_0: read-var x:str
        let s_145_0: &'static str = fn_state.x;
        // C s_145_1: const #"Unallocated encodings trapped by HCR_EL2.TID3" : str
        let s_145_1: &'static str = "Unallocated encodings trapped by HCR_EL2.TID3";
        // D s_145_2: cmp-eq s_145_0 s_145_1
        let s_145_2: bool = ((s_145_0) == (s_145_1));
        // N s_145_3: branch s_145_2 b507 b146
        if s_145_2 {
            return block_507(state, tracer, fn_state);
        } else {
            return block_146(state, tracer, fn_state);
        };
    }
    fn block_146<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_146_0: read-var x:str
        let s_146_0: &'static str = fn_state.x;
        // C s_146_1: const #"Report abort using Long-descriptor format" : str
        let s_146_1: &'static str = "Report abort using Long-descriptor format";
        // D s_146_2: cmp-eq s_146_0 s_146_1
        let s_146_2: bool = ((s_146_0) == (s_146_1));
        // N s_146_3: branch s_146_2 b506 b147
        if s_146_2 {
            return block_506(state, tracer, fn_state);
        } else {
            return block_147(state, tracer, fn_state);
        };
    }
    fn block_147<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_147_0: read-var x:str
        let s_147_0: &'static str = fn_state.x;
        // C s_147_1: const #"Fault on TxSZ value above maximum" : str
        let s_147_1: &'static str = "Fault on TxSZ value above maximum";
        // D s_147_2: cmp-eq s_147_0 s_147_1
        let s_147_2: bool = ((s_147_0) == (s_147_1));
        // N s_147_3: branch s_147_2 b505 b148
        if s_147_2 {
            return block_505(state, tracer, fn_state);
        } else {
            return block_148(state, tracer, fn_state);
        };
    }
    fn block_148<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_148_0: read-var x:str
        let s_148_0: &'static str = fn_state.x;
        // C s_148_1: const #"Fault on TxSZ value below minimum" : str
        let s_148_1: &'static str = "Fault on TxSZ value below minimum";
        // D s_148_2: cmp-eq s_148_0 s_148_1
        let s_148_2: bool = ((s_148_0) == (s_148_1));
        // N s_148_3: branch s_148_2 b504 b149
        if s_148_2 {
            return block_504(state, tracer, fn_state);
        } else {
            return block_149(state, tracer, fn_state);
        };
    }
    fn block_149<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_149_0: read-var x:str
        let s_149_0: &'static str = fn_state.x;
        // C s_149_1: const #"Has 52-bit IPA and PA support" : str
        let s_149_1: &'static str = "Has 52-bit IPA and PA support";
        // D s_149_2: cmp-eq s_149_0 s_149_1
        let s_149_2: bool = ((s_149_0) == (s_149_1));
        // N s_149_3: branch s_149_2 b503 b150
        if s_149_2 {
            return block_503(state, tracer, fn_state);
        } else {
            return block_150(state, tracer, fn_state);
        };
    }
    fn block_150<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_150_0: read-var x:str
        let s_150_0: &'static str = fn_state.x;
        // C s_150_1: const #"Has 56-bit PA support" : str
        let s_150_1: &'static str = "Has 56-bit PA support";
        // D s_150_2: cmp-eq s_150_0 s_150_1
        let s_150_2: bool = ((s_150_0) == (s_150_1));
        // N s_150_3: branch s_150_2 b502 b151
        if s_150_2 {
            return block_502(state, tracer, fn_state);
        } else {
            return block_151(state, tracer, fn_state);
        };
    }
    fn block_151<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_151_0: read-var x:str
        let s_151_0: &'static str = fn_state.x;
        // C s_151_1: const #"Has 56-bit VA support" : str
        let s_151_1: &'static str = "Has 56-bit VA support";
        // D s_151_2: cmp-eq s_151_0 s_151_1
        let s_151_2: bool = ((s_151_0) == (s_151_1));
        // N s_151_3: branch s_151_2 b501 b152
        if s_151_2 {
            return block_501(state, tracer, fn_state);
        } else {
            return block_152(state, tracer, fn_state);
        };
    }
    fn block_152<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_152_0: read-var x:str
        let s_152_0: &'static str = fn_state.x;
        // C s_152_1: const #"Has 128-bit Descriptor support" : str
        let s_152_1: &'static str = "Has 128-bit Descriptor support";
        // D s_152_2: cmp-eq s_152_0 s_152_1
        let s_152_2: bool = ((s_152_0) == (s_152_1));
        // N s_152_3: branch s_152_2 b500 b153
        if s_152_2 {
            return block_500(state, tracer, fn_state);
        } else {
            return block_153(state, tracer, fn_state);
        };
    }
    fn block_153<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_153_0: read-var x:str
        let s_153_0: &'static str = fn_state.x;
        // C s_153_1: const #"Has S1 Permission Indirection support" : str
        let s_153_1: &'static str = "Has S1 Permission Indirection support";
        // D s_153_2: cmp-eq s_153_0 s_153_1
        let s_153_2: bool = ((s_153_0) == (s_153_1));
        // N s_153_3: branch s_153_2 b499 b154
        if s_153_2 {
            return block_499(state, tracer, fn_state);
        } else {
            return block_154(state, tracer, fn_state);
        };
    }
    fn block_154<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_154_0: read-var x:str
        let s_154_0: &'static str = fn_state.x;
        // C s_154_1: const #"Has S1 Permission Overlay support" : str
        let s_154_1: &'static str = "Has S1 Permission Overlay support";
        // D s_154_2: cmp-eq s_154_0 s_154_1
        let s_154_2: bool = ((s_154_0) == (s_154_1));
        // N s_154_3: branch s_154_2 b498 b155
        if s_154_2 {
            return block_498(state, tracer, fn_state);
        } else {
            return block_155(state, tracer, fn_state);
        };
    }
    fn block_155<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_155_0: read-var x:str
        let s_155_0: &'static str = fn_state.x;
        // C s_155_1: const #"Has S2 Permission Indirection support" : str
        let s_155_1: &'static str = "Has S2 Permission Indirection support";
        // D s_155_2: cmp-eq s_155_0 s_155_1
        let s_155_2: bool = ((s_155_0) == (s_155_1));
        // N s_155_3: branch s_155_2 b497 b156
        if s_155_2 {
            return block_497(state, tracer, fn_state);
        } else {
            return block_156(state, tracer, fn_state);
        };
    }
    fn block_156<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_156_0: read-var x:str
        let s_156_0: &'static str = fn_state.x;
        // C s_156_1: const #"Has S2 Permission Overlay support" : str
        let s_156_1: &'static str = "Has S2 Permission Overlay support";
        // D s_156_2: cmp-eq s_156_0 s_156_1
        let s_156_2: bool = ((s_156_0) == (s_156_1));
        // N s_156_3: branch s_156_2 b496 b157
        if s_156_2 {
            return block_496(state, tracer, fn_state);
        } else {
            return block_157(state, tracer, fn_state);
        };
    }
    fn block_157<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_157_0: read-var x:str
        let s_157_0: &'static str = fn_state.x;
        // C s_157_1: const #"Has Translation Hardening Extension" : str
        let s_157_1: &'static str = "Has Translation Hardening Extension";
        // D s_157_2: cmp-eq s_157_0 s_157_1
        let s_157_2: bool = ((s_157_0) == (s_157_1));
        // N s_157_3: branch s_157_2 b495 b158
        if s_157_2 {
            return block_495(state, tracer, fn_state);
        } else {
            return block_158(state, tracer, fn_state);
        };
    }
    fn block_158<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_158_0: read-var x:str
        let s_158_0: &'static str = fn_state.x;
        // C s_158_1: const #"Has LSE128" : str
        let s_158_1: &'static str = "Has LSE128";
        // D s_158_2: cmp-eq s_158_0 s_158_1
        let s_158_2: bool = ((s_158_0) == (s_158_1));
        // N s_158_3: branch s_158_2 b494 b159
        if s_158_2 {
            return block_494(state, tracer, fn_state);
        } else {
            return block_159(state, tracer, fn_state);
        };
    }
    fn block_159<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_159_0: read-var x:str
        let s_159_0: &'static str = fn_state.x;
        // C s_159_1: const #"Has SysReg128" : str
        let s_159_1: &'static str = "Has SysReg128";
        // D s_159_2: cmp-eq s_159_0 s_159_1
        let s_159_2: bool = ((s_159_0) == (s_159_1));
        // N s_159_3: branch s_159_2 b493 b160
        if s_159_2 {
            return block_493(state, tracer, fn_state);
        } else {
            return block_160(state, tracer, fn_state);
        };
    }
    fn block_160<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_160_0: read-var x:str
        let s_160_0: &'static str = fn_state.x;
        // C s_160_1: const #"Has SysInstr128" : str
        let s_160_1: &'static str = "Has SysInstr128";
        // D s_160_2: cmp-eq s_160_0 s_160_1
        let s_160_2: bool = ((s_160_0) == (s_160_1));
        // N s_160_3: branch s_160_2 b492 b161
        if s_160_2 {
            return block_492(state, tracer, fn_state);
        } else {
            return block_161(state, tracer, fn_state);
        };
    }
    fn block_161<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_161_0: read-var x:str
        let s_161_0: &'static str = fn_state.x;
        // C s_161_1: const #"Arch has TCR2" : str
        let s_161_1: &'static str = "Arch has TCR2";
        // D s_161_2: cmp-eq s_161_0 s_161_1
        let s_161_2: bool = ((s_161_0) == (s_161_1));
        // N s_161_3: branch s_161_2 b491 b162
        if s_161_2 {
            return block_491(state, tracer, fn_state);
        } else {
            return block_162(state, tracer, fn_state);
        };
    }
    fn block_162<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_162_0: read-var x:str
        let s_162_0: &'static str = fn_state.x;
        // C s_162_1: const #"Arch has SCTLR2" : str
        let s_162_1: &'static str = "Arch has SCTLR2";
        // D s_162_2: cmp-eq s_162_0 s_162_1
        let s_162_2: bool = ((s_162_0) == (s_162_1));
        // N s_162_3: branch s_162_2 b490 b163
        if s_162_2 {
            return block_490(state, tracer, fn_state);
        } else {
            return block_163(state, tracer, fn_state);
        };
    }
    fn block_163<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_163_0: read-var x:str
        let s_163_0: &'static str = fn_state.x;
        // C s_163_1: const #"Has 4K Translation Granule" : str
        let s_163_1: &'static str = "Has 4K Translation Granule";
        // D s_163_2: cmp-eq s_163_0 s_163_1
        let s_163_2: bool = ((s_163_0) == (s_163_1));
        // N s_163_3: branch s_163_2 b489 b164
        if s_163_2 {
            return block_489(state, tracer, fn_state);
        } else {
            return block_164(state, tracer, fn_state);
        };
    }
    fn block_164<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_164_0: read-var x:str
        let s_164_0: &'static str = fn_state.x;
        // C s_164_1: const #"Has 16K Translation Granule" : str
        let s_164_1: &'static str = "Has 16K Translation Granule";
        // D s_164_2: cmp-eq s_164_0 s_164_1
        let s_164_2: bool = ((s_164_0) == (s_164_1));
        // N s_164_3: branch s_164_2 b488 b165
        if s_164_2 {
            return block_488(state, tracer, fn_state);
        } else {
            return block_165(state, tracer, fn_state);
        };
    }
    fn block_165<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_165_0: read-var x:str
        let s_165_0: &'static str = fn_state.x;
        // C s_165_1: const #"Has 64K Translation Granule" : str
        let s_165_1: &'static str = "Has 64K Translation Granule";
        // D s_165_2: cmp-eq s_165_0 s_165_1
        let s_165_2: bool = ((s_165_0) == (s_165_1));
        // N s_165_3: branch s_165_2 b487 b166
        if s_165_2 {
            return block_487(state, tracer, fn_state);
        } else {
            return block_166(state, tracer, fn_state);
        };
    }
    fn block_166<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_166_0: read-var x:str
        let s_166_0: &'static str = fn_state.x;
        // C s_166_1: const #"Has Stage 2 4K Translation Granule" : str
        let s_166_1: &'static str = "Has Stage 2 4K Translation Granule";
        // D s_166_2: cmp-eq s_166_0 s_166_1
        let s_166_2: bool = ((s_166_0) == (s_166_1));
        // N s_166_3: branch s_166_2 b486 b167
        if s_166_2 {
            return block_486(state, tracer, fn_state);
        } else {
            return block_167(state, tracer, fn_state);
        };
    }
    fn block_167<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_167_0: read-var x:str
        let s_167_0: &'static str = fn_state.x;
        // C s_167_1: const #"Has Stage 2 16K Translation Granule" : str
        let s_167_1: &'static str = "Has Stage 2 16K Translation Granule";
        // D s_167_2: cmp-eq s_167_0 s_167_1
        let s_167_2: bool = ((s_167_0) == (s_167_1));
        // N s_167_3: branch s_167_2 b485 b168
        if s_167_2 {
            return block_485(state, tracer, fn_state);
        } else {
            return block_168(state, tracer, fn_state);
        };
    }
    fn block_168<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_168_0: read-var x:str
        let s_168_0: &'static str = fn_state.x;
        // C s_168_1: const #"Has Stage 2 64K Translation Granule" : str
        let s_168_1: &'static str = "Has Stage 2 64K Translation Granule";
        // D s_168_2: cmp-eq s_168_0 s_168_1
        let s_168_2: bool = ((s_168_0) == (s_168_1));
        // N s_168_3: branch s_168_2 b484 b169
        if s_168_2 {
            return block_484(state, tracer, fn_state);
        } else {
            return block_169(state, tracer, fn_state);
        };
    }
    fn block_169<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_169_0: read-var x:str
        let s_169_0: &'static str = fn_state.x;
        // C s_169_1: const #"Has PAN3 extension" : str
        let s_169_1: &'static str = "Has PAN3 extension";
        // D s_169_2: cmp-eq s_169_0 s_169_1
        let s_169_2: bool = ((s_169_0) == (s_169_1));
        // N s_169_3: branch s_169_2 b483 b170
        if s_169_2 {
            return block_483(state, tracer, fn_state);
        } else {
            return block_170(state, tracer, fn_state);
        };
    }
    fn block_170<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_170_0: read-var x:str
        let s_170_0: &'static str = fn_state.x;
        // C s_170_1: const #"SCR_EL3.SIF affects EPAN" : str
        let s_170_1: &'static str = "SCR_EL3.SIF affects EPAN";
        // D s_170_2: cmp-eq s_170_0 s_170_1
        let s_170_2: bool = ((s_170_0) == (s_170_1));
        // N s_170_3: branch s_170_2 b482 b171
        if s_170_2 {
            return block_482(state, tracer, fn_state);
        } else {
            return block_171(state, tracer, fn_state);
        };
    }
    fn block_171<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_171_0: read-var x:str
        let s_171_0: &'static str = fn_state.x;
        // C s_171_1: const #"Has MTE3 extension" : str
        let s_171_1: &'static str = "Has MTE3 extension";
        // D s_171_2: cmp-eq s_171_0 s_171_1
        let s_171_2: bool = ((s_171_0) == (s_171_1));
        // N s_171_3: branch s_171_2 b481 b172
        if s_171_2 {
            return block_481(state, tracer, fn_state);
        } else {
            return block_172(state, tracer, fn_state);
        };
    }
    fn block_172<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_172_0: read-var x:str
        let s_172_0: &'static str = fn_state.x;
        // C s_172_1: const #"Has MTE Asymmetric Faults extension" : str
        let s_172_1: &'static str = "Has MTE Asymmetric Faults extension";
        // D s_172_2: cmp-eq s_172_0 s_172_1
        let s_172_2: bool = ((s_172_0) == (s_172_1));
        // N s_172_3: branch s_172_2 b480 b173
        if s_172_2 {
            return block_480(state, tracer, fn_state);
        } else {
            return block_173(state, tracer, fn_state);
        };
    }
    fn block_173<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_173_0: read-var x:str
        let s_173_0: &'static str = fn_state.x;
        // C s_173_1: const #"Has MTE tag access permission extension" : str
        let s_173_1: &'static str = "Has MTE tag access permission extension";
        // D s_173_2: cmp-eq s_173_0 s_173_1
        let s_173_2: bool = ((s_173_0) == (s_173_1));
        // N s_173_3: branch s_173_2 b479 b174
        if s_173_2 {
            return block_479(state, tracer, fn_state);
        } else {
            return block_174(state, tracer, fn_state);
        };
    }
    fn block_174<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_174_0: read-var x:str
        let s_174_0: &'static str = fn_state.x;
        // C s_174_1: const #"Has Load Store 64-Byte instruction support" : str
        let s_174_1: &'static str = "Has Load Store 64-Byte instruction support";
        // D s_174_2: cmp-eq s_174_0 s_174_1
        let s_174_2: bool = ((s_174_0) == (s_174_1));
        // N s_174_3: branch s_174_2 b478 b175
        if s_174_2 {
            return block_478(state, tracer, fn_state);
        } else {
            return block_175(state, tracer, fn_state);
        };
    }
    fn block_175<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_175_0: read-var x:str
        let s_175_0: &'static str = fn_state.x;
        // C s_175_1: const #"Memory location supports ST64B and LD64B" : str
        let s_175_1: &'static str = "Memory location supports ST64B and LD64B";
        // D s_175_2: cmp-eq s_175_0 s_175_1
        let s_175_2: bool = ((s_175_0) == (s_175_1));
        // N s_175_3: branch s_175_2 b477 b176
        if s_175_2 {
            return block_477(state, tracer, fn_state);
        } else {
            return block_176(state, tracer, fn_state);
        };
    }
    fn block_176<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_176_0: read-var x:str
        let s_176_0: &'static str = fn_state.x;
        // C s_176_1: const #"Has Store 64-Byte with return instruction support" : str
        let s_176_1: &'static str = "Has Store 64-Byte with return instruction support";
        // D s_176_2: cmp-eq s_176_0 s_176_1
        let s_176_2: bool = ((s_176_0) == (s_176_1));
        // N s_176_3: branch s_176_2 b476 b177
        if s_176_2 {
            return block_476(state, tracer, fn_state);
        } else {
            return block_177(state, tracer, fn_state);
        };
    }
    fn block_177<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_177_0: read-var x:str
        let s_177_0: &'static str = fn_state.x;
        // C s_177_1: const #"Has Store 64-Byte EL0 with return instruction support" : str
        let s_177_1: &'static str = "Has Store 64-Byte EL0 with return instruction support";
        // D s_177_2: cmp-eq s_177_0 s_177_1
        let s_177_2: bool = ((s_177_0) == (s_177_1));
        // N s_177_3: branch s_177_2 b475 b178
        if s_177_2 {
            return block_475(state, tracer, fn_state);
        } else {
            return block_178(state, tracer, fn_state);
        };
    }
    fn block_178<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_178_0: read-var x:str
        let s_178_0: &'static str = fn_state.x;
        // C s_178_1: const #"Has increased Reciprocal Estimate and Square Root Estimate precision support" : str
        let s_178_1: &'static str = "Has increased Reciprocal Estimate and Square Root Estimate precision support";
        // D s_178_2: cmp-eq s_178_0 s_178_1
        let s_178_2: bool = ((s_178_0) == (s_178_1));
        // N s_178_3: branch s_178_2 b474 b179
        if s_178_2 {
            return block_474(state, tracer, fn_state);
        } else {
            return block_179(state, tracer, fn_state);
        };
    }
    fn block_179<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_179_0: read-var x:str
        let s_179_0: &'static str = fn_state.x;
        // C s_179_1: const #"Has SPEv1p1 extension" : str
        let s_179_1: &'static str = "Has SPEv1p1 extension";
        // D s_179_2: cmp-eq s_179_0 s_179_1
        let s_179_2: bool = ((s_179_0) == (s_179_1));
        // N s_179_3: branch s_179_2 b473 b180
        if s_179_2 {
            return block_473(state, tracer, fn_state);
        } else {
            return block_180(state, tracer, fn_state);
        };
    }
    fn block_180<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_180_0: read-var x:str
        let s_180_0: &'static str = fn_state.x;
        // C s_180_1: const #"SPE SyncExternal as SError" : str
        let s_180_1: &'static str = "SPE SyncExternal as SError";
        // D s_180_2: cmp-eq s_180_0 s_180_1
        let s_180_2: bool = ((s_180_0) == (s_180_1));
        // N s_180_3: branch s_180_2 b472 b181
        if s_180_2 {
            return block_472(state, tracer, fn_state);
        } else {
            return block_181(state, tracer, fn_state);
        };
    }
    fn block_181<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_181_0: read-var x:str
        let s_181_0: &'static str = fn_state.x;
        // C s_181_1: const #"SPE async External abort" : str
        let s_181_1: &'static str = "SPE async External abort";
        // D s_181_2: cmp-eq s_181_0 s_181_1
        let s_181_2: bool = ((s_181_0) == (s_181_1));
        // N s_181_3: branch s_181_2 b471 b182
        if s_181_2 {
            return block_471(state, tracer, fn_state);
        } else {
            return block_182(state, tracer, fn_state);
        };
    }
    fn block_182<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_182_0: read-var x:str
        let s_182_0: &'static str = fn_state.x;
        // C s_182_1: const #"Has Implicit Error Synchronization Barrier before Exception" : str
        let s_182_1: &'static str = "Has Implicit Error Synchronization Barrier before Exception";
        // D s_182_2: cmp-eq s_182_0 s_182_1
        let s_182_2: bool = ((s_182_0) == (s_182_1));
        // N s_182_3: branch s_182_2 b470 b183
        if s_182_2 {
            return block_470(state, tracer, fn_state);
        } else {
            return block_183(state, tracer, fn_state);
        };
    }
    fn block_183<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_183_0: read-var x:str
        let s_183_0: &'static str = fn_state.x;
        // C s_183_1: const #"SPE TTW fault External abort" : str
        let s_183_1: &'static str = "SPE TTW fault External abort";
        // D s_183_2: cmp-eq s_183_0 s_183_1
        let s_183_2: bool = ((s_183_0) == (s_183_1));
        // N s_183_3: branch s_183_2 b469 b184
        if s_183_2 {
            return block_469(state, tracer, fn_state);
        } else {
            return block_184(state, tracer, fn_state);
        };
    }
    fn block_184<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_184_0: read-var x:str
        let s_184_0: &'static str = fn_state.x;
        // C s_184_1: const #"Has SPEv1p2 extension" : str
        let s_184_1: &'static str = "Has SPEv1p2 extension";
        // D s_184_2: cmp-eq s_184_0 s_184_1
        let s_184_2: bool = ((s_184_0) == (s_184_1));
        // N s_184_3: branch s_184_2 b468 b185
        if s_184_2 {
            return block_468(state, tracer, fn_state);
        } else {
            return block_185(state, tracer, fn_state);
        };
    }
    fn block_185<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_185_0: read-var x:str
        let s_185_0: &'static str = fn_state.x;
        // C s_185_1: const #"event [x] is not implemented, or filtering on event [x] is not supported" : str
        let s_185_1: &'static str = "event [x] is not implemented, or filtering on event [x] is not supported";
        // D s_185_2: cmp-eq s_185_0 s_185_1
        let s_185_2: bool = ((s_185_0) == (s_185_1));
        // N s_185_3: branch s_185_2 b467 b186
        if s_185_2 {
            return block_467(state, tracer, fn_state);
        } else {
            return block_186(state, tracer, fn_state);
        };
    }
    fn block_186<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_186_0: read-var x:str
        let s_186_0: &'static str = fn_state.x;
        // C s_186_1: const #"event 63 is not implemented, or filtering on event 63 is not supported" : str
        let s_186_1: &'static str = "event 63 is not implemented, or filtering on event 63 is not supported";
        // D s_186_2: cmp-eq s_186_0 s_186_1
        let s_186_2: bool = ((s_186_0) == (s_186_1));
        // N s_186_3: branch s_186_2 b466 b187
        if s_186_2 {
            return block_466(state, tracer, fn_state);
        } else {
            return block_187(state, tracer, fn_state);
        };
    }
    fn block_187<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_187_0: read-var x:str
        let s_187_0: &'static str = fn_state.x;
        // C s_187_1: const #"event 62 is not implemented, or filtering on event 62 is not supported" : str
        let s_187_1: &'static str = "event 62 is not implemented, or filtering on event 62 is not supported";
        // D s_187_2: cmp-eq s_187_0 s_187_1
        let s_187_2: bool = ((s_187_0) == (s_187_1));
        // N s_187_3: branch s_187_2 b465 b188
        if s_187_2 {
            return block_465(state, tracer, fn_state);
        } else {
            return block_188(state, tracer, fn_state);
        };
    }
    fn block_188<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_188_0: read-var x:str
        let s_188_0: &'static str = fn_state.x;
        // C s_188_1: const #"event 61 is not implemented, or filtering on event 61 is not supported" : str
        let s_188_1: &'static str = "event 61 is not implemented, or filtering on event 61 is not supported";
        // D s_188_2: cmp-eq s_188_0 s_188_1
        let s_188_2: bool = ((s_188_0) == (s_188_1));
        // N s_188_3: branch s_188_2 b464 b189
        if s_188_2 {
            return block_464(state, tracer, fn_state);
        } else {
            return block_189(state, tracer, fn_state);
        };
    }
    fn block_189<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_189_0: read-var x:str
        let s_189_0: &'static str = fn_state.x;
        // C s_189_1: const #"event 60 is not implemented, or filtering on event 60 is not supported" : str
        let s_189_1: &'static str = "event 60 is not implemented, or filtering on event 60 is not supported";
        // D s_189_2: cmp-eq s_189_0 s_189_1
        let s_189_2: bool = ((s_189_0) == (s_189_1));
        // N s_189_3: branch s_189_2 b463 b190
        if s_189_2 {
            return block_463(state, tracer, fn_state);
        } else {
            return block_190(state, tracer, fn_state);
        };
    }
    fn block_190<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_190_0: read-var x:str
        let s_190_0: &'static str = fn_state.x;
        // C s_190_1: const #"event 59 is not implemented, or filtering on event 59 is not supported" : str
        let s_190_1: &'static str = "event 59 is not implemented, or filtering on event 59 is not supported";
        // D s_190_2: cmp-eq s_190_0 s_190_1
        let s_190_2: bool = ((s_190_0) == (s_190_1));
        // N s_190_3: branch s_190_2 b462 b191
        if s_190_2 {
            return block_462(state, tracer, fn_state);
        } else {
            return block_191(state, tracer, fn_state);
        };
    }
    fn block_191<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_191_0: read-var x:str
        let s_191_0: &'static str = fn_state.x;
        // C s_191_1: const #"event 58 is not implemented, or filtering on event 58 is not supported" : str
        let s_191_1: &'static str = "event 58 is not implemented, or filtering on event 58 is not supported";
        // D s_191_2: cmp-eq s_191_0 s_191_1
        let s_191_2: bool = ((s_191_0) == (s_191_1));
        // N s_191_3: branch s_191_2 b461 b192
        if s_191_2 {
            return block_461(state, tracer, fn_state);
        } else {
            return block_192(state, tracer, fn_state);
        };
    }
    fn block_192<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_192_0: read-var x:str
        let s_192_0: &'static str = fn_state.x;
        // C s_192_1: const #"event 57 is not implemented, or filtering on event 57 is not supported" : str
        let s_192_1: &'static str = "event 57 is not implemented, or filtering on event 57 is not supported";
        // D s_192_2: cmp-eq s_192_0 s_192_1
        let s_192_2: bool = ((s_192_0) == (s_192_1));
        // N s_192_3: branch s_192_2 b460 b193
        if s_192_2 {
            return block_460(state, tracer, fn_state);
        } else {
            return block_193(state, tracer, fn_state);
        };
    }
    fn block_193<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_193_0: read-var x:str
        let s_193_0: &'static str = fn_state.x;
        // C s_193_1: const #"event 56 is not implemented, or filtering on event 56 is not supported" : str
        let s_193_1: &'static str = "event 56 is not implemented, or filtering on event 56 is not supported";
        // D s_193_2: cmp-eq s_193_0 s_193_1
        let s_193_2: bool = ((s_193_0) == (s_193_1));
        // N s_193_3: branch s_193_2 b459 b194
        if s_193_2 {
            return block_459(state, tracer, fn_state);
        } else {
            return block_194(state, tracer, fn_state);
        };
    }
    fn block_194<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_194_0: read-var x:str
        let s_194_0: &'static str = fn_state.x;
        // C s_194_1: const #"event 55 is not implemented, or filtering on event 55 is not supported" : str
        let s_194_1: &'static str = "event 55 is not implemented, or filtering on event 55 is not supported";
        // D s_194_2: cmp-eq s_194_0 s_194_1
        let s_194_2: bool = ((s_194_0) == (s_194_1));
        // N s_194_3: branch s_194_2 b458 b195
        if s_194_2 {
            return block_458(state, tracer, fn_state);
        } else {
            return block_195(state, tracer, fn_state);
        };
    }
    fn block_195<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_195_0: read-var x:str
        let s_195_0: &'static str = fn_state.x;
        // C s_195_1: const #"event 54 is not implemented, or filtering on event 54 is not supported" : str
        let s_195_1: &'static str = "event 54 is not implemented, or filtering on event 54 is not supported";
        // D s_195_2: cmp-eq s_195_0 s_195_1
        let s_195_2: bool = ((s_195_0) == (s_195_1));
        // N s_195_3: branch s_195_2 b457 b196
        if s_195_2 {
            return block_457(state, tracer, fn_state);
        } else {
            return block_196(state, tracer, fn_state);
        };
    }
    fn block_196<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_196_0: read-var x:str
        let s_196_0: &'static str = fn_state.x;
        // C s_196_1: const #"event 53 is not implemented, or filtering on event 53 is not supported" : str
        let s_196_1: &'static str = "event 53 is not implemented, or filtering on event 53 is not supported";
        // D s_196_2: cmp-eq s_196_0 s_196_1
        let s_196_2: bool = ((s_196_0) == (s_196_1));
        // N s_196_3: branch s_196_2 b456 b197
        if s_196_2 {
            return block_456(state, tracer, fn_state);
        } else {
            return block_197(state, tracer, fn_state);
        };
    }
    fn block_197<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_197_0: read-var x:str
        let s_197_0: &'static str = fn_state.x;
        // C s_197_1: const #"event 52 is not implemented, or filtering on event 52 is not supported" : str
        let s_197_1: &'static str = "event 52 is not implemented, or filtering on event 52 is not supported";
        // D s_197_2: cmp-eq s_197_0 s_197_1
        let s_197_2: bool = ((s_197_0) == (s_197_1));
        // N s_197_3: branch s_197_2 b455 b198
        if s_197_2 {
            return block_455(state, tracer, fn_state);
        } else {
            return block_198(state, tracer, fn_state);
        };
    }
    fn block_198<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_198_0: read-var x:str
        let s_198_0: &'static str = fn_state.x;
        // C s_198_1: const #"event 51 is not implemented, or filtering on event 51 is not supported" : str
        let s_198_1: &'static str = "event 51 is not implemented, or filtering on event 51 is not supported";
        // D s_198_2: cmp-eq s_198_0 s_198_1
        let s_198_2: bool = ((s_198_0) == (s_198_1));
        // N s_198_3: branch s_198_2 b454 b199
        if s_198_2 {
            return block_454(state, tracer, fn_state);
        } else {
            return block_199(state, tracer, fn_state);
        };
    }
    fn block_199<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_199_0: read-var x:str
        let s_199_0: &'static str = fn_state.x;
        // C s_199_1: const #"event 50 is not implemented, or filtering on event 50 is not supported" : str
        let s_199_1: &'static str = "event 50 is not implemented, or filtering on event 50 is not supported";
        // D s_199_2: cmp-eq s_199_0 s_199_1
        let s_199_2: bool = ((s_199_0) == (s_199_1));
        // N s_199_3: branch s_199_2 b453 b200
        if s_199_2 {
            return block_453(state, tracer, fn_state);
        } else {
            return block_200(state, tracer, fn_state);
        };
    }
    fn block_200<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_200_0: read-var x:str
        let s_200_0: &'static str = fn_state.x;
        // C s_200_1: const #"event 49 is not implemented, or filtering on event 49 is not supported" : str
        let s_200_1: &'static str = "event 49 is not implemented, or filtering on event 49 is not supported";
        // D s_200_2: cmp-eq s_200_0 s_200_1
        let s_200_2: bool = ((s_200_0) == (s_200_1));
        // N s_200_3: branch s_200_2 b452 b201
        if s_200_2 {
            return block_452(state, tracer, fn_state);
        } else {
            return block_201(state, tracer, fn_state);
        };
    }
    fn block_201<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_201_0: read-var x:str
        let s_201_0: &'static str = fn_state.x;
        // C s_201_1: const #"event 48 is not implemented, or filtering on event 48 is not supported" : str
        let s_201_1: &'static str = "event 48 is not implemented, or filtering on event 48 is not supported";
        // D s_201_2: cmp-eq s_201_0 s_201_1
        let s_201_2: bool = ((s_201_0) == (s_201_1));
        // N s_201_3: branch s_201_2 b451 b202
        if s_201_2 {
            return block_451(state, tracer, fn_state);
        } else {
            return block_202(state, tracer, fn_state);
        };
    }
    fn block_202<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_202_0: read-var x:str
        let s_202_0: &'static str = fn_state.x;
        // C s_202_1: const #"event 31 is not implemented, or filtering on event 31 is not supported" : str
        let s_202_1: &'static str = "event 31 is not implemented, or filtering on event 31 is not supported";
        // D s_202_2: cmp-eq s_202_0 s_202_1
        let s_202_2: bool = ((s_202_0) == (s_202_1));
        // N s_202_3: branch s_202_2 b450 b203
        if s_202_2 {
            return block_450(state, tracer, fn_state);
        } else {
            return block_203(state, tracer, fn_state);
        };
    }
    fn block_203<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_203_0: read-var x:str
        let s_203_0: &'static str = fn_state.x;
        // C s_203_1: const #"event 30 is not implemented, or filtering on event 30 is not supported" : str
        let s_203_1: &'static str = "event 30 is not implemented, or filtering on event 30 is not supported";
        // D s_203_2: cmp-eq s_203_0 s_203_1
        let s_203_2: bool = ((s_203_0) == (s_203_1));
        // N s_203_3: branch s_203_2 b449 b204
        if s_203_2 {
            return block_449(state, tracer, fn_state);
        } else {
            return block_204(state, tracer, fn_state);
        };
    }
    fn block_204<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_204_0: read-var x:str
        let s_204_0: &'static str = fn_state.x;
        // C s_204_1: const #"event 29 is not implemented, or filtering on event 29 is not supported" : str
        let s_204_1: &'static str = "event 29 is not implemented, or filtering on event 29 is not supported";
        // D s_204_2: cmp-eq s_204_0 s_204_1
        let s_204_2: bool = ((s_204_0) == (s_204_1));
        // N s_204_3: branch s_204_2 b448 b205
        if s_204_2 {
            return block_448(state, tracer, fn_state);
        } else {
            return block_205(state, tracer, fn_state);
        };
    }
    fn block_205<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_205_0: read-var x:str
        let s_205_0: &'static str = fn_state.x;
        // C s_205_1: const #"event 28 is not implemented, or filtering on event 28 is not supported" : str
        let s_205_1: &'static str = "event 28 is not implemented, or filtering on event 28 is not supported";
        // D s_205_2: cmp-eq s_205_0 s_205_1
        let s_205_2: bool = ((s_205_0) == (s_205_1));
        // N s_205_3: branch s_205_2 b447 b206
        if s_205_2 {
            return block_447(state, tracer, fn_state);
        } else {
            return block_206(state, tracer, fn_state);
        };
    }
    fn block_206<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_206_0: read-var x:str
        let s_206_0: &'static str = fn_state.x;
        // C s_206_1: const #"event 27 is not implemented, or filtering on event 27 is not supported" : str
        let s_206_1: &'static str = "event 27 is not implemented, or filtering on event 27 is not supported";
        // D s_206_2: cmp-eq s_206_0 s_206_1
        let s_206_2: bool = ((s_206_0) == (s_206_1));
        // N s_206_3: branch s_206_2 b446 b207
        if s_206_2 {
            return block_446(state, tracer, fn_state);
        } else {
            return block_207(state, tracer, fn_state);
        };
    }
    fn block_207<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_207_0: read-var x:str
        let s_207_0: &'static str = fn_state.x;
        // C s_207_1: const #"event 26 is not implemented, or filtering on event 26 is not supported" : str
        let s_207_1: &'static str = "event 26 is not implemented, or filtering on event 26 is not supported";
        // D s_207_2: cmp-eq s_207_0 s_207_1
        let s_207_2: bool = ((s_207_0) == (s_207_1));
        // N s_207_3: branch s_207_2 b445 b208
        if s_207_2 {
            return block_445(state, tracer, fn_state);
        } else {
            return block_208(state, tracer, fn_state);
        };
    }
    fn block_208<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_208_0: read-var x:str
        let s_208_0: &'static str = fn_state.x;
        // C s_208_1: const #"event 25 is not implemented, or filtering on event 25 is not supported" : str
        let s_208_1: &'static str = "event 25 is not implemented, or filtering on event 25 is not supported";
        // D s_208_2: cmp-eq s_208_0 s_208_1
        let s_208_2: bool = ((s_208_0) == (s_208_1));
        // N s_208_3: branch s_208_2 b444 b209
        if s_208_2 {
            return block_444(state, tracer, fn_state);
        } else {
            return block_209(state, tracer, fn_state);
        };
    }
    fn block_209<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_209_0: read-var x:str
        let s_209_0: &'static str = fn_state.x;
        // C s_209_1: const #"event 24 is not implemented, or filtering on event 24 is not supported" : str
        let s_209_1: &'static str = "event 24 is not implemented, or filtering on event 24 is not supported";
        // D s_209_2: cmp-eq s_209_0 s_209_1
        let s_209_2: bool = ((s_209_0) == (s_209_1));
        // N s_209_3: branch s_209_2 b443 b210
        if s_209_2 {
            return block_443(state, tracer, fn_state);
        } else {
            return block_210(state, tracer, fn_state);
        };
    }
    fn block_210<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_210_0: read-var x:str
        let s_210_0: &'static str = fn_state.x;
        // C s_210_1: const #"event 23 is not implemented, or filtering on event 23 is not supported" : str
        let s_210_1: &'static str = "event 23 is not implemented, or filtering on event 23 is not supported";
        // D s_210_2: cmp-eq s_210_0 s_210_1
        let s_210_2: bool = ((s_210_0) == (s_210_1));
        // N s_210_3: branch s_210_2 b442 b211
        if s_210_2 {
            return block_442(state, tracer, fn_state);
        } else {
            return block_211(state, tracer, fn_state);
        };
    }
    fn block_211<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_211_0: read-var x:str
        let s_211_0: &'static str = fn_state.x;
        // C s_211_1: const #"event 22 is not implemented, or filtering on event 22 is not supported" : str
        let s_211_1: &'static str = "event 22 is not implemented, or filtering on event 22 is not supported";
        // D s_211_2: cmp-eq s_211_0 s_211_1
        let s_211_2: bool = ((s_211_0) == (s_211_1));
        // N s_211_3: branch s_211_2 b441 b212
        if s_211_2 {
            return block_441(state, tracer, fn_state);
        } else {
            return block_212(state, tracer, fn_state);
        };
    }
    fn block_212<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_212_0: read-var x:str
        let s_212_0: &'static str = fn_state.x;
        // C s_212_1: const #"event 21 is not implemented, or filtering on event 21 is not supported" : str
        let s_212_1: &'static str = "event 21 is not implemented, or filtering on event 21 is not supported";
        // D s_212_2: cmp-eq s_212_0 s_212_1
        let s_212_2: bool = ((s_212_0) == (s_212_1));
        // N s_212_3: branch s_212_2 b440 b213
        if s_212_2 {
            return block_440(state, tracer, fn_state);
        } else {
            return block_213(state, tracer, fn_state);
        };
    }
    fn block_213<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_213_0: read-var x:str
        let s_213_0: &'static str = fn_state.x;
        // C s_213_1: const #"event 20 is not implemented, or filtering on event 20 is not supported" : str
        let s_213_1: &'static str = "event 20 is not implemented, or filtering on event 20 is not supported";
        // D s_213_2: cmp-eq s_213_0 s_213_1
        let s_213_2: bool = ((s_213_0) == (s_213_1));
        // N s_213_3: branch s_213_2 b439 b214
        if s_213_2 {
            return block_439(state, tracer, fn_state);
        } else {
            return block_214(state, tracer, fn_state);
        };
    }
    fn block_214<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_214_0: read-var x:str
        let s_214_0: &'static str = fn_state.x;
        // C s_214_1: const #"event 19 is not implemented, or filtering on event 19 is not supported" : str
        let s_214_1: &'static str = "event 19 is not implemented, or filtering on event 19 is not supported";
        // D s_214_2: cmp-eq s_214_0 s_214_1
        let s_214_2: bool = ((s_214_0) == (s_214_1));
        // N s_214_3: branch s_214_2 b438 b215
        if s_214_2 {
            return block_438(state, tracer, fn_state);
        } else {
            return block_215(state, tracer, fn_state);
        };
    }
    fn block_215<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_215_0: read-var x:str
        let s_215_0: &'static str = fn_state.x;
        // C s_215_1: const #"event 18 is not implemented, or filtering on event 18 is not supported" : str
        let s_215_1: &'static str = "event 18 is not implemented, or filtering on event 18 is not supported";
        // D s_215_2: cmp-eq s_215_0 s_215_1
        let s_215_2: bool = ((s_215_0) == (s_215_1));
        // N s_215_3: branch s_215_2 b437 b216
        if s_215_2 {
            return block_437(state, tracer, fn_state);
        } else {
            return block_216(state, tracer, fn_state);
        };
    }
    fn block_216<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_216_0: read-var x:str
        let s_216_0: &'static str = fn_state.x;
        // C s_216_1: const #"event 17 is not implemented, or filtering on event 17 is not supported" : str
        let s_216_1: &'static str = "event 17 is not implemented, or filtering on event 17 is not supported";
        // D s_216_2: cmp-eq s_216_0 s_216_1
        let s_216_2: bool = ((s_216_0) == (s_216_1));
        // N s_216_3: branch s_216_2 b436 b217
        if s_216_2 {
            return block_436(state, tracer, fn_state);
        } else {
            return block_217(state, tracer, fn_state);
        };
    }
    fn block_217<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_217_0: read-var x:str
        let s_217_0: &'static str = fn_state.x;
        // C s_217_1: const #"event 16 is not implemented, or filtering on event 16 is not supported" : str
        let s_217_1: &'static str = "event 16 is not implemented, or filtering on event 16 is not supported";
        // D s_217_2: cmp-eq s_217_0 s_217_1
        let s_217_2: bool = ((s_217_0) == (s_217_1));
        // N s_217_3: branch s_217_2 b435 b218
        if s_217_2 {
            return block_435(state, tracer, fn_state);
        } else {
            return block_218(state, tracer, fn_state);
        };
    }
    fn block_218<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_218_0: read-var x:str
        let s_218_0: &'static str = fn_state.x;
        // C s_218_1: const #"event 15 is not implemented, or filtering on event 15 is not supported" : str
        let s_218_1: &'static str = "event 15 is not implemented, or filtering on event 15 is not supported";
        // D s_218_2: cmp-eq s_218_0 s_218_1
        let s_218_2: bool = ((s_218_0) == (s_218_1));
        // N s_218_3: branch s_218_2 b434 b219
        if s_218_2 {
            return block_434(state, tracer, fn_state);
        } else {
            return block_219(state, tracer, fn_state);
        };
    }
    fn block_219<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_219_0: read-var x:str
        let s_219_0: &'static str = fn_state.x;
        // C s_219_1: const #"event 14 is not implemented, or filtering on event 14 is not supported" : str
        let s_219_1: &'static str = "event 14 is not implemented, or filtering on event 14 is not supported";
        // D s_219_2: cmp-eq s_219_0 s_219_1
        let s_219_2: bool = ((s_219_0) == (s_219_1));
        // N s_219_3: branch s_219_2 b433 b220
        if s_219_2 {
            return block_433(state, tracer, fn_state);
        } else {
            return block_220(state, tracer, fn_state);
        };
    }
    fn block_220<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_220_0: read-var x:str
        let s_220_0: &'static str = fn_state.x;
        // C s_220_1: const #"event 13 is not implemented, or filtering on event 13 is not supported" : str
        let s_220_1: &'static str = "event 13 is not implemented, or filtering on event 13 is not supported";
        // D s_220_2: cmp-eq s_220_0 s_220_1
        let s_220_2: bool = ((s_220_0) == (s_220_1));
        // N s_220_3: branch s_220_2 b432 b221
        if s_220_2 {
            return block_432(state, tracer, fn_state);
        } else {
            return block_221(state, tracer, fn_state);
        };
    }
    fn block_221<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_221_0: read-var x:str
        let s_221_0: &'static str = fn_state.x;
        // C s_221_1: const #"event 12 is not implemented, or filtering on event 12 is not supported" : str
        let s_221_1: &'static str = "event 12 is not implemented, or filtering on event 12 is not supported";
        // D s_221_2: cmp-eq s_221_0 s_221_1
        let s_221_2: bool = ((s_221_0) == (s_221_1));
        // N s_221_3: branch s_221_2 b431 b222
        if s_221_2 {
            return block_431(state, tracer, fn_state);
        } else {
            return block_222(state, tracer, fn_state);
        };
    }
    fn block_222<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_222_0: read-var x:str
        let s_222_0: &'static str = fn_state.x;
        // C s_222_1: const #"SPE get prev br if not br" : str
        let s_222_1: &'static str = "SPE get prev br if not br";
        // D s_222_2: cmp-eq s_222_0 s_222_1
        let s_222_2: bool = ((s_222_0) == (s_222_1));
        // N s_222_3: branch s_222_2 b430 b223
        if s_222_2 {
            return block_430(state, tracer, fn_state);
        } else {
            return block_223(state, tracer, fn_state);
        };
    }
    fn block_223<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_223_0: read-var x:str
        let s_223_0: &'static str = fn_state.x;
        // C s_223_1: const #"SPE prev br on eret" : str
        let s_223_1: &'static str = "SPE prev br on eret";
        // D s_223_2: cmp-eq s_223_0 s_223_1
        let s_223_2: bool = ((s_223_0) == (s_223_1));
        // N s_223_3: branch s_223_2 b429 b224
        if s_223_2 {
            return block_429(state, tracer, fn_state);
        } else {
            return block_224(state, tracer, fn_state);
        };
    }
    fn block_224<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_224_0: read-var x:str
        let s_224_0: &'static str = fn_state.x;
        // C s_224_1: const #"SPE prev br on isb" : str
        let s_224_1: &'static str = "SPE prev br on isb";
        // D s_224_2: cmp-eq s_224_0 s_224_1
        let s_224_2: bool = ((s_224_0) == (s_224_1));
        // N s_224_3: branch s_224_2 b428 b225
        if s_224_2 {
            return block_428(state, tracer, fn_state);
        } else {
            return block_225(state, tracer, fn_state);
        };
    }
    fn block_225<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_225_0: read-var x:str
        let s_225_0: &'static str = fn_state.x;
        // C s_225_1: const #"SPE prev br on exception" : str
        let s_225_1: &'static str = "SPE prev br on exception";
        // D s_225_2: cmp-eq s_225_0 s_225_1
        let s_225_2: bool = ((s_225_0) == (s_225_1));
        // N s_225_3: branch s_225_2 b427 b226
        if s_225_2 {
            return block_427(state, tracer, fn_state);
        } else {
            return block_226(state, tracer, fn_state);
        };
    }
    fn block_226<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_226_0: read-var x:str
        let s_226_0: &'static str = fn_state.x;
        // C s_226_1: const #"SPE 16bit counters" : str
        let s_226_1: &'static str = "SPE 16bit counters";
        // D s_226_2: cmp-eq s_226_0 s_226_1
        let s_226_2: bool = ((s_226_0) == (s_226_1));
        // N s_226_3: branch s_226_2 b426 b227
        if s_226_2 {
            return block_426(state, tracer, fn_state);
        } else {
            return block_227(state, tracer, fn_state);
        };
    }
    fn block_227<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_227_0: read-var x:str
        let s_227_0: &'static str = fn_state.x;
        // C s_227_1: const #"filtering on event 2 is optionally supported" : str
        let s_227_1: &'static str = "filtering on event 2 is optionally supported";
        // D s_227_2: cmp-eq s_227_0 s_227_1
        let s_227_2: bool = ((s_227_0) == (s_227_1));
        // N s_227_3: branch s_227_2 b425 b228
        if s_227_2 {
            return block_425(state, tracer, fn_state);
        } else {
            return block_228(state, tracer, fn_state);
        };
    }
    fn block_228<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_228_0: read-var x:str
        let s_228_0: &'static str = fn_state.x;
        // C s_228_1: const #"filtering on event 4 is optionally supported" : str
        let s_228_1: &'static str = "filtering on event 4 is optionally supported";
        // D s_228_2: cmp-eq s_228_0 s_228_1
        let s_228_2: bool = ((s_228_0) == (s_228_1));
        // N s_228_3: branch s_228_2 b424 b229
        if s_228_2 {
            return block_424(state, tracer, fn_state);
        } else {
            return block_229(state, tracer, fn_state);
        };
    }
    fn block_229<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_229_0: read-var x:str
        let s_229_0: &'static str = fn_state.x;
        // C s_229_1: const #"filtering on event 8 is optionally supported" : str
        let s_229_1: &'static str = "filtering on event 8 is optionally supported";
        // D s_229_2: cmp-eq s_229_0 s_229_1
        let s_229_2: bool = ((s_229_0) == (s_229_1));
        // N s_229_3: branch s_229_2 b423 b230
        if s_229_2 {
            return block_423(state, tracer, fn_state);
        } else {
            return block_230(state, tracer, fn_state);
        };
    }
    fn block_230<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_230_0: read-var x:str
        let s_230_0: &'static str = fn_state.x;
        // C s_230_1: const #"filtering on event 9 is optionally supported" : str
        let s_230_1: &'static str = "filtering on event 9 is optionally supported";
        // D s_230_2: cmp-eq s_230_0 s_230_1
        let s_230_2: bool = ((s_230_0) == (s_230_1));
        // N s_230_3: branch s_230_2 b422 b231
        if s_230_2 {
            return block_422(state, tracer, fn_state);
        } else {
            return block_231(state, tracer, fn_state);
        };
    }
    fn block_231<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_231_0: read-var x:str
        let s_231_0: &'static str = fn_state.x;
        // C s_231_1: const #"filtering on event 10 is optionally supported" : str
        let s_231_1: &'static str = "filtering on event 10 is optionally supported";
        // D s_231_2: cmp-eq s_231_0 s_231_1
        let s_231_2: bool = ((s_231_0) == (s_231_1));
        // N s_231_3: branch s_231_2 b421 b232
        if s_231_2 {
            return block_421(state, tracer, fn_state);
        } else {
            return block_232(state, tracer, fn_state);
        };
    }
    fn block_232<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_232_0: read-var x:str
        let s_232_0: &'static str = fn_state.x;
        // C s_232_1: const #"event 8 is implemented" : str
        let s_232_1: &'static str = "event 8 is implemented";
        // D s_232_2: cmp-eq s_232_0 s_232_1
        let s_232_2: bool = ((s_232_0) == (s_232_1));
        // N s_232_3: branch s_232_2 b420 b233
        if s_232_2 {
            return block_420(state, tracer, fn_state);
        } else {
            return block_233(state, tracer, fn_state);
        };
    }
    fn block_233<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_233_0: read-var x:str
        let s_233_0: &'static str = fn_state.x;
        // C s_233_1: const #"event 9 is implemented" : str
        let s_233_1: &'static str = "event 9 is implemented";
        // D s_233_2: cmp-eq s_233_0 s_233_1
        let s_233_2: bool = ((s_233_0) == (s_233_1));
        // N s_233_3: branch s_233_2 b419 b234
        if s_233_2 {
            return block_419(state, tracer, fn_state);
        } else {
            return block_234(state, tracer, fn_state);
        };
    }
    fn block_234<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_234_0: read-var x:str
        let s_234_0: &'static str = fn_state.x;
        // C s_234_1: const #"event 10 is implemented" : str
        let s_234_1: &'static str = "event 10 is implemented";
        // D s_234_2: cmp-eq s_234_0 s_234_1
        let s_234_2: bool = ((s_234_0) == (s_234_1));
        // N s_234_3: branch s_234_2 b418 b235
        if s_234_2 {
            return block_418(state, tracer, fn_state);
        } else {
            return block_235(state, tracer, fn_state);
        };
    }
    fn block_235<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_235_0: read-var x:str
        let s_235_0: &'static str = fn_state.x;
        // C s_235_1: const #"event 19 is implemented" : str
        let s_235_1: &'static str = "event 19 is implemented";
        // D s_235_2: cmp-eq s_235_0 s_235_1
        let s_235_2: bool = ((s_235_0) == (s_235_1));
        // N s_235_3: branch s_235_2 b417 b236
        if s_235_2 {
            return block_417(state, tracer, fn_state);
        } else {
            return block_236(state, tracer, fn_state);
        };
    }
    fn block_236<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_236_0: read-var x:str
        let s_236_0: &'static str = fn_state.x;
        // C s_236_1: const #"event 20 is implemented" : str
        let s_236_1: &'static str = "event 20 is implemented";
        // D s_236_2: cmp-eq s_236_0 s_236_1
        let s_236_2: bool = ((s_236_0) == (s_236_1));
        // N s_236_3: branch s_236_2 b416 b237
        if s_236_2 {
            return block_416(state, tracer, fn_state);
        } else {
            return block_237(state, tracer, fn_state);
        };
    }
    fn block_237<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_237_0: read-var x:str
        let s_237_0: &'static str = fn_state.x;
        // C s_237_1: const #"event 21 is implemented" : str
        let s_237_1: &'static str = "event 21 is implemented";
        // D s_237_2: cmp-eq s_237_0 s_237_1
        let s_237_2: bool = ((s_237_0) == (s_237_1));
        // N s_237_3: branch s_237_2 b415 b238
        if s_237_2 {
            return block_415(state, tracer, fn_state);
        } else {
            return block_238(state, tracer, fn_state);
        };
    }
    fn block_238<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_238_0: read-var x:str
        let s_238_0: &'static str = fn_state.x;
        // C s_238_1: const #"event 22 is implemented" : str
        let s_238_1: &'static str = "event 22 is implemented";
        // D s_238_2: cmp-eq s_238_0 s_238_1
        let s_238_2: bool = ((s_238_0) == (s_238_1));
        // N s_238_3: branch s_238_2 b414 b239
        if s_238_2 {
            return block_414(state, tracer, fn_state);
        } else {
            return block_239(state, tracer, fn_state);
        };
    }
    fn block_239<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_239_0: read-var x:str
        let s_239_0: &'static str = fn_state.x;
        // C s_239_1: const #"event 23 is implemented" : str
        let s_239_1: &'static str = "event 23 is implemented";
        // D s_239_2: cmp-eq s_239_0 s_239_1
        let s_239_2: bool = ((s_239_0) == (s_239_1));
        // N s_239_3: branch s_239_2 b413 b240
        if s_239_2 {
            return block_413(state, tracer, fn_state);
        } else {
            return block_240(state, tracer, fn_state);
        };
    }
    fn block_240<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_240_0: read-var x:str
        let s_240_0: &'static str = fn_state.x;
        // C s_240_1: const #"Has SPEv1p4 extension" : str
        let s_240_1: &'static str = "Has SPEv1p4 extension";
        // D s_240_2: cmp-eq s_240_0 s_240_1
        let s_240_2: bool = ((s_240_0) == (s_240_1));
        // N s_240_3: branch s_240_2 b412 b241
        if s_240_2 {
            return block_412(state, tracer, fn_state);
        } else {
            return block_241(state, tracer, fn_state);
        };
    }
    fn block_241<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_241_0: read-var x:str
        let s_241_0: &'static str = fn_state.x;
        // C s_241_1: const #"Has SPE_FDS extension" : str
        let s_241_1: &'static str = "Has SPE_FDS extension";
        // D s_241_2: cmp-eq s_241_0 s_241_1
        let s_241_2: bool = ((s_241_0) == (s_241_1));
        // N s_241_3: branch s_241_2 b411 b242
        if s_241_2 {
            return block_411(state, tracer, fn_state);
        } else {
            return block_242(state, tracer, fn_state);
        };
    }
    fn block_242<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_242_0: read-var x:str
        let s_242_0: &'static str = fn_state.x;
        // C s_242_1: const #"Has RME extension" : str
        let s_242_1: &'static str = "Has RME extension";
        // D s_242_2: cmp-eq s_242_0 s_242_1
        let s_242_2: bool = ((s_242_0) == (s_242_1));
        // N s_242_3: branch s_242_2 b410 b243
        if s_242_2 {
            return block_410(state, tracer, fn_state);
        } else {
            return block_243(state, tracer, fn_state);
        };
    }
    fn block_243<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_243_0: read-var x:str
        let s_243_0: &'static str = fn_state.x;
        // C s_243_1: const #"GPC Fault on DC operations" : str
        let s_243_1: &'static str = "GPC Fault on DC operations";
        // D s_243_2: cmp-eq s_243_0 s_243_1
        let s_243_2: bool = ((s_243_0) == (s_243_1));
        // N s_243_3: branch s_243_2 b409 b244
        if s_243_2 {
            return block_409(state, tracer, fn_state);
        } else {
            return block_244(state, tracer, fn_state);
        };
    }
    fn block_244<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_244_0: read-var x:str
        let s_244_0: &'static str = fn_state.x;
        // C s_244_1: const #"Realm EL2&0 regime affects EPAN" : str
        let s_244_1: &'static str = "Realm EL2&0 regime affects EPAN";
        // D s_244_2: cmp-eq s_244_0 s_244_1
        let s_244_2: bool = ((s_244_0) == (s_244_1));
        // N s_244_3: branch s_244_2 b408 b245
        if s_244_2 {
            return block_408(state, tracer, fn_state);
        } else {
            return block_245(state, tracer, fn_state);
        };
    }
    fn block_245<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_245_0: read-var x:str
        let s_245_0: &'static str = fn_state.x;
        // C s_245_1: const #"Has feature WFxT" : str
        let s_245_1: &'static str = "Has feature WFxT";
        // D s_245_2: cmp-eq s_245_0 s_245_1
        let s_245_2: bool = ((s_245_0) == (s_245_1));
        // N s_245_3: branch s_245_2 b407 b246
        if s_245_2 {
            return block_407(state, tracer, fn_state);
        } else {
            return block_246(state, tracer, fn_state);
        };
    }
    fn block_246<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_246_0: read-var x:str
        let s_246_0: &'static str = fn_state.x;
        // C s_246_1: const #"Has large 52-bit PA/IPA support" : str
        let s_246_1: &'static str = "Has large 52-bit PA/IPA support";
        // D s_246_2: cmp-eq s_246_0 s_246_1
        let s_246_2: bool = ((s_246_0) == (s_246_1));
        // N s_246_3: branch s_246_2 b406 b247
        if s_246_2 {
            return block_406(state, tracer, fn_state);
        } else {
            return block_247(state, tracer, fn_state);
        };
    }
    fn block_247<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_247_0: read-var x:str
        let s_247_0: &'static str = fn_state.x;
        // C s_247_1: const #"Has Enhanced Counter Virtualization extension" : str
        let s_247_1: &'static str = "Has Enhanced Counter Virtualization extension";
        // D s_247_2: cmp-eq s_247_0 s_247_1
        let s_247_2: bool = ((s_247_0) == (s_247_1));
        // N s_247_3: branch s_247_2 b405 b248
        if s_247_2 {
            return block_405(state, tracer, fn_state);
        } else {
            return block_248(state, tracer, fn_state);
        };
    }
    fn block_248<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_248_0: read-var x:str
        let s_248_0: &'static str = fn_state.x;
        // C s_248_1: const #"Has large 52-bit VA support" : str
        let s_248_1: &'static str = "Has large 52-bit VA support";
        // D s_248_2: cmp-eq s_248_0 s_248_1
        let s_248_2: bool = ((s_248_0) == (s_248_1));
        // N s_248_3: branch s_248_2 b404 b249
        if s_248_2 {
            return block_404(state, tracer, fn_state);
        } else {
            return block_249(state, tracer, fn_state);
        };
    }
    fn block_249<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_249_0: read-var x:str
        let s_249_0: &'static str = fn_state.x;
        // C s_249_1: const #"the PE supports sampling of speculative instructions" : str
        let s_249_1: &'static str = "the PE supports sampling of speculative instructions";
        // D s_249_2: cmp-eq s_249_0 s_249_1
        let s_249_2: bool = ((s_249_0) == (s_249_1));
        // N s_249_3: branch s_249_2 b403 b250
        if s_249_2 {
            return block_403(state, tracer, fn_state);
        } else {
            return block_250(state, tracer, fn_state);
        };
    }
    fn block_250<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_250_0: read-var x:str
        let s_250_0: &'static str = fn_state.x;
        // C s_250_1: const #"Debug has Software Lock" : str
        let s_250_1: &'static str = "Debug has Software Lock";
        // D s_250_2: cmp-eq s_250_0 s_250_1
        let s_250_2: bool = ((s_250_0) == (s_250_1));
        // N s_250_3: branch s_250_2 b402 b251
        if s_250_2 {
            return block_402(state, tracer, fn_state);
        } else {
            return block_251(state, tracer, fn_state);
        };
    }
    fn block_251<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_251_0: read-var x:str
        let s_251_0: &'static str = fn_state.x;
        // C s_251_1: const #"External abort signaled in-band synchronously" : str
        let s_251_1: &'static str = "External abort signaled in-band synchronously";
        // D s_251_2: cmp-eq s_251_0 s_251_1
        let s_251_2: bool = ((s_251_0) == (s_251_1));
        // N s_251_3: branch s_251_2 b401 b252
        if s_251_2 {
            return block_401(state, tracer, fn_state);
        } else {
            return block_252(state, tracer, fn_state);
        };
    }
    fn block_252<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_252_0: read-var x:str
        let s_252_0: &'static str = fn_state.x;
        // C s_252_1: const #"PMU has Software Lock" : str
        let s_252_1: &'static str = "PMU has Software Lock";
        // D s_252_2: cmp-eq s_252_0 s_252_1
        let s_252_2: bool = ((s_252_0) == (s_252_1));
        // N s_252_3: branch s_252_2 b400 b253
        if s_252_2 {
            return block_400(state, tracer, fn_state);
        } else {
            return block_253(state, tracer, fn_state);
        };
    }
    fn block_253<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_253_0: read-var x:str
        let s_253_0: &'static str = fn_state.x;
        // C s_253_1: const #"CTI has Software Lock" : str
        let s_253_1: &'static str = "CTI has Software Lock";
        // D s_253_2: cmp-eq s_253_0 s_253_1
        let s_253_2: bool = ((s_253_0) == (s_253_1));
        // N s_253_3: branch s_253_2 b399 b254
        if s_253_2 {
            return block_399(state, tracer, fn_state);
        } else {
            return block_254(state, tracer, fn_state);
        };
    }
    fn block_254<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_254_0: read-var x:str
        let s_254_0: &'static str = fn_state.x;
        // C s_254_1: const #"Has PAC QARMA3 functionality" : str
        let s_254_1: &'static str = "Has PAC QARMA3 functionality";
        // D s_254_2: cmp-eq s_254_0 s_254_1
        let s_254_2: bool = ((s_254_0) == (s_254_1));
        // N s_254_3: branch s_254_2 b398 b255
        if s_254_2 {
            return block_398(state, tracer, fn_state);
        } else {
            return block_255(state, tracer, fn_state);
        };
    }
    fn block_255<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_255_0: read-var x:str
        let s_255_0: &'static str = fn_state.x;
        // C s_255_1: const #"Has PAC QARMA5 functionality" : str
        let s_255_1: &'static str = "Has PAC QARMA5 functionality";
        // D s_255_2: cmp-eq s_255_0 s_255_1
        let s_255_2: bool = ((s_255_0) == (s_255_1));
        // N s_255_3: branch s_255_2 b397 b256
        if s_255_2 {
            return block_397(state, tracer, fn_state);
        } else {
            return block_256(state, tracer, fn_state);
        };
    }
    fn block_256<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_256_0: read-var x:str
        let s_256_0: &'static str = fn_state.x;
        // C s_256_1: const #"Has PAC IMP functionality" : str
        let s_256_1: &'static str = "Has PAC IMP functionality";
        // D s_256_2: cmp-eq s_256_0 s_256_1
        let s_256_2: bool = ((s_256_0) == (s_256_1));
        // N s_256_3: branch s_256_2 b393 b257
        if s_256_2 {
            return block_393(state, tracer, fn_state);
        } else {
            return block_257(state, tracer, fn_state);
        };
    }
    fn block_257<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_257_0: read-var x:str
        let s_257_0: &'static str = fn_state.x;
        // C s_257_1: const #"Has PAC QARMA3 Auth functionality" : str
        let s_257_1: &'static str = "Has PAC QARMA3 Auth functionality";
        // D s_257_2: cmp-eq s_257_0 s_257_1
        let s_257_2: bool = ((s_257_0) == (s_257_1));
        // N s_257_3: branch s_257_2 b392 b258
        if s_257_2 {
            return block_392(state, tracer, fn_state);
        } else {
            return block_258(state, tracer, fn_state);
        };
    }
    fn block_258<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_258_0: read-var x:str
        let s_258_0: &'static str = fn_state.x;
        // C s_258_1: const #"Has PAC QARMA3 Generic functionality" : str
        let s_258_1: &'static str = "Has PAC QARMA3 Generic functionality";
        // D s_258_2: cmp-eq s_258_0 s_258_1
        let s_258_2: bool = ((s_258_0) == (s_258_1));
        // N s_258_3: branch s_258_2 b391 b259
        if s_258_2 {
            return block_391(state, tracer, fn_state);
        } else {
            return block_259(state, tracer, fn_state);
        };
    }
    fn block_259<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_259_0: read-var x:str
        let s_259_0: &'static str = fn_state.x;
        // C s_259_1: const #"Has PAC QARMA5 Auth functionality" : str
        let s_259_1: &'static str = "Has PAC QARMA5 Auth functionality";
        // D s_259_2: cmp-eq s_259_0 s_259_1
        let s_259_2: bool = ((s_259_0) == (s_259_1));
        // N s_259_3: branch s_259_2 b390 b260
        if s_259_2 {
            return block_390(state, tracer, fn_state);
        } else {
            return block_260(state, tracer, fn_state);
        };
    }
    fn block_260<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_260_0: read-var x:str
        let s_260_0: &'static str = fn_state.x;
        // C s_260_1: const #"Has PAC QARMA5 Generic functionality" : str
        let s_260_1: &'static str = "Has PAC QARMA5 Generic functionality";
        // D s_260_2: cmp-eq s_260_0 s_260_1
        let s_260_2: bool = ((s_260_0) == (s_260_1));
        // N s_260_3: branch s_260_2 b389 b261
        if s_260_2 {
            return block_389(state, tracer, fn_state);
        } else {
            return block_261(state, tracer, fn_state);
        };
    }
    fn block_261<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_261_0: read-var x:str
        let s_261_0: &'static str = fn_state.x;
        // C s_261_1: const #"Has PAC IMPDEF Auth functionality" : str
        let s_261_1: &'static str = "Has PAC IMPDEF Auth functionality";
        // D s_261_2: cmp-eq s_261_0 s_261_1
        let s_261_2: bool = ((s_261_0) == (s_261_1));
        // N s_261_3: branch s_261_2 b388 b262
        if s_261_2 {
            return block_388(state, tracer, fn_state);
        } else {
            return block_262(state, tracer, fn_state);
        };
    }
    fn block_262<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_262_0: read-var x:str
        let s_262_0: &'static str = fn_state.x;
        // C s_262_1: const #"Has PAC IMPDEF Generic functionality" : str
        let s_262_1: &'static str = "Has PAC IMPDEF Generic functionality";
        // D s_262_2: cmp-eq s_262_0 s_262_1
        let s_262_2: bool = ((s_262_0) == (s_262_1));
        // N s_262_3: branch s_262_2 b387 b263
        if s_262_2 {
            return block_387(state, tracer, fn_state);
        } else {
            return block_263(state, tracer, fn_state);
        };
    }
    fn block_263<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_263_0: read-var x:str
        let s_263_0: &'static str = fn_state.x;
        // C s_263_1: const #"Bit 55 determines the size of the PAC field" : str
        let s_263_1: &'static str = "Bit 55 determines the size of the PAC field";
        // D s_263_2: cmp-eq s_263_0 s_263_1
        let s_263_2: bool = ((s_263_0) == (s_263_1));
        // N s_263_3: branch s_263_2 b386 b264
        if s_263_2 {
            return block_386(state, tracer, fn_state);
        } else {
            return block_264(state, tracer, fn_state);
        };
    }
    fn block_264<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_264_0: read-var x:str
        let s_264_0: &'static str = fn_state.x;
        // C s_264_1: const #"FEAT_LSE2: access is atomic" : str
        let s_264_1: &'static str = "FEAT_LSE2: access is atomic";
        // D s_264_2: cmp-eq s_264_0 s_264_1
        let s_264_2: bool = ((s_264_0) == (s_264_1));
        // N s_264_3: branch s_264_2 b385 b265
        if s_264_2 {
            return block_385(state, tracer, fn_state);
        } else {
            return block_265(state, tracer, fn_state);
        };
    }
    fn block_265<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_265_0: read-var x:str
        let s_265_0: &'static str = fn_state.x;
        // C s_265_1: const #"Has Generic Counter Scaling support" : str
        let s_265_1: &'static str = "Has Generic Counter Scaling support";
        // D s_265_2: cmp-eq s_265_0 s_265_1
        let s_265_2: bool = ((s_265_0) == (s_265_1));
        // N s_265_3: branch s_265_2 b384 b266
        if s_265_2 {
            return block_384(state, tracer, fn_state);
        } else {
            return block_266(state, tracer, fn_state);
        };
    }
    fn block_266<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_266_0: read-var x:str
        let s_266_0: &'static str = fn_state.x;
        // C s_266_1: const #"No tag checking of SIMD&FP loads and stores in Streaming SVE mode" : str
        let s_266_1: &'static str = "No tag checking of SIMD&FP loads and stores in Streaming SVE mode";
        // D s_266_2: cmp-eq s_266_0 s_266_1
        let s_266_2: bool = ((s_266_0) == (s_266_1));
        // N s_266_3: branch s_266_2 b383 b267
        if s_266_2 {
            return block_383(state, tracer, fn_state);
        } else {
            return block_267(state, tracer, fn_state);
        };
    }
    fn block_267<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_267_0: read-var x:str
        let s_267_0: &'static str = fn_state.x;
        // C s_267_1: const #"No tag checking of ZA loads and stores" : str
        let s_267_1: &'static str = "No tag checking of ZA loads and stores";
        // D s_267_2: cmp-eq s_267_0 s_267_1
        let s_267_2: bool = ((s_267_0) == (s_267_1));
        // N s_267_3: branch s_267_2 b382 b268
        if s_267_2 {
            return block_382(state, tracer, fn_state);
        } else {
            return block_268(state, tracer, fn_state);
        };
    }
    fn block_268<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_268_0: read-var x:str
        let s_268_0: &'static str = fn_state.x;
        // C s_268_1: const #"Has MTE4 extension" : str
        let s_268_1: &'static str = "Has MTE4 extension";
        // D s_268_2: cmp-eq s_268_0 s_268_1
        let s_268_2: bool = ((s_268_0) == (s_268_1));
        // N s_268_3: branch s_268_2 b381 b269
        if s_268_2 {
            return block_381(state, tracer, fn_state);
        } else {
            return block_269(state, tracer, fn_state);
        };
    }
    fn block_269<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_269_0: read-var x:str
        let s_269_0: &'static str = fn_state.x;
        // C s_269_1: const #"Has Canonical Tag Checking support" : str
        let s_269_1: &'static str = "Has Canonical Tag Checking support";
        // D s_269_2: cmp-eq s_269_0 s_269_1
        let s_269_2: bool = ((s_269_0) == (s_269_1));
        // N s_269_3: branch s_269_2 b380 b270
        if s_269_2 {
            return block_380(state, tracer, fn_state);
        } else {
            return block_270(state, tracer, fn_state);
        };
    }
    fn block_270<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_270_0: read-var x:str
        let s_270_0: &'static str = fn_state.x;
        // C s_270_1: const #"G1 activity monitor is implemented for counter 0" : str
        let s_270_1: &'static str = "G1 activity monitor is implemented for counter 0";
        // D s_270_2: cmp-eq s_270_0 s_270_1
        let s_270_2: bool = ((s_270_0) == (s_270_1));
        // N s_270_3: branch s_270_2 b379 b271
        if s_270_2 {
            return block_379(state, tracer, fn_state);
        } else {
            return block_271(state, tracer, fn_state);
        };
    }
    fn block_271<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_271_0: read-var x:str
        let s_271_0: &'static str = fn_state.x;
        // C s_271_1: const #"G1 activity monitor is implemented for counter 1" : str
        let s_271_1: &'static str = "G1 activity monitor is implemented for counter 1";
        // D s_271_2: cmp-eq s_271_0 s_271_1
        let s_271_2: bool = ((s_271_0) == (s_271_1));
        // N s_271_3: branch s_271_2 b378 b272
        if s_271_2 {
            return block_378(state, tracer, fn_state);
        } else {
            return block_272(state, tracer, fn_state);
        };
    }
    fn block_272<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_272_0: read-var x:str
        let s_272_0: &'static str = fn_state.x;
        // C s_272_1: const #"G1 activity monitor is implemented for counter 2" : str
        let s_272_1: &'static str = "G1 activity monitor is implemented for counter 2";
        // D s_272_2: cmp-eq s_272_0 s_272_1
        let s_272_2: bool = ((s_272_0) == (s_272_1));
        // N s_272_3: branch s_272_2 b377 b273
        if s_272_2 {
            return block_377(state, tracer, fn_state);
        } else {
            return block_273(state, tracer, fn_state);
        };
    }
    fn block_273<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_273_0: read-var x:str
        let s_273_0: &'static str = fn_state.x;
        // C s_273_1: const #"G1 activity monitor is implemented for counter 3" : str
        let s_273_1: &'static str = "G1 activity monitor is implemented for counter 3";
        // D s_273_2: cmp-eq s_273_0 s_273_1
        let s_273_2: bool = ((s_273_0) == (s_273_1));
        // N s_273_3: branch s_273_2 b376 b274
        if s_273_2 {
            return block_376(state, tracer, fn_state);
        } else {
            return block_274(state, tracer, fn_state);
        };
    }
    fn block_274<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_274_0: read-var x:str
        let s_274_0: &'static str = fn_state.x;
        // C s_274_1: const #"G1 activity monitor is implemented for counter 4" : str
        let s_274_1: &'static str = "G1 activity monitor is implemented for counter 4";
        // D s_274_2: cmp-eq s_274_0 s_274_1
        let s_274_2: bool = ((s_274_0) == (s_274_1));
        // N s_274_3: branch s_274_2 b375 b275
        if s_274_2 {
            return block_375(state, tracer, fn_state);
        } else {
            return block_275(state, tracer, fn_state);
        };
    }
    fn block_275<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_275_0: read-var x:str
        let s_275_0: &'static str = fn_state.x;
        // C s_275_1: const #"G1 activity monitor is implemented for counter 5" : str
        let s_275_1: &'static str = "G1 activity monitor is implemented for counter 5";
        // D s_275_2: cmp-eq s_275_0 s_275_1
        let s_275_2: bool = ((s_275_0) == (s_275_1));
        // N s_275_3: branch s_275_2 b374 b276
        if s_275_2 {
            return block_374(state, tracer, fn_state);
        } else {
            return block_276(state, tracer, fn_state);
        };
    }
    fn block_276<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_276_0: read-var x:str
        let s_276_0: &'static str = fn_state.x;
        // C s_276_1: const #"G1 activity monitor is implemented for counter 6" : str
        let s_276_1: &'static str = "G1 activity monitor is implemented for counter 6";
        // D s_276_2: cmp-eq s_276_0 s_276_1
        let s_276_2: bool = ((s_276_0) == (s_276_1));
        // N s_276_3: branch s_276_2 b373 b277
        if s_276_2 {
            return block_373(state, tracer, fn_state);
        } else {
            return block_277(state, tracer, fn_state);
        };
    }
    fn block_277<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_277_0: read-var x:str
        let s_277_0: &'static str = fn_state.x;
        // C s_277_1: const #"G1 activity monitor is implemented for counter 7" : str
        let s_277_1: &'static str = "G1 activity monitor is implemented for counter 7";
        // D s_277_2: cmp-eq s_277_0 s_277_1
        let s_277_2: bool = ((s_277_0) == (s_277_1));
        // N s_277_3: branch s_277_2 b372 b278
        if s_277_2 {
            return block_372(state, tracer, fn_state);
        } else {
            return block_278(state, tracer, fn_state);
        };
    }
    fn block_278<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_278_0: read-var x:str
        let s_278_0: &'static str = fn_state.x;
        // C s_278_1: const #"G1 activity monitor is implemented for counter 8" : str
        let s_278_1: &'static str = "G1 activity monitor is implemented for counter 8";
        // D s_278_2: cmp-eq s_278_0 s_278_1
        let s_278_2: bool = ((s_278_0) == (s_278_1));
        // N s_278_3: branch s_278_2 b371 b279
        if s_278_2 {
            return block_371(state, tracer, fn_state);
        } else {
            return block_279(state, tracer, fn_state);
        };
    }
    fn block_279<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_279_0: read-var x:str
        let s_279_0: &'static str = fn_state.x;
        // C s_279_1: const #"G1 activity monitor is implemented for counter 9" : str
        let s_279_1: &'static str = "G1 activity monitor is implemented for counter 9";
        // D s_279_2: cmp-eq s_279_0 s_279_1
        let s_279_2: bool = ((s_279_0) == (s_279_1));
        // N s_279_3: branch s_279_2 b370 b280
        if s_279_2 {
            return block_370(state, tracer, fn_state);
        } else {
            return block_280(state, tracer, fn_state);
        };
    }
    fn block_280<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_280_0: read-var x:str
        let s_280_0: &'static str = fn_state.x;
        // C s_280_1: const #"G1 activity monitor is implemented for counter 10" : str
        let s_280_1: &'static str = "G1 activity monitor is implemented for counter 10";
        // D s_280_2: cmp-eq s_280_0 s_280_1
        let s_280_2: bool = ((s_280_0) == (s_280_1));
        // N s_280_3: branch s_280_2 b369 b281
        if s_280_2 {
            return block_369(state, tracer, fn_state);
        } else {
            return block_281(state, tracer, fn_state);
        };
    }
    fn block_281<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_281_0: read-var x:str
        let s_281_0: &'static str = fn_state.x;
        // C s_281_1: const #"G1 activity monitor is implemented for counter 11" : str
        let s_281_1: &'static str = "G1 activity monitor is implemented for counter 11";
        // D s_281_2: cmp-eq s_281_0 s_281_1
        let s_281_2: bool = ((s_281_0) == (s_281_1));
        // N s_281_3: branch s_281_2 b368 b282
        if s_281_2 {
            return block_368(state, tracer, fn_state);
        } else {
            return block_282(state, tracer, fn_state);
        };
    }
    fn block_282<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_282_0: read-var x:str
        let s_282_0: &'static str = fn_state.x;
        // C s_282_1: const #"G1 activity monitor is implemented for counter 12" : str
        let s_282_1: &'static str = "G1 activity monitor is implemented for counter 12";
        // D s_282_2: cmp-eq s_282_0 s_282_1
        let s_282_2: bool = ((s_282_0) == (s_282_1));
        // N s_282_3: branch s_282_2 b367 b283
        if s_282_2 {
            return block_367(state, tracer, fn_state);
        } else {
            return block_283(state, tracer, fn_state);
        };
    }
    fn block_283<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_283_0: read-var x:str
        let s_283_0: &'static str = fn_state.x;
        // C s_283_1: const #"G1 activity monitor is implemented for counter 13" : str
        let s_283_1: &'static str = "G1 activity monitor is implemented for counter 13";
        // D s_283_2: cmp-eq s_283_0 s_283_1
        let s_283_2: bool = ((s_283_0) == (s_283_1));
        // N s_283_3: branch s_283_2 b366 b284
        if s_283_2 {
            return block_366(state, tracer, fn_state);
        } else {
            return block_284(state, tracer, fn_state);
        };
    }
    fn block_284<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_284_0: read-var x:str
        let s_284_0: &'static str = fn_state.x;
        // C s_284_1: const #"G1 activity monitor is implemented for counter 14" : str
        let s_284_1: &'static str = "G1 activity monitor is implemented for counter 14";
        // D s_284_2: cmp-eq s_284_0 s_284_1
        let s_284_2: bool = ((s_284_0) == (s_284_1));
        // N s_284_3: branch s_284_2 b365 b285
        if s_284_2 {
            return block_365(state, tracer, fn_state);
        } else {
            return block_285(state, tracer, fn_state);
        };
    }
    fn block_285<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_285_0: read-var x:str
        let s_285_0: &'static str = fn_state.x;
        // C s_285_1: const #"G1 activity monitor is implemented for counter 15" : str
        let s_285_1: &'static str = "G1 activity monitor is implemented for counter 15";
        // D s_285_2: cmp-eq s_285_0 s_285_1
        let s_285_2: bool = ((s_285_0) == (s_285_1));
        // N s_285_3: branch s_285_2 b364 b286
        if s_285_2 {
            return block_364(state, tracer, fn_state);
        } else {
            return block_286(state, tracer, fn_state);
        };
    }
    fn block_286<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_286_0: read-var x:str
        let s_286_0: &'static str = fn_state.x;
        // C s_286_1: const #"G1 activity monitor offset is implemented for counter 0" : str
        let s_286_1: &'static str = "G1 activity monitor offset is implemented for counter 0";
        // D s_286_2: cmp-eq s_286_0 s_286_1
        let s_286_2: bool = ((s_286_0) == (s_286_1));
        // N s_286_3: branch s_286_2 b363 b287
        if s_286_2 {
            return block_363(state, tracer, fn_state);
        } else {
            return block_287(state, tracer, fn_state);
        };
    }
    fn block_287<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_287_0: read-var x:str
        let s_287_0: &'static str = fn_state.x;
        // C s_287_1: const #"G1 activity monitor offset is implemented for counter 1" : str
        let s_287_1: &'static str = "G1 activity monitor offset is implemented for counter 1";
        // D s_287_2: cmp-eq s_287_0 s_287_1
        let s_287_2: bool = ((s_287_0) == (s_287_1));
        // N s_287_3: branch s_287_2 b362 b288
        if s_287_2 {
            return block_362(state, tracer, fn_state);
        } else {
            return block_288(state, tracer, fn_state);
        };
    }
    fn block_288<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_288_0: read-var x:str
        let s_288_0: &'static str = fn_state.x;
        // C s_288_1: const #"G1 activity monitor offset is implemented for counter 2" : str
        let s_288_1: &'static str = "G1 activity monitor offset is implemented for counter 2";
        // D s_288_2: cmp-eq s_288_0 s_288_1
        let s_288_2: bool = ((s_288_0) == (s_288_1));
        // N s_288_3: branch s_288_2 b361 b289
        if s_288_2 {
            return block_361(state, tracer, fn_state);
        } else {
            return block_289(state, tracer, fn_state);
        };
    }
    fn block_289<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_289_0: read-var x:str
        let s_289_0: &'static str = fn_state.x;
        // C s_289_1: const #"G1 activity monitor offset is implemented for counter 3" : str
        let s_289_1: &'static str = "G1 activity monitor offset is implemented for counter 3";
        // D s_289_2: cmp-eq s_289_0 s_289_1
        let s_289_2: bool = ((s_289_0) == (s_289_1));
        // N s_289_3: branch s_289_2 b360 b290
        if s_289_2 {
            return block_360(state, tracer, fn_state);
        } else {
            return block_290(state, tracer, fn_state);
        };
    }
    fn block_290<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_290_0: read-var x:str
        let s_290_0: &'static str = fn_state.x;
        // C s_290_1: const #"G1 activity monitor offset is implemented for counter 4" : str
        let s_290_1: &'static str = "G1 activity monitor offset is implemented for counter 4";
        // D s_290_2: cmp-eq s_290_0 s_290_1
        let s_290_2: bool = ((s_290_0) == (s_290_1));
        // N s_290_3: branch s_290_2 b359 b291
        if s_290_2 {
            return block_359(state, tracer, fn_state);
        } else {
            return block_291(state, tracer, fn_state);
        };
    }
    fn block_291<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_291_0: read-var x:str
        let s_291_0: &'static str = fn_state.x;
        // C s_291_1: const #"G1 activity monitor offset is implemented for counter 5" : str
        let s_291_1: &'static str = "G1 activity monitor offset is implemented for counter 5";
        // D s_291_2: cmp-eq s_291_0 s_291_1
        let s_291_2: bool = ((s_291_0) == (s_291_1));
        // N s_291_3: branch s_291_2 b358 b292
        if s_291_2 {
            return block_358(state, tracer, fn_state);
        } else {
            return block_292(state, tracer, fn_state);
        };
    }
    fn block_292<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_292_0: read-var x:str
        let s_292_0: &'static str = fn_state.x;
        // C s_292_1: const #"G1 activity monitor offset is implemented for counter 6" : str
        let s_292_1: &'static str = "G1 activity monitor offset is implemented for counter 6";
        // D s_292_2: cmp-eq s_292_0 s_292_1
        let s_292_2: bool = ((s_292_0) == (s_292_1));
        // N s_292_3: branch s_292_2 b357 b293
        if s_292_2 {
            return block_357(state, tracer, fn_state);
        } else {
            return block_293(state, tracer, fn_state);
        };
    }
    fn block_293<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_293_0: read-var x:str
        let s_293_0: &'static str = fn_state.x;
        // C s_293_1: const #"G1 activity monitor offset is implemented for counter 7" : str
        let s_293_1: &'static str = "G1 activity monitor offset is implemented for counter 7";
        // D s_293_2: cmp-eq s_293_0 s_293_1
        let s_293_2: bool = ((s_293_0) == (s_293_1));
        // N s_293_3: branch s_293_2 b356 b294
        if s_293_2 {
            return block_356(state, tracer, fn_state);
        } else {
            return block_294(state, tracer, fn_state);
        };
    }
    fn block_294<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_294_0: read-var x:str
        let s_294_0: &'static str = fn_state.x;
        // C s_294_1: const #"G1 activity monitor offset is implemented for counter 8" : str
        let s_294_1: &'static str = "G1 activity monitor offset is implemented for counter 8";
        // D s_294_2: cmp-eq s_294_0 s_294_1
        let s_294_2: bool = ((s_294_0) == (s_294_1));
        // N s_294_3: branch s_294_2 b355 b295
        if s_294_2 {
            return block_355(state, tracer, fn_state);
        } else {
            return block_295(state, tracer, fn_state);
        };
    }
    fn block_295<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_295_0: read-var x:str
        let s_295_0: &'static str = fn_state.x;
        // C s_295_1: const #"G1 activity monitor offset is implemented for counter 9" : str
        let s_295_1: &'static str = "G1 activity monitor offset is implemented for counter 9";
        // D s_295_2: cmp-eq s_295_0 s_295_1
        let s_295_2: bool = ((s_295_0) == (s_295_1));
        // N s_295_3: branch s_295_2 b354 b296
        if s_295_2 {
            return block_354(state, tracer, fn_state);
        } else {
            return block_296(state, tracer, fn_state);
        };
    }
    fn block_296<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_296_0: read-var x:str
        let s_296_0: &'static str = fn_state.x;
        // C s_296_1: const #"G1 activity monitor offset is implemented for counter 10" : str
        let s_296_1: &'static str = "G1 activity monitor offset is implemented for counter 10";
        // D s_296_2: cmp-eq s_296_0 s_296_1
        let s_296_2: bool = ((s_296_0) == (s_296_1));
        // N s_296_3: branch s_296_2 b353 b297
        if s_296_2 {
            return block_353(state, tracer, fn_state);
        } else {
            return block_297(state, tracer, fn_state);
        };
    }
    fn block_297<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_297_0: read-var x:str
        let s_297_0: &'static str = fn_state.x;
        // C s_297_1: const #"G1 activity monitor offset is implemented for counter 11" : str
        let s_297_1: &'static str = "G1 activity monitor offset is implemented for counter 11";
        // D s_297_2: cmp-eq s_297_0 s_297_1
        let s_297_2: bool = ((s_297_0) == (s_297_1));
        // N s_297_3: branch s_297_2 b352 b298
        if s_297_2 {
            return block_352(state, tracer, fn_state);
        } else {
            return block_298(state, tracer, fn_state);
        };
    }
    fn block_298<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_298_0: read-var x:str
        let s_298_0: &'static str = fn_state.x;
        // C s_298_1: const #"G1 activity monitor offset is implemented for counter 12" : str
        let s_298_1: &'static str = "G1 activity monitor offset is implemented for counter 12";
        // D s_298_2: cmp-eq s_298_0 s_298_1
        let s_298_2: bool = ((s_298_0) == (s_298_1));
        // N s_298_3: branch s_298_2 b351 b299
        if s_298_2 {
            return block_351(state, tracer, fn_state);
        } else {
            return block_299(state, tracer, fn_state);
        };
    }
    fn block_299<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_299_0: read-var x:str
        let s_299_0: &'static str = fn_state.x;
        // C s_299_1: const #"G1 activity monitor offset is implemented for counter 13" : str
        let s_299_1: &'static str = "G1 activity monitor offset is implemented for counter 13";
        // D s_299_2: cmp-eq s_299_0 s_299_1
        let s_299_2: bool = ((s_299_0) == (s_299_1));
        // N s_299_3: branch s_299_2 b350 b300
        if s_299_2 {
            return block_350(state, tracer, fn_state);
        } else {
            return block_300(state, tracer, fn_state);
        };
    }
    fn block_300<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_300_0: read-var x:str
        let s_300_0: &'static str = fn_state.x;
        // C s_300_1: const #"G1 activity monitor offset is implemented for counter 14" : str
        let s_300_1: &'static str = "G1 activity monitor offset is implemented for counter 14";
        // D s_300_2: cmp-eq s_300_0 s_300_1
        let s_300_2: bool = ((s_300_0) == (s_300_1));
        // N s_300_3: branch s_300_2 b349 b301
        if s_300_2 {
            return block_349(state, tracer, fn_state);
        } else {
            return block_301(state, tracer, fn_state);
        };
    }
    fn block_301<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_301_0: read-var x:str
        let s_301_0: &'static str = fn_state.x;
        // C s_301_1: const #"G1 activity monitor offset is implemented for counter 15" : str
        let s_301_1: &'static str = "G1 activity monitor offset is implemented for counter 15";
        // D s_301_2: cmp-eq s_301_0 s_301_1
        let s_301_2: bool = ((s_301_0) == (s_301_1));
        // N s_301_3: branch s_301_2 b348 b302
        if s_301_2 {
            return block_348(state, tracer, fn_state);
        } else {
            return block_302(state, tracer, fn_state);
        };
    }
    fn block_302<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_302_0: read-var x:str
        let s_302_0: &'static str = fn_state.x;
        // C s_302_1: const #"No fault generated for DC operations if PoC is before any level of cache" : str
        let s_302_1: &'static str = "No fault generated for DC operations if PoC is before any level of cache";
        // D s_302_2: cmp-eq s_302_0 s_302_1
        let s_302_2: bool = ((s_302_0) == (s_302_1));
        // N s_302_3: branch s_302_2 b347 b303
        if s_302_2 {
            return block_347(state, tracer, fn_state);
        } else {
            return block_303(state, tracer, fn_state);
        };
    }
    fn block_303<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_303_0: read-var x:str
        let s_303_0: &'static str = fn_state.x;
        // C s_303_1: const #"No fault generated for DC operations if PoU is before any level of cache" : str
        let s_303_1: &'static str = "No fault generated for DC operations if PoU is before any level of cache";
        // D s_303_2: cmp-eq s_303_0 s_303_1
        let s_303_2: bool = ((s_303_0) == (s_303_1));
        // N s_303_3: branch s_303_2 b346 b304
        if s_303_2 {
            return block_346(state, tracer, fn_state);
        } else {
            return block_304(state, tracer, fn_state);
        };
    }
    fn block_304<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_304_0: read-var x:str
        let s_304_0: &'static str = fn_state.x;
        // C s_304_1: const #"Generate access flag fault on IC/DC operations" : str
        let s_304_1: &'static str = "Generate access flag fault on IC/DC operations";
        // D s_304_2: cmp-eq s_304_0 s_304_1
        let s_304_2: bool = ((s_304_0) == (s_304_1));
        // N s_304_3: branch s_304_2 b345 b305
        if s_304_2 {
            return block_345(state, tracer, fn_state);
        } else {
            return block_305(state, tracer, fn_state);
        };
    }
    fn block_305<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_305_0: read-var x:str
        let s_305_0: &'static str = fn_state.x;
        // C s_305_1: const #"Generate translation fault on IC operations" : str
        let s_305_1: &'static str = "Generate translation fault on IC operations";
        // D s_305_2: cmp-eq s_305_0 s_305_1
        let s_305_2: bool = ((s_305_0) == (s_305_1));
        // N s_305_3: branch s_305_2 b344 b306
        if s_305_2 {
            return block_344(state, tracer, fn_state);
        } else {
            return block_306(state, tracer, fn_state);
        };
    }
    fn block_306<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_306_0: read-var x:str
        let s_306_0: &'static str = fn_state.x;
        // C s_306_1: const #"Generate access flag fault on IC operations" : str
        let s_306_1: &'static str = "Generate access flag fault on IC operations";
        // D s_306_2: cmp-eq s_306_0 s_306_1
        let s_306_2: bool = ((s_306_0) == (s_306_1));
        // N s_306_3: branch s_306_2 b343 b307
        if s_306_2 {
            return block_343(state, tracer, fn_state);
        } else {
            return block_307(state, tracer, fn_state);
        };
    }
    fn block_307<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_307_0: read-var x:str
        let s_307_0: &'static str = fn_state.x;
        // C s_307_1: const #"Generate address size fault on IC operations" : str
        let s_307_1: &'static str = "Generate address size fault on IC operations";
        // D s_307_2: cmp-eq s_307_0 s_307_1
        let s_307_2: bool = ((s_307_0) == (s_307_1));
        // N s_307_3: branch s_307_2 b342 b308
        if s_307_2 {
            return block_342(state, tracer, fn_state);
        } else {
            return block_308(state, tracer, fn_state);
        };
    }
    fn block_308<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_308_0: read-var x:str
        let s_308_0: &'static str = fn_state.x;
        // C s_308_1: const #"IC_IVAU generates Permission fault at EL0 without read permission" : str
        let s_308_1: &'static str = "IC_IVAU generates Permission fault at EL0 without read permission";
        // D s_308_2: cmp-eq s_308_0 s_308_1
        let s_308_2: bool = ((s_308_0) == (s_308_1));
        // N s_308_3: branch s_308_2 b341 b309
        if s_308_2 {
            return block_341(state, tracer, fn_state);
        } else {
            return block_309(state, tracer, fn_state);
        };
    }
    fn block_309<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_309_0: read-var x:str
        let s_309_0: &'static str = fn_state.x;
        // C s_309_1: const #"Instruction Cache needs translation" : str
        let s_309_1: &'static str = "Instruction Cache needs translation";
        // D s_309_2: cmp-eq s_309_0 s_309_1
        let s_309_2: bool = ((s_309_0) == (s_309_1));
        // N s_309_3: branch s_309_2 b340 b310
        if s_309_2 {
            return block_340(state, tracer, fn_state);
        } else {
            return block_310(state, tracer, fn_state);
        };
    }
    fn block_310<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_310_0: read-var x:str
        let s_310_0: &'static str = fn_state.x;
        // C s_310_1: const #"reset into AArch32" : str
        let s_310_1: &'static str = "reset into AArch32";
        // D s_310_2: cmp-eq s_310_0 s_310_1
        let s_310_2: bool = ((s_310_0) == (s_310_1));
        // N s_310_3: branch s_310_2 b339 b311
        if s_310_2 {
            return block_339(state, tracer, fn_state);
        } else {
            return block_311(state, tracer, fn_state);
        };
    }
    fn block_311<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_311_0: read-var x:str
        let s_311_0: &'static str = fn_state.x;
        // C s_311_1: const #"Memory system does not supports PoP" : str
        let s_311_1: &'static str = "Memory system does not supports PoP";
        // D s_311_2: cmp-eq s_311_0 s_311_1
        let s_311_2: bool = ((s_311_0) == (s_311_1));
        // N s_311_3: branch s_311_2 b338 b312
        if s_311_2 {
            return block_338(state, tracer, fn_state);
        } else {
            return block_312(state, tracer, fn_state);
        };
    }
    fn block_312<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_312_0: read-var x:str
        let s_312_0: &'static str = fn_state.x;
        // C s_312_1: const #"Memory system does not supports PoDP" : str
        let s_312_1: &'static str = "Memory system does not supports PoDP";
        // D s_312_2: cmp-eq s_312_0 s_312_1
        let s_312_2: bool = ((s_312_0) == (s_312_1));
        // N s_312_3: branch s_312_2 b337 b313
        if s_312_2 {
            return block_337(state, tracer, fn_state);
        } else {
            return block_313(state, tracer, fn_state);
        };
    }
    fn block_313<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_313_0: read-var x:str
        let s_313_0: &'static str = fn_state.x;
        // C s_313_1: const #"Arch has AIE" : str
        let s_313_1: &'static str = "Arch has AIE";
        // D s_313_2: cmp-eq s_313_0 s_313_1
        let s_313_2: bool = ((s_313_0) == (s_313_1));
        // N s_313_3: branch s_313_2 b336 b314
        if s_313_2 {
            return block_336(state, tracer, fn_state);
        } else {
            return block_314(state, tracer, fn_state);
        };
    }
    fn block_314<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_314_0: read-var x:str
        let s_314_0: &'static str = fn_state.x;
        // C s_314_1: const #"read" : str
        let s_314_1: &'static str = "read";
        // D s_314_2: cmp-eq s_314_0 s_314_1
        let s_314_2: bool = ((s_314_0) == (s_314_1));
        // N s_314_3: branch s_314_2 b335 b315
        if s_314_2 {
            return block_335(state, tracer, fn_state);
        } else {
            return block_315(state, tracer, fn_state);
        };
    }
    fn block_315<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_315_0: read-var x:str
        let s_315_0: &'static str = fn_state.x;
        // C s_315_1: const #"written" : str
        let s_315_1: &'static str = "written";
        // D s_315_2: cmp-eq s_315_0 s_315_1
        let s_315_2: bool = ((s_315_0) == (s_315_1));
        // N s_315_3: branch s_315_2 b334 b316
        if s_315_2 {
            return block_334(state, tracer, fn_state);
        } else {
            return block_316(state, tracer, fn_state);
        };
    }
    fn block_316<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_316_0: read-var x:str
        let s_316_0: &'static str = fn_state.x;
        // C s_316_1: const #"PFAR_ELx is valid" : str
        let s_316_1: &'static str = "PFAR_ELx is valid";
        // D s_316_2: cmp-eq s_316_0 s_316_1
        let s_316_2: bool = ((s_316_0) == (s_316_1));
        // N s_316_3: branch s_316_2 b333 b317
        if s_316_2 {
            return block_333(state, tracer, fn_state);
        } else {
            return block_317(state, tracer, fn_state);
        };
    }
    fn block_317<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_317_0: read-var x:str
        let s_317_0: &'static str = fn_state.x;
        // C s_317_1: const #"Apply granule protection check on DC to PoE" : str
        let s_317_1: &'static str = "Apply granule protection check on DC to PoE";
        // D s_317_2: cmp-eq s_317_0 s_317_1
        let s_317_2: bool = ((s_317_0) == (s_317_1));
        // N s_317_3: branch s_317_2 b332 b318
        if s_317_2 {
            return block_332(state, tracer, fn_state);
        } else {
            return block_318(state, tracer, fn_state);
        };
    }
    fn block_318<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_318_0: read-var x:str
        let s_318_0: &'static str = fn_state.x;
        // C s_318_1: const #"IMPLEMENTED_trapping of Input Denormal floating-point exceptions" : str
        let s_318_1: &'static str = "IMPLEMENTED_trapping of Input Denormal floating-point exceptions";
        // D s_318_2: cmp-eq s_318_0 s_318_1
        let s_318_2: bool = ((s_318_0) == (s_318_1));
        // N s_318_3: branch s_318_2 b331 b319
        if s_318_2 {
            return block_331(state, tracer, fn_state);
        } else {
            return block_319(state, tracer, fn_state);
        };
    }
    fn block_319<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_319_0: read-var x:str
        let s_319_0: &'static str = fn_state.x;
        // C s_319_1: const #"IMPLEMENTED_trapping of Inexact floating-point exceptions" : str
        let s_319_1: &'static str = "IMPLEMENTED_trapping of Inexact floating-point exceptions";
        // D s_319_2: cmp-eq s_319_0 s_319_1
        let s_319_2: bool = ((s_319_0) == (s_319_1));
        // N s_319_3: branch s_319_2 b330 b320
        if s_319_2 {
            return block_330(state, tracer, fn_state);
        } else {
            return block_320(state, tracer, fn_state);
        };
    }
    fn block_320<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_320_0: read-var x:str
        let s_320_0: &'static str = fn_state.x;
        // C s_320_1: const #"IMPLEMENTED_trapping of Underflow floating-point exceptions" : str
        let s_320_1: &'static str = "IMPLEMENTED_trapping of Underflow floating-point exceptions";
        // D s_320_2: cmp-eq s_320_0 s_320_1
        let s_320_2: bool = ((s_320_0) == (s_320_1));
        // N s_320_3: branch s_320_2 b329 b321
        if s_320_2 {
            return block_329(state, tracer, fn_state);
        } else {
            return block_321(state, tracer, fn_state);
        };
    }
    fn block_321<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_321_0: read-var x:str
        let s_321_0: &'static str = fn_state.x;
        // C s_321_1: const #"IMPLEMENTED_trapping of Overflow floating-point exceptions" : str
        let s_321_1: &'static str = "IMPLEMENTED_trapping of Overflow floating-point exceptions";
        // D s_321_2: cmp-eq s_321_0 s_321_1
        let s_321_2: bool = ((s_321_0) == (s_321_1));
        // N s_321_3: branch s_321_2 b328 b322
        if s_321_2 {
            return block_328(state, tracer, fn_state);
        } else {
            return block_322(state, tracer, fn_state);
        };
    }
    fn block_322<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_322_0: read-var x:str
        let s_322_0: &'static str = fn_state.x;
        // C s_322_1: const #"IMPLEMENTED_trapping of Divide by Zero floating-point exceptions" : str
        let s_322_1: &'static str = "IMPLEMENTED_trapping of Divide by Zero floating-point exceptions";
        // D s_322_2: cmp-eq s_322_0 s_322_1
        let s_322_2: bool = ((s_322_0) == (s_322_1));
        // N s_322_3: branch s_322_2 b327 b323
        if s_322_2 {
            return block_327(state, tracer, fn_state);
        } else {
            return block_323(state, tracer, fn_state);
        };
    }
    fn block_323<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_323_0: read-var x:str
        let s_323_0: &'static str = fn_state.x;
        // C s_323_1: const #"IMPLEMENTED_trapping of Invalid Operation floating-point exceptions" : str
        let s_323_1: &'static str = "IMPLEMENTED_trapping of Invalid Operation floating-point exceptions";
        // D s_323_2: cmp-eq s_323_0 s_323_1
        let s_323_2: bool = ((s_323_0) == (s_323_1));
        // N s_323_3: branch s_323_2 b326 b324
        if s_323_2 {
            return block_326(state, tracer, fn_state);
        } else {
            return block_324(state, tracer, fn_state);
        };
    }
    fn block_324<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_324_0: const #0u : u8
        let s_324_0: bool = false;
        // N s_324_1: assert s_324_0
        let s_324_1: () = assert!(s_324_0);
        // N s_324_2: jump b325
        return block_325(state, tracer, fn_state);
    }
    fn block_325<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_325_0: read-var return_value:u8
        let s_325_0: bool = fn_state.return_value;
        // N s_325_1: return s_325_0
        return s_325_0;
    }
    fn block_326<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_326_0: const #1u : u8
        let s_326_0: bool = true;
        // D s_326_1: write-var return_value <= s_326_0
        fn_state.return_value = s_326_0;
        // N s_326_2: jump b325
        return block_325(state, tracer, fn_state);
    }
    fn block_327<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_327_0: const #1u : u8
        let s_327_0: bool = true;
        // D s_327_1: write-var return_value <= s_327_0
        fn_state.return_value = s_327_0;
        // N s_327_2: jump b325
        return block_325(state, tracer, fn_state);
    }
    fn block_328<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_328_0: const #1u : u8
        let s_328_0: bool = true;
        // D s_328_1: write-var return_value <= s_328_0
        fn_state.return_value = s_328_0;
        // N s_328_2: jump b325
        return block_325(state, tracer, fn_state);
    }
    fn block_329<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_329_0: const #1u : u8
        let s_329_0: bool = true;
        // D s_329_1: write-var return_value <= s_329_0
        fn_state.return_value = s_329_0;
        // N s_329_2: jump b325
        return block_325(state, tracer, fn_state);
    }
    fn block_330<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_330_0: const #1u : u8
        let s_330_0: bool = true;
        // D s_330_1: write-var return_value <= s_330_0
        fn_state.return_value = s_330_0;
        // N s_330_2: jump b325
        return block_325(state, tracer, fn_state);
    }
    fn block_331<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_331_0: const #1u : u8
        let s_331_0: bool = true;
        // D s_331_1: write-var return_value <= s_331_0
        fn_state.return_value = s_331_0;
        // N s_331_2: jump b325
        return block_325(state, tracer, fn_state);
    }
    fn block_332<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_332_0: const #0u : u8
        let s_332_0: bool = false;
        // D s_332_1: write-var return_value <= s_332_0
        fn_state.return_value = s_332_0;
        // N s_332_2: jump b325
        return block_325(state, tracer, fn_state);
    }
    fn block_333<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_333_0: const #1u : u8
        let s_333_0: bool = true;
        // D s_333_1: write-var return_value <= s_333_0
        fn_state.return_value = s_333_0;
        // N s_333_2: jump b325
        return block_325(state, tracer, fn_state);
    }
    fn block_334<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_334_0: const #1u : u8
        let s_334_0: bool = true;
        // D s_334_1: write-var return_value <= s_334_0
        fn_state.return_value = s_334_0;
        // N s_334_2: jump b325
        return block_325(state, tracer, fn_state);
    }
    fn block_335<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_335_0: const #1u : u8
        let s_335_0: bool = true;
        // D s_335_1: write-var return_value <= s_335_0
        fn_state.return_value = s_335_0;
        // N s_335_2: jump b325
        return block_325(state, tracer, fn_state);
    }
    fn block_336<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_336_0: const #1u : u8
        let s_336_0: bool = true;
        // D s_336_1: write-var return_value <= s_336_0
        fn_state.return_value = s_336_0;
        // N s_336_2: jump b325
        return block_325(state, tracer, fn_state);
    }
    fn block_337<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_337_0: const #0u : u8
        let s_337_0: bool = false;
        // D s_337_1: write-var return_value <= s_337_0
        fn_state.return_value = s_337_0;
        // N s_337_2: jump b325
        return block_325(state, tracer, fn_state);
    }
    fn block_338<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_338_0: const #0u : u8
        let s_338_0: bool = false;
        // D s_338_1: write-var return_value <= s_338_0
        fn_state.return_value = s_338_0;
        // N s_338_2: jump b325
        return block_325(state, tracer, fn_state);
    }
    fn block_339<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_339_0: const #0u : u8
        let s_339_0: bool = false;
        // D s_339_1: write-var return_value <= s_339_0
        fn_state.return_value = s_339_0;
        // N s_339_2: jump b325
        return block_325(state, tracer, fn_state);
    }
    fn block_340<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_340_0: const #1u : u8
        let s_340_0: bool = true;
        // D s_340_1: write-var return_value <= s_340_0
        fn_state.return_value = s_340_0;
        // N s_340_2: jump b325
        return block_325(state, tracer, fn_state);
    }
    fn block_341<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_341_0: const #1u : u8
        let s_341_0: bool = true;
        // D s_341_1: write-var return_value <= s_341_0
        fn_state.return_value = s_341_0;
        // N s_341_2: jump b325
        return block_325(state, tracer, fn_state);
    }
    fn block_342<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_342_0: const #1u : u8
        let s_342_0: bool = true;
        // D s_342_1: write-var return_value <= s_342_0
        fn_state.return_value = s_342_0;
        // N s_342_2: jump b325
        return block_325(state, tracer, fn_state);
    }
    fn block_343<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_343_0: const #1u : u8
        let s_343_0: bool = true;
        // D s_343_1: write-var return_value <= s_343_0
        fn_state.return_value = s_343_0;
        // N s_343_2: jump b325
        return block_325(state, tracer, fn_state);
    }
    fn block_344<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_344_0: const #1u : u8
        let s_344_0: bool = true;
        // D s_344_1: write-var return_value <= s_344_0
        fn_state.return_value = s_344_0;
        // N s_344_2: jump b325
        return block_325(state, tracer, fn_state);
    }
    fn block_345<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_345_0: const #1u : u8
        let s_345_0: bool = true;
        // D s_345_1: write-var return_value <= s_345_0
        fn_state.return_value = s_345_0;
        // N s_345_2: jump b325
        return block_325(state, tracer, fn_state);
    }
    fn block_346<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_346_0: const #0u : u8
        let s_346_0: bool = false;
        // D s_346_1: write-var return_value <= s_346_0
        fn_state.return_value = s_346_0;
        // N s_346_2: jump b325
        return block_325(state, tracer, fn_state);
    }
    fn block_347<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_347_0: const #0u : u8
        let s_347_0: bool = false;
        // D s_347_1: write-var return_value <= s_347_0
        fn_state.return_value = s_347_0;
        // N s_347_2: jump b325
        return block_325(state, tracer, fn_state);
    }
    fn block_348<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_348_0: const #1u : u8
        let s_348_0: bool = true;
        // D s_348_1: write-var return_value <= s_348_0
        fn_state.return_value = s_348_0;
        // N s_348_2: jump b325
        return block_325(state, tracer, fn_state);
    }
    fn block_349<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_349_0: const #1u : u8
        let s_349_0: bool = true;
        // D s_349_1: write-var return_value <= s_349_0
        fn_state.return_value = s_349_0;
        // N s_349_2: jump b325
        return block_325(state, tracer, fn_state);
    }
    fn block_350<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_350_0: const #1u : u8
        let s_350_0: bool = true;
        // D s_350_1: write-var return_value <= s_350_0
        fn_state.return_value = s_350_0;
        // N s_350_2: jump b325
        return block_325(state, tracer, fn_state);
    }
    fn block_351<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_351_0: const #1u : u8
        let s_351_0: bool = true;
        // D s_351_1: write-var return_value <= s_351_0
        fn_state.return_value = s_351_0;
        // N s_351_2: jump b325
        return block_325(state, tracer, fn_state);
    }
    fn block_352<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_352_0: const #1u : u8
        let s_352_0: bool = true;
        // D s_352_1: write-var return_value <= s_352_0
        fn_state.return_value = s_352_0;
        // N s_352_2: jump b325
        return block_325(state, tracer, fn_state);
    }
    fn block_353<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_353_0: const #1u : u8
        let s_353_0: bool = true;
        // D s_353_1: write-var return_value <= s_353_0
        fn_state.return_value = s_353_0;
        // N s_353_2: jump b325
        return block_325(state, tracer, fn_state);
    }
    fn block_354<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_354_0: const #1u : u8
        let s_354_0: bool = true;
        // D s_354_1: write-var return_value <= s_354_0
        fn_state.return_value = s_354_0;
        // N s_354_2: jump b325
        return block_325(state, tracer, fn_state);
    }
    fn block_355<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_355_0: const #1u : u8
        let s_355_0: bool = true;
        // D s_355_1: write-var return_value <= s_355_0
        fn_state.return_value = s_355_0;
        // N s_355_2: jump b325
        return block_325(state, tracer, fn_state);
    }
    fn block_356<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_356_0: const #1u : u8
        let s_356_0: bool = true;
        // D s_356_1: write-var return_value <= s_356_0
        fn_state.return_value = s_356_0;
        // N s_356_2: jump b325
        return block_325(state, tracer, fn_state);
    }
    fn block_357<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_357_0: const #1u : u8
        let s_357_0: bool = true;
        // D s_357_1: write-var return_value <= s_357_0
        fn_state.return_value = s_357_0;
        // N s_357_2: jump b325
        return block_325(state, tracer, fn_state);
    }
    fn block_358<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_358_0: const #1u : u8
        let s_358_0: bool = true;
        // D s_358_1: write-var return_value <= s_358_0
        fn_state.return_value = s_358_0;
        // N s_358_2: jump b325
        return block_325(state, tracer, fn_state);
    }
    fn block_359<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_359_0: const #1u : u8
        let s_359_0: bool = true;
        // D s_359_1: write-var return_value <= s_359_0
        fn_state.return_value = s_359_0;
        // N s_359_2: jump b325
        return block_325(state, tracer, fn_state);
    }
    fn block_360<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_360_0: const #1u : u8
        let s_360_0: bool = true;
        // D s_360_1: write-var return_value <= s_360_0
        fn_state.return_value = s_360_0;
        // N s_360_2: jump b325
        return block_325(state, tracer, fn_state);
    }
    fn block_361<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_361_0: const #1u : u8
        let s_361_0: bool = true;
        // D s_361_1: write-var return_value <= s_361_0
        fn_state.return_value = s_361_0;
        // N s_361_2: jump b325
        return block_325(state, tracer, fn_state);
    }
    fn block_362<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_362_0: const #1u : u8
        let s_362_0: bool = true;
        // D s_362_1: write-var return_value <= s_362_0
        fn_state.return_value = s_362_0;
        // N s_362_2: jump b325
        return block_325(state, tracer, fn_state);
    }
    fn block_363<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_363_0: const #1u : u8
        let s_363_0: bool = true;
        // D s_363_1: write-var return_value <= s_363_0
        fn_state.return_value = s_363_0;
        // N s_363_2: jump b325
        return block_325(state, tracer, fn_state);
    }
    fn block_364<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_364_0: const #1u : u8
        let s_364_0: bool = true;
        // D s_364_1: write-var return_value <= s_364_0
        fn_state.return_value = s_364_0;
        // N s_364_2: jump b325
        return block_325(state, tracer, fn_state);
    }
    fn block_365<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_365_0: const #1u : u8
        let s_365_0: bool = true;
        // D s_365_1: write-var return_value <= s_365_0
        fn_state.return_value = s_365_0;
        // N s_365_2: jump b325
        return block_325(state, tracer, fn_state);
    }
    fn block_366<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_366_0: const #1u : u8
        let s_366_0: bool = true;
        // D s_366_1: write-var return_value <= s_366_0
        fn_state.return_value = s_366_0;
        // N s_366_2: jump b325
        return block_325(state, tracer, fn_state);
    }
    fn block_367<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_367_0: const #1u : u8
        let s_367_0: bool = true;
        // D s_367_1: write-var return_value <= s_367_0
        fn_state.return_value = s_367_0;
        // N s_367_2: jump b325
        return block_325(state, tracer, fn_state);
    }
    fn block_368<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_368_0: const #1u : u8
        let s_368_0: bool = true;
        // D s_368_1: write-var return_value <= s_368_0
        fn_state.return_value = s_368_0;
        // N s_368_2: jump b325
        return block_325(state, tracer, fn_state);
    }
    fn block_369<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_369_0: const #1u : u8
        let s_369_0: bool = true;
        // D s_369_1: write-var return_value <= s_369_0
        fn_state.return_value = s_369_0;
        // N s_369_2: jump b325
        return block_325(state, tracer, fn_state);
    }
    fn block_370<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_370_0: const #1u : u8
        let s_370_0: bool = true;
        // D s_370_1: write-var return_value <= s_370_0
        fn_state.return_value = s_370_0;
        // N s_370_2: jump b325
        return block_325(state, tracer, fn_state);
    }
    fn block_371<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_371_0: const #1u : u8
        let s_371_0: bool = true;
        // D s_371_1: write-var return_value <= s_371_0
        fn_state.return_value = s_371_0;
        // N s_371_2: jump b325
        return block_325(state, tracer, fn_state);
    }
    fn block_372<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_372_0: const #1u : u8
        let s_372_0: bool = true;
        // D s_372_1: write-var return_value <= s_372_0
        fn_state.return_value = s_372_0;
        // N s_372_2: jump b325
        return block_325(state, tracer, fn_state);
    }
    fn block_373<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_373_0: const #1u : u8
        let s_373_0: bool = true;
        // D s_373_1: write-var return_value <= s_373_0
        fn_state.return_value = s_373_0;
        // N s_373_2: jump b325
        return block_325(state, tracer, fn_state);
    }
    fn block_374<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_374_0: const #1u : u8
        let s_374_0: bool = true;
        // D s_374_1: write-var return_value <= s_374_0
        fn_state.return_value = s_374_0;
        // N s_374_2: jump b325
        return block_325(state, tracer, fn_state);
    }
    fn block_375<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_375_0: const #1u : u8
        let s_375_0: bool = true;
        // D s_375_1: write-var return_value <= s_375_0
        fn_state.return_value = s_375_0;
        // N s_375_2: jump b325
        return block_325(state, tracer, fn_state);
    }
    fn block_376<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_376_0: const #1u : u8
        let s_376_0: bool = true;
        // D s_376_1: write-var return_value <= s_376_0
        fn_state.return_value = s_376_0;
        // N s_376_2: jump b325
        return block_325(state, tracer, fn_state);
    }
    fn block_377<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_377_0: const #1u : u8
        let s_377_0: bool = true;
        // D s_377_1: write-var return_value <= s_377_0
        fn_state.return_value = s_377_0;
        // N s_377_2: jump b325
        return block_325(state, tracer, fn_state);
    }
    fn block_378<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_378_0: const #1u : u8
        let s_378_0: bool = true;
        // D s_378_1: write-var return_value <= s_378_0
        fn_state.return_value = s_378_0;
        // N s_378_2: jump b325
        return block_325(state, tracer, fn_state);
    }
    fn block_379<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_379_0: const #1u : u8
        let s_379_0: bool = true;
        // D s_379_1: write-var return_value <= s_379_0
        fn_state.return_value = s_379_0;
        // N s_379_2: jump b325
        return block_325(state, tracer, fn_state);
    }
    fn block_380<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_380_0: const #90920u : u32
        let s_380_0: u32 = 90920;
        // D s_380_1: read-reg s_380_0:u8
        let s_380_1: bool = {
            let value = state.read_register::<bool>(s_380_0 as isize);
            tracer.read_register(s_380_0 as isize, value);
            value
        };
        // D s_380_2: write-var return_value <= s_380_1
        fn_state.return_value = s_380_1;
        // N s_380_3: jump b325
        return block_325(state, tracer, fn_state);
    }
    fn block_381<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_381_0: const #23032u : u32
        let s_381_0: u32 = 23032;
        // D s_381_1: read-reg s_381_0:u8
        let s_381_1: bool = {
            let value = state.read_register::<bool>(s_381_0 as isize);
            tracer.read_register(s_381_0 as isize, value);
            value
        };
        // D s_381_2: write-var return_value <= s_381_1
        fn_state.return_value = s_381_1;
        // N s_381_3: jump b325
        return block_325(state, tracer, fn_state);
    }
    fn block_382<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_382_0: const #0u : u8
        let s_382_0: bool = false;
        // D s_382_1: write-var return_value <= s_382_0
        fn_state.return_value = s_382_0;
        // N s_382_2: jump b325
        return block_325(state, tracer, fn_state);
    }
    fn block_383<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_383_0: const #0u : u8
        let s_383_0: bool = false;
        // D s_383_1: write-var return_value <= s_383_0
        fn_state.return_value = s_383_0;
        // N s_383_2: jump b325
        return block_325(state, tracer, fn_state);
    }
    fn block_384<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_384_0: const #102384u : u32
        let s_384_0: u32 = 102384;
        // D s_384_1: read-reg s_384_0:u8
        let s_384_1: bool = {
            let value = state.read_register::<bool>(s_384_0 as isize);
            tracer.read_register(s_384_0 as isize, value);
            value
        };
        // D s_384_2: write-var return_value <= s_384_1
        fn_state.return_value = s_384_1;
        // N s_384_3: jump b325
        return block_325(state, tracer, fn_state);
    }
    fn block_385<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_385_0: const #0u : u8
        let s_385_0: bool = false;
        // D s_385_1: write-var return_value <= s_385_0
        fn_state.return_value = s_385_0;
        // N s_385_2: jump b325
        return block_325(state, tracer, fn_state);
    }
    fn block_386<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_386_0: const #14912u : u32
        let s_386_0: u32 = 14912;
        // D s_386_1: read-reg s_386_0:u8
        let s_386_1: bool = {
            let value = state.read_register::<bool>(s_386_0 as isize);
            tracer.read_register(s_386_0 as isize, value);
            value
        };
        // D s_386_2: write-var return_value <= s_386_1
        fn_state.return_value = s_386_1;
        // N s_386_3: jump b325
        return block_325(state, tracer, fn_state);
    }
    fn block_387<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_387_0: const #10552u : u32
        let s_387_0: u32 = 10552;
        // D s_387_1: read-reg s_387_0:u8
        let s_387_1: bool = {
            let value = state.read_register::<bool>(s_387_0 as isize);
            tracer.read_register(s_387_0 as isize, value);
            value
        };
        // D s_387_2: write-var return_value <= s_387_1
        fn_state.return_value = s_387_1;
        // N s_387_3: jump b325
        return block_325(state, tracer, fn_state);
    }
    fn block_388<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_388_0: const #10552u : u32
        let s_388_0: u32 = 10552;
        // D s_388_1: read-reg s_388_0:u8
        let s_388_1: bool = {
            let value = state.read_register::<bool>(s_388_0 as isize);
            tracer.read_register(s_388_0 as isize, value);
            value
        };
        // D s_388_2: write-var return_value <= s_388_1
        fn_state.return_value = s_388_1;
        // N s_388_3: jump b325
        return block_325(state, tracer, fn_state);
    }
    fn block_389<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_389_0: const #10120u : u32
        let s_389_0: u32 = 10120;
        // D s_389_1: read-reg s_389_0:u8
        let s_389_1: bool = {
            let value = state.read_register::<bool>(s_389_0 as isize);
            tracer.read_register(s_389_0 as isize, value);
            value
        };
        // D s_389_2: write-var return_value <= s_389_1
        fn_state.return_value = s_389_1;
        // N s_389_3: jump b325
        return block_325(state, tracer, fn_state);
    }
    fn block_390<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_390_0: const #10120u : u32
        let s_390_0: u32 = 10120;
        // D s_390_1: read-reg s_390_0:u8
        let s_390_1: bool = {
            let value = state.read_register::<bool>(s_390_0 as isize);
            tracer.read_register(s_390_0 as isize, value);
            value
        };
        // D s_390_2: write-var return_value <= s_390_1
        fn_state.return_value = s_390_1;
        // N s_390_3: jump b325
        return block_325(state, tracer, fn_state);
    }
    fn block_391<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_391_0: const #101096u : u32
        let s_391_0: u32 = 101096;
        // D s_391_1: read-reg s_391_0:u8
        let s_391_1: bool = {
            let value = state.read_register::<bool>(s_391_0 as isize);
            tracer.read_register(s_391_0 as isize, value);
            value
        };
        // D s_391_2: write-var return_value <= s_391_1
        fn_state.return_value = s_391_1;
        // N s_391_3: jump b325
        return block_325(state, tracer, fn_state);
    }
    fn block_392<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_392_0: const #101096u : u32
        let s_392_0: u32 = 101096;
        // D s_392_1: read-reg s_392_0:u8
        let s_392_1: bool = {
            let value = state.read_register::<bool>(s_392_0 as isize);
            tracer.read_register(s_392_0 as isize, value);
            value
        };
        // D s_392_2: write-var return_value <= s_392_1
        fn_state.return_value = s_392_1;
        // N s_392_3: jump b325
        return block_325(state, tracer, fn_state);
    }
    fn block_393<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_393_0: const #101096u : u32
        let s_393_0: u32 = 101096;
        // D s_393_1: read-reg s_393_0:u8
        let s_393_1: bool = {
            let value = state.read_register::<bool>(s_393_0 as isize);
            tracer.read_register(s_393_0 as isize, value);
            value
        };
        // N s_393_2: branch s_393_1 b396 b394
        if s_393_1 {
            return block_396(state, tracer, fn_state);
        } else {
            return block_394(state, tracer, fn_state);
        };
    }
    fn block_394<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_394_0: const #10120u : u32
        let s_394_0: u32 = 10120;
        // D s_394_1: read-reg s_394_0:u8
        let s_394_1: bool = {
            let value = state.read_register::<bool>(s_394_0 as isize);
            tracer.read_register(s_394_0 as isize, value);
            value
        };
        // D s_394_2: write-var gs#331366 <= s_394_1
        fn_state.gs_331366 = s_394_1;
        // N s_394_3: jump b395
        return block_395(state, tracer, fn_state);
    }
    fn block_395<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_395_0: read-var gs#331366:u8
        let s_395_0: bool = fn_state.gs_331366;
        // D s_395_1: not s_395_0
        let s_395_1: bool = !s_395_0;
        // D s_395_2: write-var return_value <= s_395_1
        fn_state.return_value = s_395_1;
        // N s_395_3: jump b325
        return block_325(state, tracer, fn_state);
    }
    fn block_396<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_396_0: const #1u : u8
        let s_396_0: bool = true;
        // D s_396_1: write-var gs#331366 <= s_396_0
        fn_state.gs_331366 = s_396_0;
        // N s_396_2: jump b395
        return block_395(state, tracer, fn_state);
    }
    fn block_397<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_397_0: const #10120u : u32
        let s_397_0: u32 = 10120;
        // D s_397_1: read-reg s_397_0:u8
        let s_397_1: bool = {
            let value = state.read_register::<bool>(s_397_0 as isize);
            tracer.read_register(s_397_0 as isize, value);
            value
        };
        // D s_397_2: write-var return_value <= s_397_1
        fn_state.return_value = s_397_1;
        // N s_397_3: jump b325
        return block_325(state, tracer, fn_state);
    }
    fn block_398<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_398_0: const #101096u : u32
        let s_398_0: u32 = 101096;
        // D s_398_1: read-reg s_398_0:u8
        let s_398_1: bool = {
            let value = state.read_register::<bool>(s_398_0 as isize);
            tracer.read_register(s_398_0 as isize, value);
            value
        };
        // D s_398_2: write-var return_value <= s_398_1
        fn_state.return_value = s_398_1;
        // N s_398_3: jump b325
        return block_325(state, tracer, fn_state);
    }
    fn block_399<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_399_0: const #0u : u8
        let s_399_0: bool = false;
        // D s_399_1: write-var return_value <= s_399_0
        fn_state.return_value = s_399_0;
        // N s_399_2: jump b325
        return block_325(state, tracer, fn_state);
    }
    fn block_400<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_400_0: const #0u : u8
        let s_400_0: bool = false;
        // D s_400_1: write-var return_value <= s_400_0
        fn_state.return_value = s_400_0;
        // N s_400_2: jump b325
        return block_325(state, tracer, fn_state);
    }
    fn block_401<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_401_0: const #1u : u8
        let s_401_0: bool = true;
        // D s_401_1: write-var return_value <= s_401_0
        fn_state.return_value = s_401_0;
        // N s_401_2: jump b325
        return block_325(state, tracer, fn_state);
    }
    fn block_402<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_402_0: const #4u : u32
        let s_402_0: u32 = 4;
        // S s_402_1: call HasArchVersion(s_402_0)
        let s_402_1: bool = HasArchVersion(state, tracer, s_402_0);
        // S s_402_2: not s_402_1
        let s_402_2: bool = !s_402_1;
        // D s_402_3: write-var return_value <= s_402_2
        fn_state.return_value = s_402_2;
        // N s_402_4: jump b325
        return block_325(state, tracer, fn_state);
    }
    fn block_403<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_403_0: const #1u : u8
        let s_403_0: bool = true;
        // D s_403_1: write-var return_value <= s_403_0
        fn_state.return_value = s_403_0;
        // N s_403_2: jump b325
        return block_325(state, tracer, fn_state);
    }
    fn block_404<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_404_0: const #1u : u8
        let s_404_0: bool = true;
        // D s_404_1: write-var return_value <= s_404_0
        fn_state.return_value = s_404_0;
        // N s_404_2: jump b325
        return block_325(state, tracer, fn_state);
    }
    fn block_405<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_405_0: const #1u : u8
        let s_405_0: bool = true;
        // D s_405_1: write-var return_value <= s_405_0
        fn_state.return_value = s_405_0;
        // N s_405_2: jump b325
        return block_325(state, tracer, fn_state);
    }
    fn block_406<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_406_0: const #1u : u8
        let s_406_0: bool = true;
        // D s_406_1: write-var return_value <= s_406_0
        fn_state.return_value = s_406_0;
        // N s_406_2: jump b325
        return block_325(state, tracer, fn_state);
    }
    fn block_407<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_407_0: const #90720u : u32
        let s_407_0: u32 = 90720;
        // D s_407_1: read-reg s_407_0:u8
        let s_407_1: bool = {
            let value = state.read_register::<bool>(s_407_0 as isize);
            tracer.read_register(s_407_0 as isize, value);
            value
        };
        // D s_407_2: write-var return_value <= s_407_1
        fn_state.return_value = s_407_1;
        // N s_407_3: jump b325
        return block_325(state, tracer, fn_state);
    }
    fn block_408<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_408_0: const #0u : u8
        let s_408_0: bool = false;
        // D s_408_1: write-var return_value <= s_408_0
        fn_state.return_value = s_408_0;
        // N s_408_2: jump b325
        return block_325(state, tracer, fn_state);
    }
    fn block_409<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_409_0: const #1u : u8
        let s_409_0: bool = true;
        // D s_409_1: write-var return_value <= s_409_0
        fn_state.return_value = s_409_0;
        // N s_409_2: jump b325
        return block_325(state, tracer, fn_state);
    }
    fn block_410<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_410_0: const #17712u : u32
        let s_410_0: u32 = 17712;
        // D s_410_1: read-reg s_410_0:u8
        let s_410_1: bool = {
            let value = state.read_register::<bool>(s_410_0 as isize);
            tracer.read_register(s_410_0 as isize, value);
            value
        };
        // D s_410_2: write-var return_value <= s_410_1
        fn_state.return_value = s_410_1;
        // N s_410_3: jump b325
        return block_325(state, tracer, fn_state);
    }
    fn block_411<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_411_0: const #1u : u8
        let s_411_0: bool = true;
        // D s_411_1: write-var return_value <= s_411_0
        fn_state.return_value = s_411_0;
        // N s_411_2: jump b325
        return block_325(state, tracer, fn_state);
    }
    fn block_412<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_412_0: const #1u : u8
        let s_412_0: bool = true;
        // D s_412_1: write-var return_value <= s_412_0
        fn_state.return_value = s_412_0;
        // N s_412_2: jump b325
        return block_325(state, tracer, fn_state);
    }
    fn block_413<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_413_0: const #1u : u8
        let s_413_0: bool = true;
        // D s_413_1: write-var return_value <= s_413_0
        fn_state.return_value = s_413_0;
        // N s_413_2: jump b325
        return block_325(state, tracer, fn_state);
    }
    fn block_414<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_414_0: const #1u : u8
        let s_414_0: bool = true;
        // D s_414_1: write-var return_value <= s_414_0
        fn_state.return_value = s_414_0;
        // N s_414_2: jump b325
        return block_325(state, tracer, fn_state);
    }
    fn block_415<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_415_0: const #1u : u8
        let s_415_0: bool = true;
        // D s_415_1: write-var return_value <= s_415_0
        fn_state.return_value = s_415_0;
        // N s_415_2: jump b325
        return block_325(state, tracer, fn_state);
    }
    fn block_416<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_416_0: const #1u : u8
        let s_416_0: bool = true;
        // D s_416_1: write-var return_value <= s_416_0
        fn_state.return_value = s_416_0;
        // N s_416_2: jump b325
        return block_325(state, tracer, fn_state);
    }
    fn block_417<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_417_0: const #1u : u8
        let s_417_0: bool = true;
        // D s_417_1: write-var return_value <= s_417_0
        fn_state.return_value = s_417_0;
        // N s_417_2: jump b325
        return block_325(state, tracer, fn_state);
    }
    fn block_418<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_418_0: const #1u : u8
        let s_418_0: bool = true;
        // D s_418_1: write-var return_value <= s_418_0
        fn_state.return_value = s_418_0;
        // N s_418_2: jump b325
        return block_325(state, tracer, fn_state);
    }
    fn block_419<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_419_0: const #1u : u8
        let s_419_0: bool = true;
        // D s_419_1: write-var return_value <= s_419_0
        fn_state.return_value = s_419_0;
        // N s_419_2: jump b325
        return block_325(state, tracer, fn_state);
    }
    fn block_420<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_420_0: const #1u : u8
        let s_420_0: bool = true;
        // D s_420_1: write-var return_value <= s_420_0
        fn_state.return_value = s_420_0;
        // N s_420_2: jump b325
        return block_325(state, tracer, fn_state);
    }
    fn block_421<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_421_0: const #1u : u8
        let s_421_0: bool = true;
        // D s_421_1: write-var return_value <= s_421_0
        fn_state.return_value = s_421_0;
        // N s_421_2: jump b325
        return block_325(state, tracer, fn_state);
    }
    fn block_422<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_422_0: const #1u : u8
        let s_422_0: bool = true;
        // D s_422_1: write-var return_value <= s_422_0
        fn_state.return_value = s_422_0;
        // N s_422_2: jump b325
        return block_325(state, tracer, fn_state);
    }
    fn block_423<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_423_0: const #1u : u8
        let s_423_0: bool = true;
        // D s_423_1: write-var return_value <= s_423_0
        fn_state.return_value = s_423_0;
        // N s_423_2: jump b325
        return block_325(state, tracer, fn_state);
    }
    fn block_424<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_424_0: const #1u : u8
        let s_424_0: bool = true;
        // D s_424_1: write-var return_value <= s_424_0
        fn_state.return_value = s_424_0;
        // N s_424_2: jump b325
        return block_325(state, tracer, fn_state);
    }
    fn block_425<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_425_0: const #1u : u8
        let s_425_0: bool = true;
        // D s_425_1: write-var return_value <= s_425_0
        fn_state.return_value = s_425_0;
        // N s_425_2: jump b325
        return block_325(state, tracer, fn_state);
    }
    fn block_426<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_426_0: const #() : ()
        let s_426_0: () = ();
        // S s_426_1: call HaveStatisticalProfilingv1p2(s_426_0)
        let s_426_1: bool = HaveStatisticalProfilingv1p2(state, tracer, s_426_0);
        // D s_426_2: write-var return_value <= s_426_1
        fn_state.return_value = s_426_1;
        // N s_426_3: jump b325
        return block_325(state, tracer, fn_state);
    }
    fn block_427<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_427_0: const #0u : u8
        let s_427_0: bool = false;
        // D s_427_1: write-var return_value <= s_427_0
        fn_state.return_value = s_427_0;
        // N s_427_2: jump b325
        return block_325(state, tracer, fn_state);
    }
    fn block_428<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_428_0: const #0u : u8
        let s_428_0: bool = false;
        // D s_428_1: write-var return_value <= s_428_0
        fn_state.return_value = s_428_0;
        // N s_428_2: jump b325
        return block_325(state, tracer, fn_state);
    }
    fn block_429<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_429_0: const #0u : u8
        let s_429_0: bool = false;
        // D s_429_1: write-var return_value <= s_429_0
        fn_state.return_value = s_429_0;
        // N s_429_2: jump b325
        return block_325(state, tracer, fn_state);
    }
    fn block_430<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_430_0: const #0u : u8
        let s_430_0: bool = false;
        // D s_430_1: write-var return_value <= s_430_0
        fn_state.return_value = s_430_0;
        // N s_430_2: jump b325
        return block_325(state, tracer, fn_state);
    }
    fn block_431<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_431_0: const #0u : u8
        let s_431_0: bool = false;
        // D s_431_1: write-var return_value <= s_431_0
        fn_state.return_value = s_431_0;
        // N s_431_2: jump b325
        return block_325(state, tracer, fn_state);
    }
    fn block_432<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_432_0: const #0u : u8
        let s_432_0: bool = false;
        // D s_432_1: write-var return_value <= s_432_0
        fn_state.return_value = s_432_0;
        // N s_432_2: jump b325
        return block_325(state, tracer, fn_state);
    }
    fn block_433<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_433_0: const #0u : u8
        let s_433_0: bool = false;
        // D s_433_1: write-var return_value <= s_433_0
        fn_state.return_value = s_433_0;
        // N s_433_2: jump b325
        return block_325(state, tracer, fn_state);
    }
    fn block_434<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_434_0: const #0u : u8
        let s_434_0: bool = false;
        // D s_434_1: write-var return_value <= s_434_0
        fn_state.return_value = s_434_0;
        // N s_434_2: jump b325
        return block_325(state, tracer, fn_state);
    }
    fn block_435<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_435_0: const #0u : u8
        let s_435_0: bool = false;
        // D s_435_1: write-var return_value <= s_435_0
        fn_state.return_value = s_435_0;
        // N s_435_2: jump b325
        return block_325(state, tracer, fn_state);
    }
    fn block_436<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_436_0: const #0u : u8
        let s_436_0: bool = false;
        // D s_436_1: write-var return_value <= s_436_0
        fn_state.return_value = s_436_0;
        // N s_436_2: jump b325
        return block_325(state, tracer, fn_state);
    }
    fn block_437<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_437_0: const #0u : u8
        let s_437_0: bool = false;
        // D s_437_1: write-var return_value <= s_437_0
        fn_state.return_value = s_437_0;
        // N s_437_2: jump b325
        return block_325(state, tracer, fn_state);
    }
    fn block_438<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_438_0: const #0u : u8
        let s_438_0: bool = false;
        // D s_438_1: write-var return_value <= s_438_0
        fn_state.return_value = s_438_0;
        // N s_438_2: jump b325
        return block_325(state, tracer, fn_state);
    }
    fn block_439<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_439_0: const #0u : u8
        let s_439_0: bool = false;
        // D s_439_1: write-var return_value <= s_439_0
        fn_state.return_value = s_439_0;
        // N s_439_2: jump b325
        return block_325(state, tracer, fn_state);
    }
    fn block_440<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_440_0: const #0u : u8
        let s_440_0: bool = false;
        // D s_440_1: write-var return_value <= s_440_0
        fn_state.return_value = s_440_0;
        // N s_440_2: jump b325
        return block_325(state, tracer, fn_state);
    }
    fn block_441<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_441_0: const #0u : u8
        let s_441_0: bool = false;
        // D s_441_1: write-var return_value <= s_441_0
        fn_state.return_value = s_441_0;
        // N s_441_2: jump b325
        return block_325(state, tracer, fn_state);
    }
    fn block_442<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_442_0: const #0u : u8
        let s_442_0: bool = false;
        // D s_442_1: write-var return_value <= s_442_0
        fn_state.return_value = s_442_0;
        // N s_442_2: jump b325
        return block_325(state, tracer, fn_state);
    }
    fn block_443<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_443_0: const #0u : u8
        let s_443_0: bool = false;
        // D s_443_1: write-var return_value <= s_443_0
        fn_state.return_value = s_443_0;
        // N s_443_2: jump b325
        return block_325(state, tracer, fn_state);
    }
    fn block_444<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_444_0: const #0u : u8
        let s_444_0: bool = false;
        // D s_444_1: write-var return_value <= s_444_0
        fn_state.return_value = s_444_0;
        // N s_444_2: jump b325
        return block_325(state, tracer, fn_state);
    }
    fn block_445<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_445_0: const #0u : u8
        let s_445_0: bool = false;
        // D s_445_1: write-var return_value <= s_445_0
        fn_state.return_value = s_445_0;
        // N s_445_2: jump b325
        return block_325(state, tracer, fn_state);
    }
    fn block_446<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_446_0: const #0u : u8
        let s_446_0: bool = false;
        // D s_446_1: write-var return_value <= s_446_0
        fn_state.return_value = s_446_0;
        // N s_446_2: jump b325
        return block_325(state, tracer, fn_state);
    }
    fn block_447<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_447_0: const #0u : u8
        let s_447_0: bool = false;
        // D s_447_1: write-var return_value <= s_447_0
        fn_state.return_value = s_447_0;
        // N s_447_2: jump b325
        return block_325(state, tracer, fn_state);
    }
    fn block_448<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_448_0: const #0u : u8
        let s_448_0: bool = false;
        // D s_448_1: write-var return_value <= s_448_0
        fn_state.return_value = s_448_0;
        // N s_448_2: jump b325
        return block_325(state, tracer, fn_state);
    }
    fn block_449<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_449_0: const #0u : u8
        let s_449_0: bool = false;
        // D s_449_1: write-var return_value <= s_449_0
        fn_state.return_value = s_449_0;
        // N s_449_2: jump b325
        return block_325(state, tracer, fn_state);
    }
    fn block_450<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_450_0: const #0u : u8
        let s_450_0: bool = false;
        // D s_450_1: write-var return_value <= s_450_0
        fn_state.return_value = s_450_0;
        // N s_450_2: jump b325
        return block_325(state, tracer, fn_state);
    }
    fn block_451<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_451_0: const #0u : u8
        let s_451_0: bool = false;
        // D s_451_1: write-var return_value <= s_451_0
        fn_state.return_value = s_451_0;
        // N s_451_2: jump b325
        return block_325(state, tracer, fn_state);
    }
    fn block_452<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_452_0: const #0u : u8
        let s_452_0: bool = false;
        // D s_452_1: write-var return_value <= s_452_0
        fn_state.return_value = s_452_0;
        // N s_452_2: jump b325
        return block_325(state, tracer, fn_state);
    }
    fn block_453<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_453_0: const #0u : u8
        let s_453_0: bool = false;
        // D s_453_1: write-var return_value <= s_453_0
        fn_state.return_value = s_453_0;
        // N s_453_2: jump b325
        return block_325(state, tracer, fn_state);
    }
    fn block_454<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_454_0: const #0u : u8
        let s_454_0: bool = false;
        // D s_454_1: write-var return_value <= s_454_0
        fn_state.return_value = s_454_0;
        // N s_454_2: jump b325
        return block_325(state, tracer, fn_state);
    }
    fn block_455<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_455_0: const #0u : u8
        let s_455_0: bool = false;
        // D s_455_1: write-var return_value <= s_455_0
        fn_state.return_value = s_455_0;
        // N s_455_2: jump b325
        return block_325(state, tracer, fn_state);
    }
    fn block_456<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_456_0: const #0u : u8
        let s_456_0: bool = false;
        // D s_456_1: write-var return_value <= s_456_0
        fn_state.return_value = s_456_0;
        // N s_456_2: jump b325
        return block_325(state, tracer, fn_state);
    }
    fn block_457<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_457_0: const #0u : u8
        let s_457_0: bool = false;
        // D s_457_1: write-var return_value <= s_457_0
        fn_state.return_value = s_457_0;
        // N s_457_2: jump b325
        return block_325(state, tracer, fn_state);
    }
    fn block_458<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_458_0: const #0u : u8
        let s_458_0: bool = false;
        // D s_458_1: write-var return_value <= s_458_0
        fn_state.return_value = s_458_0;
        // N s_458_2: jump b325
        return block_325(state, tracer, fn_state);
    }
    fn block_459<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_459_0: const #0u : u8
        let s_459_0: bool = false;
        // D s_459_1: write-var return_value <= s_459_0
        fn_state.return_value = s_459_0;
        // N s_459_2: jump b325
        return block_325(state, tracer, fn_state);
    }
    fn block_460<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_460_0: const #0u : u8
        let s_460_0: bool = false;
        // D s_460_1: write-var return_value <= s_460_0
        fn_state.return_value = s_460_0;
        // N s_460_2: jump b325
        return block_325(state, tracer, fn_state);
    }
    fn block_461<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_461_0: const #0u : u8
        let s_461_0: bool = false;
        // D s_461_1: write-var return_value <= s_461_0
        fn_state.return_value = s_461_0;
        // N s_461_2: jump b325
        return block_325(state, tracer, fn_state);
    }
    fn block_462<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_462_0: const #0u : u8
        let s_462_0: bool = false;
        // D s_462_1: write-var return_value <= s_462_0
        fn_state.return_value = s_462_0;
        // N s_462_2: jump b325
        return block_325(state, tracer, fn_state);
    }
    fn block_463<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_463_0: const #0u : u8
        let s_463_0: bool = false;
        // D s_463_1: write-var return_value <= s_463_0
        fn_state.return_value = s_463_0;
        // N s_463_2: jump b325
        return block_325(state, tracer, fn_state);
    }
    fn block_464<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_464_0: const #0u : u8
        let s_464_0: bool = false;
        // D s_464_1: write-var return_value <= s_464_0
        fn_state.return_value = s_464_0;
        // N s_464_2: jump b325
        return block_325(state, tracer, fn_state);
    }
    fn block_465<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_465_0: const #0u : u8
        let s_465_0: bool = false;
        // D s_465_1: write-var return_value <= s_465_0
        fn_state.return_value = s_465_0;
        // N s_465_2: jump b325
        return block_325(state, tracer, fn_state);
    }
    fn block_466<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_466_0: const #0u : u8
        let s_466_0: bool = false;
        // D s_466_1: write-var return_value <= s_466_0
        fn_state.return_value = s_466_0;
        // N s_466_2: jump b325
        return block_325(state, tracer, fn_state);
    }
    fn block_467<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_467_0: const #0u : u8
        let s_467_0: bool = false;
        // D s_467_1: write-var return_value <= s_467_0
        fn_state.return_value = s_467_0;
        // N s_467_2: jump b325
        return block_325(state, tracer, fn_state);
    }
    fn block_468<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_468_0: const #1u : u8
        let s_468_0: bool = true;
        // D s_468_1: write-var return_value <= s_468_0
        fn_state.return_value = s_468_0;
        // N s_468_2: jump b325
        return block_325(state, tracer, fn_state);
    }
    fn block_469<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_469_0: const #1u : u8
        let s_469_0: bool = true;
        // D s_469_1: write-var return_value <= s_469_0
        fn_state.return_value = s_469_0;
        // N s_469_2: jump b325
        return block_325(state, tracer, fn_state);
    }
    fn block_470<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_470_0: const #0u : u8
        let s_470_0: bool = false;
        // D s_470_1: write-var return_value <= s_470_0
        fn_state.return_value = s_470_0;
        // N s_470_2: jump b325
        return block_325(state, tracer, fn_state);
    }
    fn block_471<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_471_0: const #0u : u8
        let s_471_0: bool = false;
        // D s_471_1: write-var return_value <= s_471_0
        fn_state.return_value = s_471_0;
        // N s_471_2: jump b325
        return block_325(state, tracer, fn_state);
    }
    fn block_472<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_472_0: const #0u : u8
        let s_472_0: bool = false;
        // D s_472_1: write-var return_value <= s_472_0
        fn_state.return_value = s_472_0;
        // N s_472_2: jump b325
        return block_325(state, tracer, fn_state);
    }
    fn block_473<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_473_0: const #1u : u8
        let s_473_0: bool = true;
        // D s_473_1: write-var return_value <= s_473_0
        fn_state.return_value = s_473_0;
        // N s_473_2: jump b325
        return block_325(state, tracer, fn_state);
    }
    fn block_474<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_474_0: const #23360u : u32
        let s_474_0: u32 = 23360;
        // D s_474_1: read-reg s_474_0:u8
        let s_474_1: bool = {
            let value = state.read_register::<bool>(s_474_0 as isize);
            tracer.read_register(s_474_0 as isize, value);
            value
        };
        // D s_474_2: write-var return_value <= s_474_1
        fn_state.return_value = s_474_1;
        // N s_474_3: jump b325
        return block_325(state, tracer, fn_state);
    }
    fn block_475<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_475_0: const #101016u : u32
        let s_475_0: u32 = 101016;
        // D s_475_1: read-reg s_475_0:u8
        let s_475_1: bool = {
            let value = state.read_register::<bool>(s_475_0 as isize);
            tracer.read_register(s_475_0 as isize, value);
            value
        };
        // D s_475_2: write-var return_value <= s_475_1
        fn_state.return_value = s_475_1;
        // N s_475_3: jump b325
        return block_325(state, tracer, fn_state);
    }
    fn block_476<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_476_0: const #14352u : u32
        let s_476_0: u32 = 14352;
        // D s_476_1: read-reg s_476_0:u8
        let s_476_1: bool = {
            let value = state.read_register::<bool>(s_476_0 as isize);
            tracer.read_register(s_476_0 as isize, value);
            value
        };
        // D s_476_2: write-var return_value <= s_476_1
        fn_state.return_value = s_476_1;
        // N s_476_3: jump b325
        return block_325(state, tracer, fn_state);
    }
    fn block_477<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_477_0: const #101840u : u32
        let s_477_0: u32 = 101840;
        // D s_477_1: read-reg s_477_0:u8
        let s_477_1: bool = {
            let value = state.read_register::<bool>(s_477_0 as isize);
            tracer.read_register(s_477_0 as isize, value);
            value
        };
        // D s_477_2: write-var return_value <= s_477_1
        fn_state.return_value = s_477_1;
        // N s_477_3: jump b325
        return block_325(state, tracer, fn_state);
    }
    fn block_478<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_478_0: const #1u : u8
        let s_478_0: bool = true;
        // D s_478_1: write-var return_value <= s_478_0
        fn_state.return_value = s_478_0;
        // N s_478_2: jump b325
        return block_325(state, tracer, fn_state);
    }
    fn block_479<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_479_0: const #17368u : u32
        let s_479_0: u32 = 17368;
        // D s_479_1: read-reg s_479_0:u8
        let s_479_1: bool = {
            let value = state.read_register::<bool>(s_479_0 as isize);
            tracer.read_register(s_479_0 as isize, value);
            value
        };
        // D s_479_2: write-var return_value <= s_479_1
        fn_state.return_value = s_479_1;
        // N s_479_3: jump b325
        return block_325(state, tracer, fn_state);
    }
    fn block_480<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_480_0: const #23032u : u32
        let s_480_0: u32 = 23032;
        // D s_480_1: read-reg s_480_0:u8
        let s_480_1: bool = {
            let value = state.read_register::<bool>(s_480_0 as isize);
            tracer.read_register(s_480_0 as isize, value);
            value
        };
        // D s_480_2: write-var return_value <= s_480_1
        fn_state.return_value = s_480_1;
        // N s_480_3: jump b325
        return block_325(state, tracer, fn_state);
    }
    fn block_481<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_481_0: const #104504u : u32
        let s_481_0: u32 = 104504;
        // D s_481_1: read-reg s_481_0:u8
        let s_481_1: bool = {
            let value = state.read_register::<bool>(s_481_0 as isize);
            tracer.read_register(s_481_0 as isize, value);
            value
        };
        // D s_481_2: write-var return_value <= s_481_1
        fn_state.return_value = s_481_1;
        // N s_481_3: jump b325
        return block_325(state, tracer, fn_state);
    }
    fn block_482<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_482_0: const #0u : u8
        let s_482_0: bool = false;
        // D s_482_1: write-var return_value <= s_482_0
        fn_state.return_value = s_482_0;
        // N s_482_2: jump b325
        return block_325(state, tracer, fn_state);
    }
    fn block_483<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_483_0: const #12800u : u32
        let s_483_0: u32 = 12800;
        // D s_483_1: read-reg s_483_0:u8
        let s_483_1: bool = {
            let value = state.read_register::<bool>(s_483_0 as isize);
            tracer.read_register(s_483_0 as isize, value);
            value
        };
        // D s_483_2: write-var return_value <= s_483_1
        fn_state.return_value = s_483_1;
        // N s_483_3: jump b325
        return block_325(state, tracer, fn_state);
    }
    fn block_484<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_484_0: const #1u : u8
        let s_484_0: bool = true;
        // D s_484_1: write-var return_value <= s_484_0
        fn_state.return_value = s_484_0;
        // N s_484_2: jump b325
        return block_325(state, tracer, fn_state);
    }
    fn block_485<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_485_0: const #1u : u8
        let s_485_0: bool = true;
        // D s_485_1: write-var return_value <= s_485_0
        fn_state.return_value = s_485_0;
        // N s_485_2: jump b325
        return block_325(state, tracer, fn_state);
    }
    fn block_486<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_486_0: const #1u : u8
        let s_486_0: bool = true;
        // D s_486_1: write-var return_value <= s_486_0
        fn_state.return_value = s_486_0;
        // N s_486_2: jump b325
        return block_325(state, tracer, fn_state);
    }
    fn block_487<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_487_0: const #1u : u8
        let s_487_0: bool = true;
        // D s_487_1: write-var return_value <= s_487_0
        fn_state.return_value = s_487_0;
        // N s_487_2: jump b325
        return block_325(state, tracer, fn_state);
    }
    fn block_488<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_488_0: const #1u : u8
        let s_488_0: bool = true;
        // D s_488_1: write-var return_value <= s_488_0
        fn_state.return_value = s_488_0;
        // N s_488_2: jump b325
        return block_325(state, tracer, fn_state);
    }
    fn block_489<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_489_0: const #1u : u8
        let s_489_0: bool = true;
        // D s_489_1: write-var return_value <= s_489_0
        fn_state.return_value = s_489_0;
        // N s_489_2: jump b325
        return block_325(state, tracer, fn_state);
    }
    fn block_490<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_490_0: const #12032u : u32
        let s_490_0: u32 = 12032;
        // D s_490_1: read-reg s_490_0:u8
        let s_490_1: bool = {
            let value = state.read_register::<bool>(s_490_0 as isize);
            tracer.read_register(s_490_0 as isize, value);
            value
        };
        // D s_490_2: write-var return_value <= s_490_1
        fn_state.return_value = s_490_1;
        // N s_490_3: jump b325
        return block_325(state, tracer, fn_state);
    }
    fn block_491<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_491_0: const #16312u : u32
        let s_491_0: u32 = 16312;
        // D s_491_1: read-reg s_491_0:u8
        let s_491_1: bool = {
            let value = state.read_register::<bool>(s_491_0 as isize);
            tracer.read_register(s_491_0 as isize, value);
            value
        };
        // D s_491_2: write-var return_value <= s_491_1
        fn_state.return_value = s_491_1;
        // N s_491_3: jump b325
        return block_325(state, tracer, fn_state);
    }
    fn block_492<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_492_0: const #100904u : u32
        let s_492_0: u32 = 100904;
        // D s_492_1: read-reg s_492_0:u8
        let s_492_1: bool = {
            let value = state.read_register::<bool>(s_492_0 as isize);
            tracer.read_register(s_492_0 as isize, value);
            value
        };
        // D s_492_2: write-var return_value <= s_492_1
        fn_state.return_value = s_492_1;
        // N s_492_3: jump b325
        return block_325(state, tracer, fn_state);
    }
    fn block_493<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_493_0: const #19024u : u32
        let s_493_0: u32 = 19024;
        // D s_493_1: read-reg s_493_0:u8
        let s_493_1: bool = {
            let value = state.read_register::<bool>(s_493_0 as isize);
            tracer.read_register(s_493_0 as isize, value);
            value
        };
        // D s_493_2: write-var return_value <= s_493_1
        fn_state.return_value = s_493_1;
        // N s_493_3: jump b325
        return block_325(state, tracer, fn_state);
    }
    fn block_494<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_494_0: const #11576u : u32
        let s_494_0: u32 = 11576;
        // D s_494_1: read-reg s_494_0:u8
        let s_494_1: bool = {
            let value = state.read_register::<bool>(s_494_0 as isize);
            tracer.read_register(s_494_0 as isize, value);
            value
        };
        // D s_494_2: write-var return_value <= s_494_1
        fn_state.return_value = s_494_1;
        // N s_494_3: jump b325
        return block_325(state, tracer, fn_state);
    }
    fn block_495<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_495_0: const #23168u : u32
        let s_495_0: u32 = 23168;
        // D s_495_1: read-reg s_495_0:u8
        let s_495_1: bool = {
            let value = state.read_register::<bool>(s_495_0 as isize);
            tracer.read_register(s_495_0 as isize, value);
            value
        };
        // D s_495_2: write-var return_value <= s_495_1
        fn_state.return_value = s_495_1;
        // N s_495_3: jump b325
        return block_325(state, tracer, fn_state);
    }
    fn block_496<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_496_0: const #104712u : u32
        let s_496_0: u32 = 104712;
        // D s_496_1: read-reg s_496_0:u8
        let s_496_1: bool = {
            let value = state.read_register::<bool>(s_496_0 as isize);
            tracer.read_register(s_496_0 as isize, value);
            value
        };
        // D s_496_2: write-var return_value <= s_496_1
        fn_state.return_value = s_496_1;
        // N s_496_3: jump b325
        return block_325(state, tracer, fn_state);
    }
    fn block_497<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_497_0: const #20960u : u32
        let s_497_0: u32 = 20960;
        // D s_497_1: read-reg s_497_0:u8
        let s_497_1: bool = {
            let value = state.read_register::<bool>(s_497_0 as isize);
            tracer.read_register(s_497_0 as isize, value);
            value
        };
        // D s_497_2: write-var return_value <= s_497_1
        fn_state.return_value = s_497_1;
        // N s_497_3: jump b325
        return block_325(state, tracer, fn_state);
    }
    fn block_498<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_498_0: const #15592u : u32
        let s_498_0: u32 = 15592;
        // D s_498_1: read-reg s_498_0:u8
        let s_498_1: bool = {
            let value = state.read_register::<bool>(s_498_0 as isize);
            tracer.read_register(s_498_0 as isize, value);
            value
        };
        // D s_498_2: write-var return_value <= s_498_1
        fn_state.return_value = s_498_1;
        // N s_498_3: jump b325
        return block_325(state, tracer, fn_state);
    }
    fn block_499<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_499_0: const #19960u : u32
        let s_499_0: u32 = 19960;
        // D s_499_1: read-reg s_499_0:u8
        let s_499_1: bool = {
            let value = state.read_register::<bool>(s_499_0 as isize);
            tracer.read_register(s_499_0 as isize, value);
            value
        };
        // D s_499_2: write-var return_value <= s_499_1
        fn_state.return_value = s_499_1;
        // N s_499_3: jump b325
        return block_325(state, tracer, fn_state);
    }
    fn block_500<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_500_0: const #102592u : u32
        let s_500_0: u32 = 102592;
        // D s_500_1: read-reg s_500_0:u8
        let s_500_1: bool = {
            let value = state.read_register::<bool>(s_500_0 as isize);
            tracer.read_register(s_500_0 as isize, value);
            value
        };
        // D s_500_2: write-var return_value <= s_500_1
        fn_state.return_value = s_500_1;
        // N s_500_3: jump b325
        return block_325(state, tracer, fn_state);
    }
    fn block_501<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_501_0: const #14456u : u32
        let s_501_0: u32 = 14456;
        // D s_501_1: read-reg s_501_0:u8
        let s_501_1: bool = {
            let value = state.read_register::<bool>(s_501_0 as isize);
            tracer.read_register(s_501_0 as isize, value);
            value
        };
        // D s_501_2: write-var return_value <= s_501_1
        fn_state.return_value = s_501_1;
        // N s_501_3: jump b325
        return block_325(state, tracer, fn_state);
    }
    fn block_502<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_502_0: const #102592u : u32
        let s_502_0: u32 = 102592;
        // D s_502_1: read-reg s_502_0:u8
        let s_502_1: bool = {
            let value = state.read_register::<bool>(s_502_0 as isize);
            tracer.read_register(s_502_0 as isize, value);
            value
        };
        // D s_502_2: write-var return_value <= s_502_1
        fn_state.return_value = s_502_1;
        // N s_502_3: jump b325
        return block_325(state, tracer, fn_state);
    }
    fn block_503<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_503_0: const #103976u : u32
        let s_503_0: u32 = 103976;
        // D s_503_1: read-reg s_503_0:u8
        let s_503_1: bool = {
            let value = state.read_register::<bool>(s_503_0 as isize);
            tracer.read_register(s_503_0 as isize, value);
            value
        };
        // D s_503_2: write-var return_value <= s_503_1
        fn_state.return_value = s_503_1;
        // N s_503_3: jump b325
        return block_325(state, tracer, fn_state);
    }
    fn block_504<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_504_0: const #0u : u8
        let s_504_0: bool = false;
        // D s_504_1: write-var return_value <= s_504_0
        fn_state.return_value = s_504_0;
        // N s_504_2: jump b325
        return block_325(state, tracer, fn_state);
    }
    fn block_505<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_505_0: const #0u : u8
        let s_505_0: bool = false;
        // D s_505_1: write-var return_value <= s_505_0
        fn_state.return_value = s_505_0;
        // N s_505_2: jump b325
        return block_325(state, tracer, fn_state);
    }
    fn block_506<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_506_0: const #1u : u8
        let s_506_0: bool = true;
        // D s_506_1: write-var return_value <= s_506_0
        fn_state.return_value = s_506_0;
        // N s_506_2: jump b325
        return block_325(state, tracer, fn_state);
    }
    fn block_507<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_507_0: const #1u : u8
        let s_507_0: bool = true;
        // D s_507_1: write-var return_value <= s_507_0
        fn_state.return_value = s_507_0;
        // N s_507_2: jump b325
        return block_325(state, tracer, fn_state);
    }
    fn block_508<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_508_0: const #1u : u8
        let s_508_0: bool = true;
        // D s_508_1: write-var return_value <= s_508_0
        fn_state.return_value = s_508_0;
        // N s_508_2: jump b325
        return block_325(state, tracer, fn_state);
    }
    fn block_509<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_509_0: const #1u : u8
        let s_509_0: bool = true;
        // D s_509_1: write-var return_value <= s_509_0
        fn_state.return_value = s_509_0;
        // N s_509_2: jump b325
        return block_325(state, tracer, fn_state);
    }
    fn block_510<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_510_0: const #1u : u8
        let s_510_0: bool = true;
        // D s_510_1: write-var return_value <= s_510_0
        fn_state.return_value = s_510_0;
        // N s_510_2: jump b325
        return block_325(state, tracer, fn_state);
    }
    fn block_511<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_511_0: const #1u : u8
        let s_511_0: bool = true;
        // D s_511_1: write-var return_value <= s_511_0
        fn_state.return_value = s_511_0;
        // N s_511_2: jump b325
        return block_325(state, tracer, fn_state);
    }
    fn block_512<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_512_0: const #12128u : u32
        let s_512_0: u32 = 12128;
        // D s_512_1: read-reg s_512_0:u8
        let s_512_1: bool = {
            let value = state.read_register::<bool>(s_512_0 as isize);
            tracer.read_register(s_512_0 as isize, value);
            value
        };
        // D s_512_2: write-var return_value <= s_512_1
        fn_state.return_value = s_512_1;
        // N s_512_3: jump b325
        return block_325(state, tracer, fn_state);
    }
    fn block_513<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_513_0: const #22064u : u32
        let s_513_0: u32 = 22064;
        // D s_513_1: read-reg s_513_0:u8
        let s_513_1: bool = {
            let value = state.read_register::<bool>(s_513_0 as isize);
            tracer.read_register(s_513_0 as isize, value);
            value
        };
        // D s_513_2: write-var return_value <= s_513_1
        fn_state.return_value = s_513_1;
        // N s_513_3: jump b325
        return block_325(state, tracer, fn_state);
    }
    fn block_514<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_514_0: const #104616u : u32
        let s_514_0: u32 = 104616;
        // D s_514_1: read-reg s_514_0:u8
        let s_514_1: bool = {
            let value = state.read_register::<bool>(s_514_0 as isize);
            tracer.read_register(s_514_0 as isize, value);
            value
        };
        // D s_514_2: write-var return_value <= s_514_1
        fn_state.return_value = s_514_1;
        // N s_514_3: jump b325
        return block_325(state, tracer, fn_state);
    }
    fn block_515<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_515_0: const #1u : u8
        let s_515_0: bool = true;
        // D s_515_1: write-var return_value <= s_515_0
        fn_state.return_value = s_515_0;
        // N s_515_2: jump b325
        return block_325(state, tracer, fn_state);
    }
    fn block_516<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_516_0: const #1u : u8
        let s_516_0: bool = true;
        // D s_516_1: write-var return_value <= s_516_0
        fn_state.return_value = s_516_0;
        // N s_516_2: jump b325
        return block_325(state, tracer, fn_state);
    }
    fn block_517<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_517_0: const #1u : u8
        let s_517_0: bool = true;
        // D s_517_1: write-var return_value <= s_517_0
        fn_state.return_value = s_517_0;
        // N s_517_2: jump b325
        return block_325(state, tracer, fn_state);
    }
    fn block_518<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_518_0: const #1u : u8
        let s_518_0: bool = true;
        // D s_518_1: write-var return_value <= s_518_0
        fn_state.return_value = s_518_0;
        // N s_518_2: jump b325
        return block_325(state, tracer, fn_state);
    }
    fn block_519<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_519_0: const #1u : u8
        let s_519_0: bool = true;
        // D s_519_1: write-var return_value <= s_519_0
        fn_state.return_value = s_519_0;
        // N s_519_2: jump b325
        return block_325(state, tracer, fn_state);
    }
    fn block_520<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_520_0: const #1u : u8
        let s_520_0: bool = true;
        // D s_520_1: write-var return_value <= s_520_0
        fn_state.return_value = s_520_0;
        // N s_520_2: jump b325
        return block_325(state, tracer, fn_state);
    }
    fn block_521<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_521_0: const #1u : u8
        let s_521_0: bool = true;
        // D s_521_1: write-var return_value <= s_521_0
        fn_state.return_value = s_521_0;
        // N s_521_2: jump b325
        return block_325(state, tracer, fn_state);
    }
    fn block_522<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_522_0: const #1u : u8
        let s_522_0: bool = true;
        // D s_522_1: write-var return_value <= s_522_0
        fn_state.return_value = s_522_0;
        // N s_522_2: jump b325
        return block_325(state, tracer, fn_state);
    }
    fn block_523<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_523_0: const #1u : u8
        let s_523_0: bool = true;
        // D s_523_1: write-var return_value <= s_523_0
        fn_state.return_value = s_523_0;
        // N s_523_2: jump b325
        return block_325(state, tracer, fn_state);
    }
    fn block_524<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_524_0: const #0u : u8
        let s_524_0: bool = false;
        // D s_524_1: write-var return_value <= s_524_0
        fn_state.return_value = s_524_0;
        // N s_524_2: jump b325
        return block_325(state, tracer, fn_state);
    }
    fn block_525<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_525_0: const #1u : u8
        let s_525_0: bool = true;
        // D s_525_1: write-var return_value <= s_525_0
        fn_state.return_value = s_525_0;
        // N s_525_2: jump b325
        return block_325(state, tracer, fn_state);
    }
    fn block_526<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_526_0: const #1u : u8
        let s_526_0: bool = true;
        // D s_526_1: write-var return_value <= s_526_0
        fn_state.return_value = s_526_0;
        // N s_526_2: jump b325
        return block_325(state, tracer, fn_state);
    }
    fn block_527<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_527_0: const #1u : u8
        let s_527_0: bool = true;
        // D s_527_1: write-var return_value <= s_527_0
        fn_state.return_value = s_527_0;
        // N s_527_2: jump b325
        return block_325(state, tracer, fn_state);
    }
    fn block_528<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_528_0: const #1u : u8
        let s_528_0: bool = true;
        // D s_528_1: write-var return_value <= s_528_0
        fn_state.return_value = s_528_0;
        // N s_528_2: jump b325
        return block_325(state, tracer, fn_state);
    }
    fn block_529<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_529_0: const #1u : u8
        let s_529_0: bool = true;
        // D s_529_1: write-var return_value <= s_529_0
        fn_state.return_value = s_529_0;
        // N s_529_2: jump b325
        return block_325(state, tracer, fn_state);
    }
    fn block_530<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_530_0: const #1u : u8
        let s_530_0: bool = true;
        // D s_530_1: write-var return_value <= s_530_0
        fn_state.return_value = s_530_0;
        // N s_530_2: jump b325
        return block_325(state, tracer, fn_state);
    }
    fn block_531<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_531_0: const #1u : u8
        let s_531_0: bool = true;
        // D s_531_1: write-var return_value <= s_531_0
        fn_state.return_value = s_531_0;
        // N s_531_2: jump b325
        return block_325(state, tracer, fn_state);
    }
    fn block_532<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_532_0: const #1u : u8
        let s_532_0: bool = true;
        // D s_532_1: write-var return_value <= s_532_0
        fn_state.return_value = s_532_0;
        // N s_532_2: jump b325
        return block_325(state, tracer, fn_state);
    }
    fn block_533<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_533_0: const #0u : u8
        let s_533_0: bool = false;
        // D s_533_1: write-var return_value <= s_533_0
        fn_state.return_value = s_533_0;
        // N s_533_2: jump b325
        return block_325(state, tracer, fn_state);
    }
    fn block_534<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_534_0: const #12152u : u32
        let s_534_0: u32 = 12152;
        // D s_534_1: read-reg s_534_0:u8
        let s_534_1: bool = {
            let value = state.read_register::<bool>(s_534_0 as isize);
            tracer.read_register(s_534_0 as isize, value);
            value
        };
        // D s_534_2: write-var return_value <= s_534_1
        fn_state.return_value = s_534_1;
        // N s_534_3: jump b325
        return block_325(state, tracer, fn_state);
    }
    fn block_535<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_535_0: const #14400u : u32
        let s_535_0: u32 = 14400;
        // D s_535_1: read-reg s_535_0:u8
        let s_535_1: bool = {
            let value = state.read_register::<bool>(s_535_0 as isize);
            tracer.read_register(s_535_0 as isize, value);
            value
        };
        // D s_535_2: write-var return_value <= s_535_1
        fn_state.return_value = s_535_1;
        // N s_535_3: jump b325
        return block_325(state, tracer, fn_state);
    }
    fn block_536<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_536_0: const #13584u : u32
        let s_536_0: u32 = 13584;
        // D s_536_1: read-reg s_536_0:u8
        let s_536_1: bool = {
            let value = state.read_register::<bool>(s_536_0 as isize);
            tracer.read_register(s_536_0 as isize, value);
            value
        };
        // D s_536_2: write-var return_value <= s_536_1
        fn_state.return_value = s_536_1;
        // N s_536_3: jump b325
        return block_325(state, tracer, fn_state);
    }
    fn block_537<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_537_0: const #22000u : u32
        let s_537_0: u32 = 22000;
        // D s_537_1: read-reg s_537_0:u8
        let s_537_1: bool = {
            let value = state.read_register::<bool>(s_537_0 as isize);
            tracer.read_register(s_537_0 as isize, value);
            value
        };
        // D s_537_2: write-var return_value <= s_537_1
        fn_state.return_value = s_537_1;
        // N s_537_3: jump b325
        return block_325(state, tracer, fn_state);
    }
    fn block_538<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_538_0: const #1u : u8
        let s_538_0: bool = true;
        // D s_538_1: write-var return_value <= s_538_0
        fn_state.return_value = s_538_0;
        // N s_538_2: jump b325
        return block_325(state, tracer, fn_state);
    }
    fn block_539<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_539_0: const #1u : u8
        let s_539_0: bool = true;
        // D s_539_1: write-var return_value <= s_539_0
        fn_state.return_value = s_539_0;
        // N s_539_2: jump b325
        return block_325(state, tracer, fn_state);
    }
    fn block_540<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_540_0: const #1u : u8
        let s_540_0: bool = true;
        // D s_540_1: write-var return_value <= s_540_0
        fn_state.return_value = s_540_0;
        // N s_540_2: jump b325
        return block_325(state, tracer, fn_state);
    }
    fn block_541<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_541_0: const #1u : u8
        let s_541_0: bool = true;
        // D s_541_1: write-var return_value <= s_541_0
        fn_state.return_value = s_541_0;
        // N s_541_2: jump b325
        return block_325(state, tracer, fn_state);
    }
    fn block_542<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_542_0: const #1u : u8
        let s_542_0: bool = true;
        // D s_542_1: write-var return_value <= s_542_0
        fn_state.return_value = s_542_0;
        // N s_542_2: jump b325
        return block_325(state, tracer, fn_state);
    }
    fn block_543<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_543_0: const #1u : u8
        let s_543_0: bool = true;
        // D s_543_1: write-var return_value <= s_543_0
        fn_state.return_value = s_543_0;
        // N s_543_2: jump b325
        return block_325(state, tracer, fn_state);
    }
    fn block_544<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_544_0: const #1u : u8
        let s_544_0: bool = true;
        // D s_544_1: write-var return_value <= s_544_0
        fn_state.return_value = s_544_0;
        // N s_544_2: jump b325
        return block_325(state, tracer, fn_state);
    }
    fn block_545<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_545_0: const #1u : u8
        let s_545_0: bool = true;
        // D s_545_1: write-var return_value <= s_545_0
        fn_state.return_value = s_545_0;
        // N s_545_2: jump b325
        return block_325(state, tracer, fn_state);
    }
    fn block_546<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_546_0: const #1u : u8
        let s_546_0: bool = true;
        // D s_546_1: write-var return_value <= s_546_0
        fn_state.return_value = s_546_0;
        // N s_546_2: jump b325
        return block_325(state, tracer, fn_state);
    }
    fn block_547<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_547_0: const #1u : u8
        let s_547_0: bool = true;
        // D s_547_1: write-var return_value <= s_547_0
        fn_state.return_value = s_547_0;
        // N s_547_2: jump b325
        return block_325(state, tracer, fn_state);
    }
    fn block_548<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_548_0: const #1u : u8
        let s_548_0: bool = true;
        // D s_548_1: write-var return_value <= s_548_0
        fn_state.return_value = s_548_0;
        // N s_548_2: jump b325
        return block_325(state, tracer, fn_state);
    }
    fn block_549<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_549_0: const #1u : u8
        let s_549_0: bool = true;
        // D s_549_1: write-var return_value <= s_549_0
        fn_state.return_value = s_549_0;
        // N s_549_2: jump b325
        return block_325(state, tracer, fn_state);
    }
    fn block_550<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_550_0: const #1u : u8
        let s_550_0: bool = true;
        // D s_550_1: write-var return_value <= s_550_0
        fn_state.return_value = s_550_0;
        // N s_550_2: jump b325
        return block_325(state, tracer, fn_state);
    }
    fn block_551<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_551_0: const #0u : u8
        let s_551_0: bool = false;
        // D s_551_1: write-var return_value <= s_551_0
        fn_state.return_value = s_551_0;
        // N s_551_2: jump b325
        return block_325(state, tracer, fn_state);
    }
    fn block_552<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_552_0: const #0u : u8
        let s_552_0: bool = false;
        // D s_552_1: write-var return_value <= s_552_0
        fn_state.return_value = s_552_0;
        // N s_552_2: jump b325
        return block_325(state, tracer, fn_state);
    }
    fn block_553<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_553_0: const #1u : u8
        let s_553_0: bool = true;
        // D s_553_1: write-var return_value <= s_553_0
        fn_state.return_value = s_553_0;
        // N s_553_2: jump b325
        return block_325(state, tracer, fn_state);
    }
    fn block_554<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_554_0: const #1u : u8
        let s_554_0: bool = true;
        // D s_554_1: write-var return_value <= s_554_0
        fn_state.return_value = s_554_0;
        // N s_554_2: jump b325
        return block_325(state, tracer, fn_state);
    }
    fn block_555<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_555_0: const #14432u : u32
        let s_555_0: u32 = 14432;
        // D s_555_1: read-reg s_555_0:u8
        let s_555_1: bool = {
            let value = state.read_register::<bool>(s_555_0 as isize);
            tracer.read_register(s_555_0 as isize, value);
            value
        };
        // D s_555_2: write-var return_value <= s_555_1
        fn_state.return_value = s_555_1;
        // N s_555_3: jump b325
        return block_325(state, tracer, fn_state);
    }
    fn block_556<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_556_0: const #1u : u8
        let s_556_0: bool = true;
        // D s_556_1: write-var return_value <= s_556_0
        fn_state.return_value = s_556_0;
        // N s_556_2: jump b325
        return block_325(state, tracer, fn_state);
    }
    fn block_557<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_557_0: const #1u : u8
        let s_557_0: bool = true;
        // D s_557_1: write-var return_value <= s_557_0
        fn_state.return_value = s_557_0;
        // N s_557_2: jump b325
        return block_325(state, tracer, fn_state);
    }
    fn block_558<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_558_0: const #1u : u8
        let s_558_0: bool = true;
        // D s_558_1: write-var return_value <= s_558_0
        fn_state.return_value = s_558_0;
        // N s_558_2: jump b325
        return block_325(state, tracer, fn_state);
    }
    fn block_559<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_559_0: const #1u : u8
        let s_559_0: bool = true;
        // D s_559_1: write-var return_value <= s_559_0
        fn_state.return_value = s_559_0;
        // N s_559_2: jump b325
        return block_325(state, tracer, fn_state);
    }
    fn block_560<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_560_0: const #90840u : u32
        let s_560_0: u32 = 90840;
        // D s_560_1: read-reg s_560_0:u8
        let s_560_1: bool = {
            let value = state.read_register::<bool>(s_560_0 as isize);
            tracer.read_register(s_560_0 as isize, value);
            value
        };
        // D s_560_2: write-var return_value <= s_560_1
        fn_state.return_value = s_560_1;
        // N s_560_3: jump b325
        return block_325(state, tracer, fn_state);
    }
    fn block_561<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_561_0: const #14464u : u32
        let s_561_0: u32 = 14464;
        // D s_561_1: read-reg s_561_0:u8
        let s_561_1: bool = {
            let value = state.read_register::<bool>(s_561_0 as isize);
            tracer.read_register(s_561_0 as isize, value);
            value
        };
        // D s_561_2: write-var return_value <= s_561_1
        fn_state.return_value = s_561_1;
        // N s_561_3: jump b325
        return block_325(state, tracer, fn_state);
    }
    fn block_562<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_562_0: const #23032u : u32
        let s_562_0: u32 = 23032;
        // D s_562_1: read-reg s_562_0:u8
        let s_562_1: bool = {
            let value = state.read_register::<bool>(s_562_0 as isize);
            tracer.read_register(s_562_0 as isize, value);
            value
        };
        // D s_562_2: write-var return_value <= s_562_1
        fn_state.return_value = s_562_1;
        // N s_562_3: jump b325
        return block_325(state, tracer, fn_state);
    }
    fn block_563<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_563_0: const #91000u : u32
        let s_563_0: u32 = 91000;
        // D s_563_1: read-reg s_563_0:u8
        let s_563_1: u8 = {
            let value = state.read_register::<u8>(s_563_0 as isize);
            tracer.read_register(s_563_0 as isize, value);
            value
        };
        // D s_563_2: cast zx s_563_1 -> bv
        let s_563_2: Bits = Bits::new(s_563_1 as u128, 4u16);
        // D s_563_3: cast zx s_563_2 -> i
        let s_563_3: i128 = (s_563_2.value() as i128);
        // D s_563_4: cast reint s_563_3 -> i64
        let s_563_4: i64 = (s_563_3 as i64);
        // C s_563_5: const #2u : u8
        let s_563_5: u8 = 2;
        // C s_563_6: cast zx s_563_5 -> bv
        let s_563_6: Bits = Bits::new(s_563_5 as u128, 4u16);
        // C s_563_7: cast zx s_563_6 -> i
        let s_563_7: i128 = (s_563_6.value() as i128);
        // C s_563_8: cast reint s_563_7 -> i64
        let s_563_8: i64 = (s_563_7 as i64);
        // D s_563_9: cast zx s_563_4 -> i
        let s_563_9: i128 = (i128::try_from(s_563_4).unwrap());
        // C s_563_10: cast zx s_563_8 -> i
        let s_563_10: i128 = (i128::try_from(s_563_8).unwrap());
        // D s_563_11: cmp-eq s_563_9 s_563_10
        let s_563_11: bool = ((s_563_9) == (s_563_10));
        // D s_563_12: write-var return_value <= s_563_11
        fn_state.return_value = s_563_11;
        // N s_563_13: jump b325
        return block_325(state, tracer, fn_state);
    }
    fn block_564<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_564_0: const #91000u : u32
        let s_564_0: u32 = 91000;
        // D s_564_1: read-reg s_564_0:u8
        let s_564_1: u8 = {
            let value = state.read_register::<u8>(s_564_0 as isize);
            tracer.read_register(s_564_0 as isize, value);
            value
        };
        // D s_564_2: cast zx s_564_1 -> bv
        let s_564_2: Bits = Bits::new(s_564_1 as u128, 4u16);
        // D s_564_3: cast zx s_564_2 -> i
        let s_564_3: i128 = (s_564_2.value() as i128);
        // D s_564_4: cast reint s_564_3 -> i64
        let s_564_4: i64 = (s_564_3 as i64);
        // C s_564_5: const #1u : u8
        let s_564_5: u8 = 1;
        // C s_564_6: cast zx s_564_5 -> bv
        let s_564_6: Bits = Bits::new(s_564_5 as u128, 4u16);
        // C s_564_7: cast zx s_564_6 -> i
        let s_564_7: i128 = (s_564_6.value() as i128);
        // C s_564_8: cast reint s_564_7 -> i64
        let s_564_8: i64 = (s_564_7 as i64);
        // D s_564_9: cast zx s_564_4 -> i
        let s_564_9: i128 = (i128::try_from(s_564_4).unwrap());
        // C s_564_10: cast zx s_564_8 -> i
        let s_564_10: i128 = (i128::try_from(s_564_8).unwrap());
        // D s_564_11: cmp-eq s_564_9 s_564_10
        let s_564_11: bool = ((s_564_9) == (s_564_10));
        // D s_564_12: write-var return_value <= s_564_11
        fn_state.return_value = s_564_11;
        // N s_564_13: jump b325
        return block_325(state, tracer, fn_state);
    }
    fn block_565<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_565_0: const #1u : u8
        let s_565_0: bool = true;
        // D s_565_1: write-var return_value <= s_565_0
        fn_state.return_value = s_565_0;
        // N s_565_2: jump b325
        return block_325(state, tracer, fn_state);
    }
    fn block_566<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_566_0: const #1u : u8
        let s_566_0: bool = true;
        // D s_566_1: write-var return_value <= s_566_0
        fn_state.return_value = s_566_0;
        // N s_566_2: jump b325
        return block_325(state, tracer, fn_state);
    }
    fn block_567<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_567_0: const #1u : u8
        let s_567_0: bool = true;
        // D s_567_1: write-var return_value <= s_567_0
        fn_state.return_value = s_567_0;
        // N s_567_2: jump b325
        return block_325(state, tracer, fn_state);
    }
    fn block_568<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_568_0: const #23288u : u32
        let s_568_0: u32 = 23288;
        // D s_568_1: read-reg s_568_0:u8
        let s_568_1: bool = {
            let value = state.read_register::<bool>(s_568_0 as isize);
            tracer.read_register(s_568_0 as isize, value);
            value
        };
        // D s_568_2: write-var return_value <= s_568_1
        fn_state.return_value = s_568_1;
        // N s_568_3: jump b325
        return block_325(state, tracer, fn_state);
    }
    fn block_569<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_569_0: const #16336u : u32
        let s_569_0: u32 = 16336;
        // D s_569_1: read-reg s_569_0:u8
        let s_569_1: bool = {
            let value = state.read_register::<bool>(s_569_0 as isize);
            tracer.read_register(s_569_0 as isize, value);
            value
        };
        // D s_569_2: write-var return_value <= s_569_1
        fn_state.return_value = s_569_1;
        // N s_569_3: jump b325
        return block_325(state, tracer, fn_state);
    }
    fn block_570<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_570_0: const #15224u : u32
        let s_570_0: u32 = 15224;
        // D s_570_1: read-reg s_570_0:u8
        let s_570_1: bool = {
            let value = state.read_register::<bool>(s_570_0 as isize);
            tracer.read_register(s_570_0 as isize, value);
            value
        };
        // D s_570_2: write-var return_value <= s_570_1
        fn_state.return_value = s_570_1;
        // N s_570_3: jump b325
        return block_325(state, tracer, fn_state);
    }
    fn block_571<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_571_0: const #90400u : u32
        let s_571_0: u32 = 90400;
        // D s_571_1: read-reg s_571_0:u8
        let s_571_1: bool = {
            let value = state.read_register::<bool>(s_571_0 as isize);
            tracer.read_register(s_571_0 as isize, value);
            value
        };
        // D s_571_2: write-var return_value <= s_571_1
        fn_state.return_value = s_571_1;
        // N s_571_3: jump b325
        return block_325(state, tracer, fn_state);
    }
    fn block_572<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_572_0: const #91056u : u32
        let s_572_0: u32 = 91056;
        // D s_572_1: read-reg s_572_0:u8
        let s_572_1: bool = {
            let value = state.read_register::<bool>(s_572_0 as isize);
            tracer.read_register(s_572_0 as isize, value);
            value
        };
        // D s_572_2: write-var return_value <= s_572_1
        fn_state.return_value = s_572_1;
        // N s_572_3: jump b325
        return block_325(state, tracer, fn_state);
    }
    fn block_573<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_573_0: const #21056u : u32
        let s_573_0: u32 = 21056;
        // D s_573_1: read-reg s_573_0:u8
        let s_573_1: bool = {
            let value = state.read_register::<bool>(s_573_0 as isize);
            tracer.read_register(s_573_0 as isize, value);
            value
        };
        // D s_573_2: write-var return_value <= s_573_1
        fn_state.return_value = s_573_1;
        // N s_573_3: jump b325
        return block_325(state, tracer, fn_state);
    }
    fn block_574<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_574_0: const #90144u : u32
        let s_574_0: u32 = 90144;
        // D s_574_1: read-reg s_574_0:u8
        let s_574_1: u8 = {
            let value = state.read_register::<u8>(s_574_0 as isize);
            tracer.read_register(s_574_0 as isize, value);
            value
        };
        // D s_574_2: cast zx s_574_1 -> bv
        let s_574_2: Bits = Bits::new(s_574_1 as u128, 4u16);
        // C s_574_3: const #1u : u8
        let s_574_3: u8 = 1;
        // C s_574_4: cast zx s_574_3 -> bv
        let s_574_4: Bits = Bits::new(s_574_3 as u128, 4u16);
        // D s_574_5: cmp-eq s_574_2 s_574_4
        let s_574_5: bool = ((s_574_2) == (s_574_4));
        // N s_574_6: branch s_574_5 b577 b575
        if s_574_5 {
            return block_577(state, tracer, fn_state);
        } else {
            return block_575(state, tracer, fn_state);
        };
    }
    fn block_575<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_575_0: const #0u : u8
        let s_575_0: bool = false;
        // D s_575_1: write-var gs#331367 <= s_575_0
        fn_state.gs_331367 = s_575_0;
        // N s_575_2: jump b576
        return block_576(state, tracer, fn_state);
    }
    fn block_576<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_576_0: read-var gs#331367:u8
        let s_576_0: bool = fn_state.gs_331367;
        // D s_576_1: write-var return_value <= s_576_0
        fn_state.return_value = s_576_0;
        // N s_576_2: jump b325
        return block_325(state, tracer, fn_state);
    }
    fn block_577<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_577_0: const #14256u : u32
        let s_577_0: u32 = 14256;
        // D s_577_1: read-reg s_577_0:u8
        let s_577_1: u8 = {
            let value = state.read_register::<u8>(s_577_0 as isize);
            tracer.read_register(s_577_0 as isize, value);
            value
        };
        // D s_577_2: cast zx s_577_1 -> bv
        let s_577_2: Bits = Bits::new(s_577_1 as u128, 4u16);
        // C s_577_3: const #1u : u8
        let s_577_3: u8 = 1;
        // C s_577_4: cast zx s_577_3 -> bv
        let s_577_4: Bits = Bits::new(s_577_3 as u128, 4u16);
        // D s_577_5: cmp-eq s_577_2 s_577_4
        let s_577_5: bool = ((s_577_2) == (s_577_4));
        // D s_577_6: write-var gs#331367 <= s_577_5
        fn_state.gs_331367 = s_577_5;
        // N s_577_7: jump b576
        return block_576(state, tracer, fn_state);
    }
    fn block_578<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_578_0: const #90144u : u32
        let s_578_0: u32 = 90144;
        // D s_578_1: read-reg s_578_0:u8
        let s_578_1: u8 = {
            let value = state.read_register::<u8>(s_578_0 as isize);
            tracer.read_register(s_578_0 as isize, value);
            value
        };
        // D s_578_2: cast zx s_578_1 -> bv
        let s_578_2: Bits = Bits::new(s_578_1 as u128, 4u16);
        // C s_578_3: const #0u : u8
        let s_578_3: u8 = 0;
        // C s_578_4: cast zx s_578_3 -> bv
        let s_578_4: Bits = Bits::new(s_578_3 as u128, 4u16);
        // D s_578_5: cmp-eq s_578_2 s_578_4
        let s_578_5: bool = ((s_578_2) == (s_578_4));
        // N s_578_6: branch s_578_5 b581 b579
        if s_578_5 {
            return block_581(state, tracer, fn_state);
        } else {
            return block_579(state, tracer, fn_state);
        };
    }
    fn block_579<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_579_0: const #0u : u8
        let s_579_0: bool = false;
        // D s_579_1: write-var gs#331368 <= s_579_0
        fn_state.gs_331368 = s_579_0;
        // N s_579_2: jump b580
        return block_580(state, tracer, fn_state);
    }
    fn block_580<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_580_0: read-var gs#331368:u8
        let s_580_0: bool = fn_state.gs_331368;
        // D s_580_1: write-var return_value <= s_580_0
        fn_state.return_value = s_580_0;
        // N s_580_2: jump b325
        return block_325(state, tracer, fn_state);
    }
    fn block_581<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_581_0: const #14256u : u32
        let s_581_0: u32 = 14256;
        // D s_581_1: read-reg s_581_0:u8
        let s_581_1: u8 = {
            let value = state.read_register::<u8>(s_581_0 as isize);
            tracer.read_register(s_581_0 as isize, value);
            value
        };
        // D s_581_2: cast zx s_581_1 -> bv
        let s_581_2: Bits = Bits::new(s_581_1 as u128, 4u16);
        // C s_581_3: const #1u : u8
        let s_581_3: u8 = 1;
        // C s_581_4: cast zx s_581_3 -> bv
        let s_581_4: Bits = Bits::new(s_581_3 as u128, 4u16);
        // D s_581_5: cmp-eq s_581_2 s_581_4
        let s_581_5: bool = ((s_581_2) == (s_581_4));
        // D s_581_6: write-var gs#331368 <= s_581_5
        fn_state.gs_331368 = s_581_5;
        // N s_581_7: jump b580
        return block_580(state, tracer, fn_state);
    }
    fn block_582<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_582_0: const #103224u : u32
        let s_582_0: u32 = 103224;
        // D s_582_1: read-reg s_582_0:u8
        let s_582_1: bool = {
            let value = state.read_register::<bool>(s_582_0 as isize);
            tracer.read_register(s_582_0 as isize, value);
            value
        };
        // D s_582_2: write-var return_value <= s_582_1
        fn_state.return_value = s_582_1;
        // N s_582_3: jump b325
        return block_325(state, tracer, fn_state);
    }
    fn block_583<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_583_0: const #21232u : u32
        let s_583_0: u32 = 21232;
        // D s_583_1: read-reg s_583_0:u8
        let s_583_1: bool = {
            let value = state.read_register::<bool>(s_583_0 as isize);
            tracer.read_register(s_583_0 as isize, value);
            value
        };
        // D s_583_2: write-var return_value <= s_583_1
        fn_state.return_value = s_583_1;
        // N s_583_3: jump b325
        return block_325(state, tracer, fn_state);
    }
    fn block_584<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_584_0: const #1u : u8
        let s_584_0: bool = true;
        // D s_584_1: write-var return_value <= s_584_0
        fn_state.return_value = s_584_0;
        // N s_584_2: jump b325
        return block_325(state, tracer, fn_state);
    }
    fn block_585<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_585_0: const #22568u : u32
        let s_585_0: u32 = 22568;
        // D s_585_1: read-reg s_585_0:u8
        let s_585_1: bool = {
            let value = state.read_register::<bool>(s_585_0 as isize);
            tracer.read_register(s_585_0 as isize, value);
            value
        };
        // D s_585_2: write-var return_value <= s_585_1
        fn_state.return_value = s_585_1;
        // N s_585_3: jump b325
        return block_325(state, tracer, fn_state);
    }
    fn block_586<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_586_0: const #100896u : u32
        let s_586_0: u32 = 100896;
        // D s_586_1: read-reg s_586_0:u8
        let s_586_1: bool = {
            let value = state.read_register::<bool>(s_586_0 as isize);
            tracer.read_register(s_586_0 as isize, value);
            value
        };
        // D s_586_2: write-var return_value <= s_586_1
        fn_state.return_value = s_586_1;
        // N s_586_3: jump b325
        return block_325(state, tracer, fn_state);
    }
    fn block_587<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_587_0: const #0u : u8
        let s_587_0: bool = false;
        // D s_587_1: write-var return_value <= s_587_0
        fn_state.return_value = s_587_0;
        // N s_587_2: jump b325
        return block_325(state, tracer, fn_state);
    }
    fn block_588<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_588_0: const #1u : u8
        let s_588_0: bool = true;
        // D s_588_1: write-var return_value <= s_588_0
        fn_state.return_value = s_588_0;
        // N s_588_2: jump b325
        return block_325(state, tracer, fn_state);
    }
    fn block_589<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_589_0: const #90760u : u32
        let s_589_0: u32 = 90760;
        // D s_589_1: read-reg s_589_0:u8
        let s_589_1: bool = {
            let value = state.read_register::<bool>(s_589_0 as isize);
            tracer.read_register(s_589_0 as isize, value);
            value
        };
        // D s_589_2: write-var return_value <= s_589_1
        fn_state.return_value = s_589_1;
        // N s_589_3: jump b325
        return block_325(state, tracer, fn_state);
    }
    fn block_590<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_590_0: const #103192u : u32
        let s_590_0: u32 = 103192;
        // D s_590_1: read-reg s_590_0:u8
        let s_590_1: bool = {
            let value = state.read_register::<bool>(s_590_0 as isize);
            tracer.read_register(s_590_0 as isize, value);
            value
        };
        // D s_590_2: write-var return_value <= s_590_1
        fn_state.return_value = s_590_1;
        // N s_590_3: jump b325
        return block_325(state, tracer, fn_state);
    }
    fn block_591<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_591_0: const #14344u : u32
        let s_591_0: u32 = 14344;
        // D s_591_1: read-reg s_591_0:u8
        let s_591_1: bool = {
            let value = state.read_register::<bool>(s_591_0 as isize);
            tracer.read_register(s_591_0 as isize, value);
            value
        };
        // D s_591_2: write-var return_value <= s_591_1
        fn_state.return_value = s_591_1;
        // N s_591_3: jump b325
        return block_325(state, tracer, fn_state);
    }
    fn block_592<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_592_0: const #89528u : u32
        let s_592_0: u32 = 89528;
        // D s_592_1: read-reg s_592_0:u8
        let s_592_1: bool = {
            let value = state.read_register::<bool>(s_592_0 as isize);
            tracer.read_register(s_592_0 as isize, value);
            value
        };
        // D s_592_2: write-var return_value <= s_592_1
        fn_state.return_value = s_592_1;
        // N s_592_3: jump b325
        return block_325(state, tracer, fn_state);
    }
    fn block_593<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_593_0: const #23640u : u32
        let s_593_0: u32 = 23640;
        // D s_593_1: read-reg s_593_0:u8
        let s_593_1: bool = {
            let value = state.read_register::<bool>(s_593_0 as isize);
            tracer.read_register(s_593_0 as isize, value);
            value
        };
        // D s_593_2: write-var return_value <= s_593_1
        fn_state.return_value = s_593_1;
        // N s_593_3: jump b325
        return block_325(state, tracer, fn_state);
    }
    fn block_594<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_594_0: const #102336u : u32
        let s_594_0: u32 = 102336;
        // D s_594_1: read-reg s_594_0:u8
        let s_594_1: bool = {
            let value = state.read_register::<bool>(s_594_0 as isize);
            tracer.read_register(s_594_0 as isize, value);
            value
        };
        // D s_594_2: write-var return_value <= s_594_1
        fn_state.return_value = s_594_1;
        // N s_594_3: jump b325
        return block_325(state, tracer, fn_state);
    }
    fn block_595<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_595_0: const #102432u : u32
        let s_595_0: u32 = 102432;
        // D s_595_1: read-reg s_595_0:u8
        let s_595_1: bool = {
            let value = state.read_register::<bool>(s_595_0 as isize);
            tracer.read_register(s_595_0 as isize, value);
            value
        };
        // D s_595_2: write-var return_value <= s_595_1
        fn_state.return_value = s_595_1;
        // N s_595_3: jump b325
        return block_325(state, tracer, fn_state);
    }
    fn block_596<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_596_0: const #101184u : u32
        let s_596_0: u32 = 101184;
        // D s_596_1: read-reg s_596_0:u8
        let s_596_1: bool = {
            let value = state.read_register::<bool>(s_596_0 as isize);
            tracer.read_register(s_596_0 as isize, value);
            value
        };
        // D s_596_2: write-var return_value <= s_596_1
        fn_state.return_value = s_596_1;
        // N s_596_3: jump b325
        return block_325(state, tracer, fn_state);
    }
    fn block_597<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_597_0: const #14616u : u32
        let s_597_0: u32 = 14616;
        // D s_597_1: read-reg s_597_0:u8
        let s_597_1: bool = {
            let value = state.read_register::<bool>(s_597_0 as isize);
            tracer.read_register(s_597_0 as isize, value);
            value
        };
        // D s_597_2: write-var return_value <= s_597_1
        fn_state.return_value = s_597_1;
        // N s_597_3: jump b325
        return block_325(state, tracer, fn_state);
    }
    fn block_598<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_598_0: const #1u : u8
        let s_598_0: bool = true;
        // D s_598_1: write-var return_value <= s_598_0
        fn_state.return_value = s_598_0;
        // N s_598_2: jump b325
        return block_325(state, tracer, fn_state);
    }
    fn block_599<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_599_0: const #1u : u8
        let s_599_0: bool = true;
        // D s_599_1: write-var return_value <= s_599_0
        fn_state.return_value = s_599_0;
        // N s_599_2: jump b325
        return block_325(state, tracer, fn_state);
    }
    fn block_600<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_600_0: const #1u : u8
        let s_600_0: bool = true;
        // D s_600_1: write-var return_value <= s_600_0
        fn_state.return_value = s_600_0;
        // N s_600_2: jump b325
        return block_325(state, tracer, fn_state);
    }
    fn block_601<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_601_0: const #1u : u8
        let s_601_0: bool = true;
        // D s_601_1: write-var return_value <= s_601_0
        fn_state.return_value = s_601_0;
        // N s_601_2: jump b325
        return block_325(state, tracer, fn_state);
    }
    fn block_602<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_602_0: const #1u : u8
        let s_602_0: bool = true;
        // D s_602_1: write-var return_value <= s_602_0
        fn_state.return_value = s_602_0;
        // N s_602_2: jump b325
        return block_325(state, tracer, fn_state);
    }
    fn block_603<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_603_0: const #1u : u8
        let s_603_0: bool = true;
        // D s_603_1: write-var return_value <= s_603_0
        fn_state.return_value = s_603_0;
        // N s_603_2: jump b325
        return block_325(state, tracer, fn_state);
    }
    fn block_604<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_604_0: const #1568u : u32
        let s_604_0: u32 = 1568;
        // D s_604_1: read-reg s_604_0:u8
        let s_604_1: bool = {
            let value = state.read_register::<bool>(s_604_0 as isize);
            tracer.read_register(s_604_0 as isize, value);
            value
        };
        // N s_604_2: branch s_604_1 b607 b605
        if s_604_1 {
            return block_607(state, tracer, fn_state);
        } else {
            return block_605(state, tracer, fn_state);
        };
    }
    fn block_605<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_605_0: const #0u : u8
        let s_605_0: bool = false;
        // D s_605_1: write-var gs#331369 <= s_605_0
        fn_state.gs_331369 = s_605_0;
        // N s_605_2: jump b606
        return block_606(state, tracer, fn_state);
    }
    fn block_606<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_606_0: read-var gs#331369:u8
        let s_606_0: bool = fn_state.gs_331369;
        // D s_606_1: write-var return_value <= s_606_0
        fn_state.return_value = s_606_0;
        // N s_606_2: jump b325
        return block_325(state, tracer, fn_state);
    }
    fn block_607<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_607_0: const #100264u : u32
        let s_607_0: u32 = 100264;
        // D s_607_1: read-reg s_607_0:u8
        let s_607_1: bool = {
            let value = state.read_register::<bool>(s_607_0 as isize);
            tracer.read_register(s_607_0 as isize, value);
            value
        };
        // D s_607_2: write-var gs#331369 <= s_607_1
        fn_state.gs_331369 = s_607_1;
        // N s_607_3: jump b606
        return block_606(state, tracer, fn_state);
    }
    fn block_608<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_608_0: const #1632u : u32
        let s_608_0: u32 = 1632;
        // D s_608_1: read-reg s_608_0:u8
        let s_608_1: bool = {
            let value = state.read_register::<bool>(s_608_0 as isize);
            tracer.read_register(s_608_0 as isize, value);
            value
        };
        // D s_608_2: write-var return_value <= s_608_1
        fn_state.return_value = s_608_1;
        // N s_608_3: jump b325
        return block_325(state, tracer, fn_state);
    }
    fn block_609<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_609_0: const #16328u : u32
        let s_609_0: u32 = 16328;
        // D s_609_1: read-reg s_609_0:u8
        let s_609_1: bool = {
            let value = state.read_register::<bool>(s_609_0 as isize);
            tracer.read_register(s_609_0 as isize, value);
            value
        };
        // D s_609_2: write-var return_value <= s_609_1
        fn_state.return_value = s_609_1;
        // N s_609_3: jump b325
        return block_325(state, tracer, fn_state);
    }
    fn block_610<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_610_0: const #0u : u8
        let s_610_0: bool = false;
        // D s_610_1: write-var return_value <= s_610_0
        fn_state.return_value = s_610_0;
        // N s_610_2: jump b325
        return block_325(state, tracer, fn_state);
    }
    fn block_611<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_611_0: const #1u : u8
        let s_611_0: bool = true;
        // D s_611_1: write-var return_value <= s_611_0
        fn_state.return_value = s_611_0;
        // N s_611_2: jump b325
        return block_325(state, tracer, fn_state);
    }
    fn block_612<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_612_0: const #1u : u8
        let s_612_0: bool = true;
        // D s_612_1: write-var return_value <= s_612_0
        fn_state.return_value = s_612_0;
        // N s_612_2: jump b325
        return block_325(state, tracer, fn_state);
    }
    fn block_613<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_613_0: const #0u : u8
        let s_613_0: bool = false;
        // D s_613_1: write-var return_value <= s_613_0
        fn_state.return_value = s_613_0;
        // N s_613_2: jump b325
        return block_325(state, tracer, fn_state);
    }
    fn block_614<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_614_0: const #1u : u8
        let s_614_0: bool = true;
        // D s_614_1: write-var return_value <= s_614_0
        fn_state.return_value = s_614_0;
        // N s_614_2: jump b325
        return block_325(state, tracer, fn_state);
    }
    fn block_615<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_615_0: const #1u : u8
        let s_615_0: bool = true;
        // D s_615_1: write-var return_value <= s_615_0
        fn_state.return_value = s_615_0;
        // N s_615_2: jump b325
        return block_325(state, tracer, fn_state);
    }
    fn block_616<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_616_0: const #0u : u8
        let s_616_0: bool = false;
        // D s_616_1: write-var return_value <= s_616_0
        fn_state.return_value = s_616_0;
        // N s_616_2: jump b325
        return block_325(state, tracer, fn_state);
    }
    fn block_617<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_617_0: const #0u : u8
        let s_617_0: bool = false;
        // D s_617_1: write-var return_value <= s_617_0
        fn_state.return_value = s_617_0;
        // N s_617_2: jump b325
        return block_325(state, tracer, fn_state);
    }
    fn block_618<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_618_0: const #0u : u8
        let s_618_0: bool = false;
        // D s_618_1: write-var return_value <= s_618_0
        fn_state.return_value = s_618_0;
        // N s_618_2: jump b325
        return block_325(state, tracer, fn_state);
    }
    fn block_619<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_619_0: const #1u : u8
        let s_619_0: bool = true;
        // D s_619_1: write-var return_value <= s_619_0
        fn_state.return_value = s_619_0;
        // N s_619_2: jump b325
        return block_325(state, tracer, fn_state);
    }
    fn block_620<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_620_0: const #1u : u8
        let s_620_0: bool = true;
        // D s_620_1: write-var return_value <= s_620_0
        fn_state.return_value = s_620_0;
        // N s_620_2: jump b325
        return block_325(state, tracer, fn_state);
    }
    fn block_621<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_621_0: const #1u : u8
        let s_621_0: bool = true;
        // D s_621_1: write-var return_value <= s_621_0
        fn_state.return_value = s_621_0;
        // N s_621_2: jump b325
        return block_325(state, tracer, fn_state);
    }
    fn block_622<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_622_0: const #1u : u8
        let s_622_0: bool = true;
        // D s_622_1: write-var return_value <= s_622_0
        fn_state.return_value = s_622_0;
        // N s_622_2: jump b325
        return block_325(state, tracer, fn_state);
    }
    fn block_623<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_623_0: const #1u : u8
        let s_623_0: bool = true;
        // D s_623_1: write-var return_value <= s_623_0
        fn_state.return_value = s_623_0;
        // N s_623_2: jump b325
        return block_325(state, tracer, fn_state);
    }
    fn block_624<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_624_0: const #1u : u8
        let s_624_0: bool = true;
        // D s_624_1: write-var return_value <= s_624_0
        fn_state.return_value = s_624_0;
        // N s_624_2: jump b325
        return block_325(state, tracer, fn_state);
    }
    fn block_625<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_625_0: const #1u : u8
        let s_625_0: bool = true;
        // D s_625_1: write-var return_value <= s_625_0
        fn_state.return_value = s_625_0;
        // N s_625_2: jump b325
        return block_325(state, tracer, fn_state);
    }
    fn block_626<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_626_0: const #1u : u8
        let s_626_0: bool = true;
        // D s_626_1: write-var return_value <= s_626_0
        fn_state.return_value = s_626_0;
        // N s_626_2: jump b325
        return block_325(state, tracer, fn_state);
    }
    fn block_627<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_627_0: const #1u : u8
        let s_627_0: bool = true;
        // D s_627_1: write-var return_value <= s_627_0
        fn_state.return_value = s_627_0;
        // N s_627_2: jump b325
        return block_325(state, tracer, fn_state);
    }
    fn block_628<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_628_0: const #1u : u8
        let s_628_0: bool = true;
        // D s_628_1: write-var return_value <= s_628_0
        fn_state.return_value = s_628_0;
        // N s_628_2: jump b325
        return block_325(state, tracer, fn_state);
    }
    fn block_629<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_629_0: const #0u : u8
        let s_629_0: bool = false;
        // D s_629_1: write-var return_value <= s_629_0
        fn_state.return_value = s_629_0;
        // N s_629_2: jump b325
        return block_325(state, tracer, fn_state);
    }
    fn block_630<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_630_0: const #1u : u8
        let s_630_0: bool = true;
        // D s_630_1: write-var return_value <= s_630_0
        fn_state.return_value = s_630_0;
        // N s_630_2: jump b325
        return block_325(state, tracer, fn_state);
    }
    fn block_631<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_631_0: const #1u : u8
        let s_631_0: bool = true;
        // D s_631_1: write-var return_value <= s_631_0
        fn_state.return_value = s_631_0;
        // N s_631_2: jump b325
        return block_325(state, tracer, fn_state);
    }
    fn block_632<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_632_0: const #1u : u8
        let s_632_0: bool = true;
        // D s_632_1: write-var return_value <= s_632_0
        fn_state.return_value = s_632_0;
        // N s_632_2: jump b325
        return block_325(state, tracer, fn_state);
    }
    fn block_633<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_633_0: const #0u : u8
        let s_633_0: bool = false;
        // D s_633_1: write-var return_value <= s_633_0
        fn_state.return_value = s_633_0;
        // N s_633_2: jump b325
        return block_325(state, tracer, fn_state);
    }
    fn block_634<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_634_0: const #0u : u8
        let s_634_0: bool = false;
        // D s_634_1: write-var return_value <= s_634_0
        fn_state.return_value = s_634_0;
        // N s_634_2: jump b325
        return block_325(state, tracer, fn_state);
    }
    fn block_635<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_635_0: const #1u : u8
        let s_635_0: bool = true;
        // D s_635_1: write-var return_value <= s_635_0
        fn_state.return_value = s_635_0;
        // N s_635_2: jump b325
        return block_325(state, tracer, fn_state);
    }
    fn block_636<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_636_0: const #1408u : u32
        let s_636_0: u32 = 1408;
        // D s_636_1: read-reg s_636_0:u8
        let s_636_1: bool = {
            let value = state.read_register::<bool>(s_636_0 as isize);
            tracer.read_register(s_636_0 as isize, value);
            value
        };
        // D s_636_2: write-var return_value <= s_636_1
        fn_state.return_value = s_636_1;
        // N s_636_3: jump b325
        return block_325(state, tracer, fn_state);
    }
    fn block_637<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_637_0: const #1u : u8
        let s_637_0: bool = true;
        // D s_637_1: write-var return_value <= s_637_0
        fn_state.return_value = s_637_0;
        // N s_637_2: jump b325
        return block_325(state, tracer, fn_state);
    }
    fn block_638<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_638_0: const #1u : u8
        let s_638_0: bool = true;
        // D s_638_1: write-var return_value <= s_638_0
        fn_state.return_value = s_638_0;
        // N s_638_2: jump b325
        return block_325(state, tracer, fn_state);
    }
    fn block_639<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_639_0: const #1u : u8
        let s_639_0: bool = true;
        // D s_639_1: write-var return_value <= s_639_0
        fn_state.return_value = s_639_0;
        // N s_639_2: jump b325
        return block_325(state, tracer, fn_state);
    }
    fn block_640<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_640_0: const #0u : u8
        let s_640_0: bool = false;
        // D s_640_1: write-var return_value <= s_640_0
        fn_state.return_value = s_640_0;
        // N s_640_2: jump b325
        return block_325(state, tracer, fn_state);
    }
    fn block_641<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_641_0: const #1u : u8
        let s_641_0: bool = true;
        // D s_641_1: write-var return_value <= s_641_0
        fn_state.return_value = s_641_0;
        // N s_641_2: jump b325
        return block_325(state, tracer, fn_state);
    }
    fn block_642<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_642_0: const #1u : u8
        let s_642_0: bool = true;
        // D s_642_1: write-var return_value <= s_642_0
        fn_state.return_value = s_642_0;
        // N s_642_2: jump b325
        return block_325(state, tracer, fn_state);
    }
    fn block_643<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_643_0: const #0u : u8
        let s_643_0: bool = false;
        // D s_643_1: write-var return_value <= s_643_0
        fn_state.return_value = s_643_0;
        // N s_643_2: jump b325
        return block_325(state, tracer, fn_state);
    }
    fn block_644<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_644_0: const #1u : u8
        let s_644_0: bool = true;
        // D s_644_1: write-var return_value <= s_644_0
        fn_state.return_value = s_644_0;
        // N s_644_2: jump b325
        return block_325(state, tracer, fn_state);
    }
    fn block_645<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_645_0: const #1u : u8
        let s_645_0: bool = true;
        // D s_645_1: write-var return_value <= s_645_0
        fn_state.return_value = s_645_0;
        // N s_645_2: jump b325
        return block_325(state, tracer, fn_state);
    }
    fn block_646<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_646_0: const #10368u : u32
        let s_646_0: u32 = 10368;
        // D s_646_1: read-reg s_646_0:u8
        let s_646_1: bool = {
            let value = state.read_register::<bool>(s_646_0 as isize);
            tracer.read_register(s_646_0 as isize, value);
            value
        };
        // D s_646_2: not s_646_1
        let s_646_2: bool = !s_646_1;
        // D s_646_3: write-var return_value <= s_646_2
        fn_state.return_value = s_646_2;
        // N s_646_4: jump b325
        return block_325(state, tracer, fn_state);
    }
    fn block_647<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_647_0: const #16975u : u32
        let s_647_0: u32 = 16975;
        // D s_647_1: read-reg s_647_0:u8
        let s_647_1: u8 = {
            let value = state.read_register::<u8>(s_647_0 as isize);
            tracer.read_register(s_647_0 as isize, value);
            value
        };
        // D s_647_2: cast zx s_647_1 -> bv
        let s_647_2: Bits = Bits::new(s_647_1 as u128, 2u16);
        // C s_647_3: const #448u : u32
        let s_647_3: u32 = 448;
        // D s_647_4: read-reg s_647_3:u8
        let s_647_4: u8 = {
            let value = state.read_register::<u8>(s_647_3 as isize);
            tracer.read_register(s_647_3 as isize, value);
            value
        };
        // D s_647_5: cast zx s_647_4 -> bv
        let s_647_5: Bits = Bits::new(s_647_4 as u128, 2u16);
        // D s_647_6: cmp-eq s_647_2 s_647_5
        let s_647_6: bool = ((s_647_2) == (s_647_5));
        // D s_647_7: write-var return_value <= s_647_6
        fn_state.return_value = s_647_6;
        // N s_647_8: jump b325
        return block_325(state, tracer, fn_state);
    }
    fn block_648<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_648_0: const #1u : u8
        let s_648_0: bool = true;
        // D s_648_1: write-var return_value <= s_648_0
        fn_state.return_value = s_648_0;
        // N s_648_2: jump b325
        return block_325(state, tracer, fn_state);
    }
    fn block_649<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_649_0: const #0u : u8
        let s_649_0: bool = false;
        // D s_649_1: write-var return_value <= s_649_0
        fn_state.return_value = s_649_0;
        // N s_649_2: jump b325
        return block_325(state, tracer, fn_state);
    }
    fn block_650<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_650_0: const #0u : u8
        let s_650_0: bool = false;
        // D s_650_1: write-var return_value <= s_650_0
        fn_state.return_value = s_650_0;
        // N s_650_2: jump b325
        return block_325(state, tracer, fn_state);
    }
    fn block_651<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_651_0: const #0u : u8
        let s_651_0: bool = false;
        // D s_651_1: write-var return_value <= s_651_0
        fn_state.return_value = s_651_0;
        // N s_651_2: jump b325
        return block_325(state, tracer, fn_state);
    }
    fn block_652<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_652_0: const #21040u : u32
        let s_652_0: u32 = 21040;
        // D s_652_1: read-reg s_652_0:u8
        let s_652_1: bool = {
            let value = state.read_register::<bool>(s_652_0 as isize);
            tracer.read_register(s_652_0 as isize, value);
            value
        };
        // D s_652_2: write-var return_value <= s_652_1
        fn_state.return_value = s_652_1;
        // N s_652_3: jump b325
        return block_325(state, tracer, fn_state);
    }
    fn block_653<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_653_0: const #0u : u8
        let s_653_0: bool = false;
        // D s_653_1: write-var return_value <= s_653_0
        fn_state.return_value = s_653_0;
        // N s_653_2: jump b325
        return block_325(state, tracer, fn_state);
    }
    fn block_654<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_654_0: const #0u : u8
        let s_654_0: bool = false;
        // D s_654_1: write-var return_value <= s_654_0
        fn_state.return_value = s_654_0;
        // N s_654_2: jump b325
        return block_325(state, tracer, fn_state);
    }
    fn block_655<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_655_0: const #0u : u8
        let s_655_0: bool = false;
        // D s_655_1: write-var return_value <= s_655_0
        fn_state.return_value = s_655_0;
        // N s_655_2: jump b325
        return block_325(state, tracer, fn_state);
    }
    fn block_656<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_656_0: const #1u : u8
        let s_656_0: bool = true;
        // D s_656_1: write-var return_value <= s_656_0
        fn_state.return_value = s_656_0;
        // N s_656_2: jump b325
        return block_325(state, tracer, fn_state);
    }
    fn block_657<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_657_0: const #1u : u8
        let s_657_0: bool = true;
        // D s_657_1: write-var return_value <= s_657_0
        fn_state.return_value = s_657_0;
        // N s_657_2: jump b325
        return block_325(state, tracer, fn_state);
    }
    fn block_658<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_658_0: const #() : ()
        let s_658_0: () = ();
        // S s_658_1: call HighestEL(s_658_0)
        let s_658_1: u8 = HighestEL(state, tracer, s_658_0);
        // C s_658_2: const #424u : u32
        let s_658_2: u32 = 424;
        // D s_658_3: read-reg s_658_2:u8
        let s_658_3: u8 = {
            let value = state.read_register::<u8>(s_658_2 as isize);
            tracer.read_register(s_658_2 as isize, value);
            value
        };
        // D s_658_4: cast zx s_658_3 -> bv
        let s_658_4: Bits = Bits::new(s_658_3 as u128, 2u16);
        // S s_658_5: cast zx s_658_1 -> bv
        let s_658_5: Bits = Bits::new(s_658_1 as u128, 2u16);
        // D s_658_6: cmp-eq s_658_4 s_658_5
        let s_658_6: bool = ((s_658_4) == (s_658_5));
        // D s_658_7: write-var return_value <= s_658_6
        fn_state.return_value = s_658_6;
        // N s_658_8: jump b325
        return block_325(state, tracer, fn_state);
    }
    fn block_659<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_659_0: const #() : ()
        let s_659_0: () = ();
        // S s_659_1: call HighestEL(s_659_0)
        let s_659_1: u8 = HighestEL(state, tracer, s_659_0);
        // C s_659_2: const #432u : u32
        let s_659_2: u32 = 432;
        // D s_659_3: read-reg s_659_2:u8
        let s_659_3: u8 = {
            let value = state.read_register::<u8>(s_659_2 as isize);
            tracer.read_register(s_659_2 as isize, value);
            value
        };
        // D s_659_4: cast zx s_659_3 -> bv
        let s_659_4: Bits = Bits::new(s_659_3 as u128, 2u16);
        // S s_659_5: cast zx s_659_1 -> bv
        let s_659_5: Bits = Bits::new(s_659_1 as u128, 2u16);
        // D s_659_6: cmp-eq s_659_4 s_659_5
        let s_659_6: bool = ((s_659_4) == (s_659_5));
        // D s_659_7: write-var return_value <= s_659_6
        fn_state.return_value = s_659_6;
        // N s_659_8: jump b325
        return block_325(state, tracer, fn_state);
    }
    fn block_660<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_660_0: const #() : ()
        let s_660_0: () = ();
        // S s_660_1: call HighestEL(s_660_0)
        let s_660_1: u8 = HighestEL(state, tracer, s_660_0);
        // C s_660_2: const #440u : u32
        let s_660_2: u32 = 440;
        // D s_660_3: read-reg s_660_2:u8
        let s_660_3: u8 = {
            let value = state.read_register::<u8>(s_660_2 as isize);
            tracer.read_register(s_660_2 as isize, value);
            value
        };
        // D s_660_4: cast zx s_660_3 -> bv
        let s_660_4: Bits = Bits::new(s_660_3 as u128, 2u16);
        // S s_660_5: cast zx s_660_1 -> bv
        let s_660_5: Bits = Bits::new(s_660_1 as u128, 2u16);
        // D s_660_6: cmp-eq s_660_4 s_660_5
        let s_660_6: bool = ((s_660_4) == (s_660_5));
        // D s_660_7: write-var return_value <= s_660_6
        fn_state.return_value = s_660_6;
        // N s_660_8: jump b325
        return block_325(state, tracer, fn_state);
    }
    fn block_661<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_661_0: const #() : ()
        let s_661_0: () = ();
        // S s_661_1: call HighestEL(s_661_0)
        let s_661_1: u8 = HighestEL(state, tracer, s_661_0);
        // C s_661_2: const #432u : u32
        let s_661_2: u32 = 432;
        // D s_661_3: read-reg s_661_2:u8
        let s_661_3: u8 = {
            let value = state.read_register::<u8>(s_661_2 as isize);
            tracer.read_register(s_661_2 as isize, value);
            value
        };
        // D s_661_4: cast zx s_661_3 -> bv
        let s_661_4: Bits = Bits::new(s_661_3 as u128, 2u16);
        // S s_661_5: cast zx s_661_1 -> bv
        let s_661_5: Bits = Bits::new(s_661_1 as u128, 2u16);
        // D s_661_6: cmp-eq s_661_4 s_661_5
        let s_661_6: bool = ((s_661_4) == (s_661_5));
        // N s_661_7: branch s_661_6 b664 b662
        if s_661_6 {
            return block_664(state, tracer, fn_state);
        } else {
            return block_662(state, tracer, fn_state);
        };
    }
    fn block_662<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_662_0: const #() : ()
        let s_662_0: () = ();
        // S s_662_1: call HighestEL(s_662_0)
        let s_662_1: u8 = HighestEL(state, tracer, s_662_0);
        // C s_662_2: const #424u : u32
        let s_662_2: u32 = 424;
        // D s_662_3: read-reg s_662_2:u8
        let s_662_3: u8 = {
            let value = state.read_register::<u8>(s_662_2 as isize);
            tracer.read_register(s_662_2 as isize, value);
            value
        };
        // D s_662_4: cast zx s_662_3 -> bv
        let s_662_4: Bits = Bits::new(s_662_3 as u128, 2u16);
        // S s_662_5: cast zx s_662_1 -> bv
        let s_662_5: Bits = Bits::new(s_662_1 as u128, 2u16);
        // D s_662_6: cmp-eq s_662_4 s_662_5
        let s_662_6: bool = ((s_662_4) == (s_662_5));
        // D s_662_7: write-var gs#331370 <= s_662_6
        fn_state.gs_331370 = s_662_6;
        // N s_662_8: jump b663
        return block_663(state, tracer, fn_state);
    }
    fn block_663<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_663_0: read-var gs#331370:u8
        let s_663_0: bool = fn_state.gs_331370;
        // D s_663_1: write-var return_value <= s_663_0
        fn_state.return_value = s_663_0;
        // N s_663_2: jump b325
        return block_325(state, tracer, fn_state);
    }
    fn block_664<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_664_0: const #1u : u8
        let s_664_0: bool = true;
        // D s_664_1: write-var gs#331370 <= s_664_0
        fn_state.gs_331370 = s_664_0;
        // N s_664_2: jump b663
        return block_663(state, tracer, fn_state);
    }
}
